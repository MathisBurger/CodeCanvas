use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::models::assignment::AnswerType;
use crate::models::assignment::QuestionCatalogue;
use crate::models::assignment::QuestionCatalogueElement;
use serde_json::Value;

use crate::{
    error::ApiError,
    models::{
        assignment::{Assignment, AssignmentRepository},
        DB,
    },
};

/// Handles the creation of a question catalogue
pub fn handle_catalogue_creation(
    items: Vec<QuestionCatalogueElement>,
    assignment: &mut Assignment,
    conn: &mut DB,
) -> Result<(), ApiError> {
    let mut question_catalogue: HashMap<String, QuestionCatalogueElement> = HashMap::new();
    let mut hasher = DefaultHasher::new();
    for item in items {
        if !match_question_type(item.answer_type.clone(), item.answer.clone()) {
            return Err(ApiError::BadRequest {
                message: "Data types does not match".to_string(),
            });
        }
        item.clone().hash(&mut hasher);
        question_catalogue.insert(format!("{}", hasher.finish()), item.clone());
    }
    assignment.question_catalogue = Some(
        serde_json::to_value(QuestionCatalogue {
            catalogue: question_catalogue,
        })
        .unwrap(),
    );
    AssignmentRepository::update_assignment(assignment.clone(), conn);
    Ok(())
}

/// Matches the answer type of a question
pub fn match_question_type(answer_type: AnswerType, value: Value) -> bool {
    return match answer_type {
        AnswerType::String | AnswerType::StrContains => value.is_string(),
        AnswerType::Number => value.is_number(),
        AnswerType::Boolean => value.is_boolean(),
    };
}
