use std::{collections::HashMap, iter::Map};

use actix_multipart::{
    form::{json::Json, tempfile::TempFile, MultipartForm},
    Multipart,
};
use actix_web::HttpResponse;
use mongodb::Database;
use serde::Deserialize;

use crate::{
    error::ApiError,
    models::{assignment::Assignment, DB},
    response::assignment::{AssignmentFile, AssignmentFileStructure},
};

#[derive(Deserialize)]
struct RunnerData {
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
    assignment: Assignment,
) -> Result<Assignment, ApiError> {
    // TODO: Validate runner cofig
    let mut file_structure = form.file_structure.0;
    let mut filename_map = build_filename_map(&form.files)?;
    let mut actual_files: Vec<&mut AssignmentFile> = vec![];
    validate_test_file_structure(&mut file_structure, &mut filename_map, &mut actual_files)?;
    // TODO: Create mongodb entries and only update file-structure refs. Then store structure.

    file_structure.files.unwrap();
    Err(ApiError::BadRequest {
        message: "".to_string(),
    })
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
        for mut folder in folders {
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
