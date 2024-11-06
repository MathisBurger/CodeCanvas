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
            if (test_structure && file.is_test_file && file.object_id.is_none())
                || (!test_structure && !file.is_test_file)
            {
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
                // File exists already in filename_map or exists in old file structure
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
    if !structure.files.clone().unwrap_or_default().is_empty() {
        return true;
    }
    let folders = structure.folders.clone().unwrap_or_default();

    if !folders.is_empty() {
        for folder in folders {
            if file_structure_contains_files(&folder) {
                return true;
            }
        }
    }
    false
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

pub fn compare_structures(
    from: &AssignmentFileStructure,
    to: &AssignmentFileStructure,
) -> Result<(), ApiError> {
    for folder in to.folders.as_ref().unwrap_or(&mut default_structure_vec()) {
        if let Some(ref from_folder) = get_folder_by_name(&from, folder.current_folder_name.clone())
        {
            compare_structures(from_folder, folder)?;
        }
    }

    for file in to.files.clone().unwrap_or(default_vec()) {
        if let Some(from_file) = get_file_by_name(from, file.filename) {
            if from_file.object_id.is_some()
                && file.object_id.is_some()
                && from_file.object_id.unwrap() != file.object_id.unwrap()
            {
                return Err(ApiError::BadRequest {
                    message: "Invalid carry object_id".to_string(),
                });
            }
        }
    }

    Ok(())
}

fn get_folder_by_name(
    structure: &AssignmentFileStructure,
    name_option: Option<String>,
) -> Option<AssignmentFileStructure> {
    if let Some(name) = name_option {
        for folder in structure
            .folders
            .as_ref()
            .unwrap_or(&default_structure_vec())
        {
            if let Some(ref folder_name) = folder.current_folder_name {
                if *folder_name == name {
                    return Some(folder.clone());
                }
            }
        }
    }
    None
}

fn get_file_by_name(structure: &AssignmentFileStructure, name: String) -> Option<AssignmentFile> {
    for file in structure.files.as_ref().unwrap_or(&Vec::new()) {
        if file.filename == name {
            return Some(file.clone());
        }
    }
    None
}

fn default_vec() -> Vec<AssignmentFile> {
    vec![]
}

fn default_structure_vec() -> Vec<AssignmentFileStructure> {
    vec![]
}
