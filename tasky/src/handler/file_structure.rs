use crate::error::ApiError;
use crate::response::assignment::AssignmentFile;
use crate::response::assignment::AssignmentFileStructure;
use actix_multipart::form::tempfile::TempFile;
use std::collections::HashMap;

/// Validates whether the given file structure is valid or not
/// It is being checked if all files in the structure are contained by the test/task files.
pub fn validate_test_file_structure<'a>(
    structure: &'a mut AssignmentFileStructure,
    files: &mut HashMap<String, (bool, &TempFile)>,
    actual_files: &mut Vec<&'a mut AssignmentFile>,
    test_structure: bool,
) -> Result<(), ApiError> {
    // Search all folders recursively
    if structure.folders.is_some() {
        let folders = structure.folders.as_mut().unwrap();
        for folder in folders {
            validate_test_file_structure(folder, files, actual_files, test_structure)?;
        }
    }

    if structure.files.is_some() {
        let folder_files = structure.files.as_mut().unwrap();
        for file in folder_files {
            // Only search for test files in this case
            if (test_structure && file.is_test_file) || (!test_structure && !file.is_test_file) {
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
pub fn file_structure_contains_files(structure: &AssignmentFileStructure) -> bool {
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
pub fn build_filename_map(
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
