use super::file_structure::*;
use super::questions::match_question_type;
use crate::models::assignment::QuestionCatalogue;
use crate::models::assignment::{AnswerType, QuestionCatalogueElement};
use crate::models::solution::ApprovalStatus;
use crate::models::solution::QuestionResult;
use crate::models::solution::QuestionSolution;
use crate::response::assignment::AssignmentFile;
use crate::{models::DB, security::IsGranted};
use actix_multipart::form::json::Json;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use mongodb::Database;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;

use crate::{
    auth_middleware::UserData,
    error::ApiError,
    models::{
        assignment::Assignment,
        solution::{NewSolution, Solution, SolutionRepository},
    },
    mongo::task_file::{TaskFile, TaskFileCollection},
};

/// Multipart form to create solution
#[derive(MultipartForm)]
pub struct CreateSolutionMultipart {
    #[multipart(limit = "10MB")]
    pub files: Vec<TempFile>,
    pub answers: Json<HashMap<String, QuestionSolution>>,
}

/// Handles the creation of a solution
/// This means persisting data in postgres, files in mongodb
/// and validating the input data
pub async fn handle_create_multipart(
    form: CreateSolutionMultipart,
    user_data: &UserData,
    mongodb: &Database,
    db: &mut DB,
    assignment: &Assignment,
) -> Result<Solution, ApiError> {
    let mut new_solution = NewSolution {
        submitter_id: user_data.user_id,
        assignment_id: assignment.id,
        approval_status: Some(ApprovalStatus::Pending.string()),
        group_id: assignment.group_id,
    };

    if !new_solution.is_granted(crate::security::SecurityAction::Create, user_data) {
        return Err(ApiError::Forbidden {
            message: "You cannot create a new solution".to_string(),
        });
    }

    if form.answers.is_empty() && form.files.is_empty() {
        return Err(ApiError::BadRequest {
            message: "No data provided".to_string(),
        });
    }
    let mut solution = SolutionRepository::create_solution(new_solution, db);

    if assignment.file_structure.is_none() && !form.files.is_empty() {
        return Err(ApiError::BadRequest {
            message: "No code tests submitted. Therefore, no solutions can be handed in."
                .to_string(),
        });
    }
    if assignment.question_catalogue.is_none() && !form.answers.is_empty() {
        return Err(ApiError::BadRequest {
            message: "No questions submitted. Therefore, no solutions can be handed in."
                .to_string(),
        });
    }

    if form.files.is_empty() {
        handle_questions(assignment, form, &mut solution)?;
    } else {
        handle_file_structure(assignment, form, &mut solution, mongodb).await?;
    }

    SolutionRepository::update_solution(solution.clone(), db);
    Ok(solution)
}

/// Handles solution creation in case of a file based solution (Code execution)
async fn handle_file_structure(
    assignment: &Assignment,
    form: CreateSolutionMultipart,
    solution: &mut Solution,
    mongodb: &Database,
) -> Result<(), ApiError> {
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
        serde_json::to_value(file_structure).map_err(|_| ApiError::InternalServerError {
            message: "Cannot convert file structure to JSON".to_string(),
        })?;

    solution.file_structure = Some(file_structure_value);
    Ok(())
}

/// Handles the submission of answers as solutions
fn handle_questions(
    assignment: &Assignment,
    form: CreateSolutionMultipart,
    solution: &mut Solution,
) -> Result<(), ApiError> {
    let question_catalogue: QuestionCatalogue =
        serde_json::from_value(assignment.question_catalogue.clone().unwrap()).map_err(|_| {
            ApiError::InternalServerError {
                message: "Cannot parse question catalogue".to_string(),
            }
        })?;

    let mut all_correct = true;
    let mut result: Vec<QuestionResult> = vec![];

    for (hash, question) in question_catalogue.catalogue {
        if !form.answers.contains_key(&hash) {
            return Err(ApiError::BadRequest {
                message: "Missing answer".to_string(),
            });
        }
        let answer = form.answers.get(&hash).unwrap();
        if compare_answers(&question, answer.answer.clone()) {
            result.push(QuestionResult {
                question: question.question.clone(),
                correct: true,
                answer: answer.answer.clone(),
            });
        } else {
            result.push(QuestionResult {
                question: question.question.clone(),
                correct: false,
                answer: answer.answer.clone(),
            });
            all_correct = false;
        }
    }
    solution.question_result = serde_json::to_value(result).ok();
    solution.approval_status = Some(
        all_correct
            .then(|| ApprovalStatus::Successful.string())
            .unwrap_or(ApprovalStatus::Failed.string()),
    );
    Ok(())
}

/// Compares an answer to the value it should be
fn compare_answers(question: &QuestionCatalogueElement, answer: Value) -> bool {
    if !match_question_type(question.answer_type.clone(), answer.clone()) {
        return false;
    }
    match question.answer_type {
        AnswerType::Number => question.answer.as_number().unwrap() == answer.as_number().unwrap(),
        AnswerType::String => question.answer.as_str().unwrap() == answer.as_str().unwrap(),
        AnswerType::StrContains => answer
            .as_str()
            .unwrap()
            .contains(question.answer.as_str().unwrap()),
        AnswerType::Boolean => question.answer.as_bool().unwrap() == answer.as_bool().unwrap(),
    }
}
