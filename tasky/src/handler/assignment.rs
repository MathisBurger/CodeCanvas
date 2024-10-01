use std::{collections::HashMap, io::Read, iter::Map};

use actix_multipart::{
    form::{json::Json, tempfile::TempFile, MultipartForm},
    Multipart,
};
use actix_web::HttpResponse;
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

#[derive(Deserialize)]
pub(self) struct RunnerData {
    pub runner_cpu: String,
    pub runner_memory: String,
    pub runner_timeout: String,
}

#[derive(MultipartForm)]
pub struct CreateCodeTestMultipart {
    pub file_structure: Json<AssignmentFileStructure>,
    #[multipart(limit = "10MB")]
    pub files: Vec<TempFile>,
    pub runner_config: Json<RunnerData>,
}

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
    validate_test_file_structure(&mut file_structure, &mut filename_map, &mut actual_files)?;

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
    let file_structure_value =
        serde_json::to_value(file_structure).map_err(|e| ApiError::InternalServerError {
            message: "Cannot convert file structure to JSON".to_string(),
        })?;
    assignment.file_structure = Some(file_structure_value);
    assignment.runner_cpu = form.runner_config.runner_cpu.clone();
    assignment.runner_memory = form.runner_config.runner_memory.clone();
    assignment.runner_timeout = form.runner_config.runner_timeout.clone();
    AssignmentRepository::update_assignment(assignment.clone(), db);
    Ok(assignment)
}

/// Validates whether the given file structure is valid or not
/// It is being checked if all files in the structure are contained by the test files.
fn validate_test_file_structure<'a>(
    structure: &'a mut AssignmentFileStructure,
    files: &mut HashMap<String, (bool, &TempFile)>,
    actual_files: &mut Vec<&'a mut AssignmentFile>,
) -> Result<(), ApiError> {
    // Search all folders recursively
    if structure.folders.is_some() {
        let folders = structure.folders.as_mut().unwrap();
        for folder in folders {
            validate_test_file_structure(folder, files, actual_files)?;
        }
    }

    if structure.files.is_some() {
        let folder_files = structure.files.as_mut().unwrap();
        for file in folder_files {
            // Only search for test files in this case
            if file.is_test_file {
                let file_option = files.get(&file.filename);
                if file_option.is_none() {
                    return Err(ApiError::BadRequest {
                        message: format!(
                            "File {} from file structure has not been uploaded",
                            file.filename
                        ),
                    });
                }
                let file_unwrapped = file_option.unwrap();
                if file_unwrapped.0 {
                    return Err(ApiError::BadRequest { message: format!("File {} exists twice in file structure. Even if the files are in different folders, they need to be named differently", file.filename) });
                }
                files.insert(file.filename.clone(), (true, file_unwrapped.1));
                actual_files.push(file);
            }
        }
    }

    Ok(())
}

/// Checks if a file structure contains files
fn file_structure_contains_files(structure: &AssignmentFileStructure) -> bool {
    if structure.files.clone().unwrap_or_default().len() > 0 {
        return true;
    }
    let folders = structure.folders.clone().unwrap_or_default();
    if folders.len() > 0 {
        for folder in folders {
            if file_structure_contains_files(&folder) {
                return true;
            }
        }
    }
    return false;
}

/// Builds a map with the filename as key and a tuple of bool and the file reference as value
/// The boolean value indicates whether this specific filename has already appeared before in filestructure
/// Therefore, it is possible to eliminate duplicate file names
fn build_filename_map(
    files: &Vec<TempFile>,
) -> Result<HashMap<String, (bool, &TempFile)>, ApiError> {
    let mut map = HashMap::new();
    for file in files {
        map.insert(
            file.file_name.clone().ok_or(ApiError::BadRequest {
                message: "You uploaded an file with invalid name".to_string(),
            })?,
            (false, file),
        );
    }
    Ok(map)
}
