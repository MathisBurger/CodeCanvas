use super::file_structure::*;
use crate::models::DB;
use crate::response::assignment::AssignmentFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use diesel::sql_types::Json;
use mongodb::Database;
use std::io::Read;

use crate::{
    auth_middleware::UserData,
    error::ApiError,
    models::{
        assignment::Assignment,
        solution::{NewSolution, Solution, SolutionRepository},
    },
    mongo::task_file::{TaskFile, TaskFileCollection},
    response::assignment::AssignmentFileStructure,
};

#[derive(MultipartForm)]
pub struct CreateSolutionMultipart {
    #[multipart(limit = "10MB")]
    pub files: Vec<TempFile>,
}

pub async fn handle_create_multipart(
    form: CreateSolutionMultipart,
    user_data: &UserData,
    mongodb: &Database,
    db: &mut DB,
    assignment: &Assignment,
) -> Result<Solution, ApiError> {
    if assignment.file_structure.is_none() {
        return Err(ApiError::BadRequest {
            message: "No code tests submitted. Therefore, no solutions can be handed in."
                .to_string(),
        });
    }
    let mut file_structure = serde_json::from_value(assignment.file_structure.clone().unwrap())
        .map_err(|_| ApiError::InternalServerError {
            message: "Cannot parse file structure".to_string(),
        })?;
    let mut filename_map = build_filename_map(&form.files)?;
    let mut actual_files: Vec<&mut AssignmentFile> = vec![];
    validate_test_file_structure(
        &mut file_structure,
        &mut filename_map,
        &mut actual_files,
        false,
    )?;

    let new_solution = NewSolution {
        submitter_id: user_data.user_id,
        assignment_id: assignment.id,
        approved_by_tutor: false,
    };
    let mut solution = SolutionRepository::create_solution(new_solution, db);

    let mut file_data: Vec<(String, String, usize)> = vec![];
    for file in &mut actual_files {
        let mut content = String::new();
        let size = filename_map
            .get(&file.filename)
            .unwrap()
            .1
            .file
            .as_file()
            .read_to_string(&mut content)
            .unwrap();
        file.file_size = Some(size);
        file_data.push((file.filename.clone(), content, size));
    }

    let mongo_files = TaskFileCollection::create_many(
        file_data
            .iter()
            .map(|f| TaskFile {
                id: None,
                file_name: f.0.clone(),
                solution_id: solution.id,
                content: f.1.clone(),
                content_size: f.2,
            })
            .collect(),
        mongodb,
    )
    .await;
    for (i, file) in actual_files.into_iter().enumerate() {
        file.object_id = Some(mongo_files.get(i).unwrap().to_hex());
    }
    let file_structure_value =
        serde_json::to_value(file_structure).map_err(|e| ApiError::InternalServerError {
            message: "Cannot convert file structure to JSON".to_string(),
        })?;
    solution.file_structure = Some(file_structure_value);
    SolutionRepository::update_solution(solution.clone(), db);
    Ok(solution)
}
