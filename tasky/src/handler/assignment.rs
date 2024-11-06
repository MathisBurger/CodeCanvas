use std::{collections::HashMap, io::Read};

use super::file_structure::{self, *};
use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
use mongodb::Database;
use serde::Deserialize;

use crate::{
    error::ApiError,
    models::{
        assignment::{Assignment, AssignmentRepository},
        DB,
    },
    mongo::test_file::{TestFile, TestFileCollection},
    response::assignment::{AssignmentFile, AssignmentFileStructure},
};

/// Runner data for the executor runner
/// sent by Multipart request as JSON
#[derive(Deserialize)]
pub struct RunnerData {
    pub runner_cpu: String,
    pub runner_memory: String,
    pub runner_timeout: String,
    pub runner_cmd: String,
}

/// Multipart form to create code tests
#[derive(MultipartForm)]
pub struct CreateCodeTestMultipart {
    pub file_structure: Json<AssignmentFileStructure>,
    #[multipart(limit = "10MB")]
    pub files: Vec<TempFile>,
    pub runner_config: Json<RunnerData>,
}

/// Handles to create code tests
/// This means storing data in postgres, files in mongo
/// and validating the input
pub async fn handle_create_multipart(
    form: CreateCodeTestMultipart,
    mongodb: &Database,
    db: &mut DB,
    mut assignment: Assignment,
) -> Result<Assignment, ApiError> {
    // TODO: Validate runner config
    let mut file_structure = form.file_structure.0;
    if !file_structure_contains_files(&file_structure) {
        return Err(ApiError::BadRequest {
            message: "File structure does not contain any file".to_string(),
        });
    }

    let mut filename_map = build_filename_map(&form.files)?;
    let mut actual_files: Vec<&mut AssignmentFile> = vec![];

    validate_test_file_structure(
        &mut file_structure,
        &mut filename_map,
        &mut actual_files,
        true,
    )?;

    create_files_and_update_ids(&mut actual_files, mongodb, filename_map, &assignment).await;

    let file_structure_value =
        serde_json::to_value(file_structure).map_err(|_| ApiError::InternalServerError {
            message: "Cannot convert file structure to JSON".to_string(),
        })?;

    assignment.file_structure = Some(file_structure_value);
    assignment.runner_cpu = form.runner_config.runner_cpu.clone();
    assignment.runner_memory = form.runner_config.runner_memory.clone();
    assignment.runner_timeout = form.runner_config.runner_timeout.clone();
    assignment.runner_cmd = form.runner_config.runner_cmd.clone();
    AssignmentRepository::update_assignment(assignment.clone(), db);

    Ok(assignment)
}

pub async fn handle_update_multipart(
    form: CreateCodeTestMultipart,
    mongodb: &Database,
    db: &mut DB,
    mut assignment: Assignment,
) -> Result<Assignment, ApiError> {
    let mut new_file_structure = form.file_structure.0;
    if !file_structure_contains_files(&new_file_structure) {
        return Err(ApiError::BadRequest {
            message: "File structure does not contain any file".to_string(),
        });
    }

    if let Some(ref file_structure_value) = assignment.file_structure {
        let file_structure: AssignmentFileStructure =
            serde_json::from_value(file_structure_value.clone()).unwrap();

        let mut filename_map = build_filename_map(&form.files)?;
        let mut new_files: Vec<&mut AssignmentFile> = vec![];

        compare_structures(&file_structure, &new_file_structure)?;

        // Validates file structures and gets new files to persist
        validate_test_file_structure(
            &mut new_file_structure,
            &mut filename_map,
            &mut new_files,
            true,
        )?;

        create_files_and_update_ids(&mut new_files, mongodb, filename_map, &assignment.clone())
            .await;

        let file_structure_value = serde_json::to_value(new_file_structure).map_err(|_| {
            ApiError::InternalServerError {
                message: "Cannot convert file structure to JSON".to_string(),
            }
        })?;

        assignment.file_structure = Some(file_structure_value);
        assignment.runner_cpu = form.runner_config.runner_cpu.clone();
        assignment.runner_memory = form.runner_config.runner_memory.clone();
        assignment.runner_timeout = form.runner_config.runner_timeout.clone();
        assignment.runner_cmd = form.runner_config.runner_cmd.clone();
        AssignmentRepository::update_assignment(assignment.clone(), db);

        return Ok(assignment);
    }
    return Err(ApiError::BadRequest {
        message: "The assignment does not have a file structure".to_string(),
    });
}

/// Creates files and updates the correlated object_ids
/// Also updates files in file_structure, because the actual_files are referenced from the file structure
async fn create_files_and_update_ids(
    actual_files: &mut Vec<&mut AssignmentFile>,
    mongodb: &Database,
    filename_map: HashMap<String, (bool, &TempFile)>,
    assignment: &Assignment,
) {
    let mut file_data: Vec<(String, String, usize)> = vec![];
    for file in actual_files.into_iter() {
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

    let mongo_files = TestFileCollection::create_many(
        file_data
            .iter()
            .map(|f| TestFile {
                id: None,
                file_name: f.0.clone(),
                assignment_id: assignment.id,
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
}
