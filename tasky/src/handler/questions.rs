use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::models::assignment::QuestionCatalogue;
use crate::models::assignment::QuestionCatalogueElement;
use serde::{Deserialize, Serialize};

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
) {
    let mut question_catalogue: HashMap<u64, QuestionCatalogueElement> = HashMap::new();
    let mut hasher = DefaultHasher::new();
    for item in items {
        item.hash(&mut hasher);
        question_catalogue.insert(hasher.finish(), item);
    }
    assignment.question_catalogue = Some(
        serde_json::to_value(QuestionCatalogue {
            catalogue: question_catalogue,
        })
        .unwrap(),
    );
    AssignmentRepository::update_assignment(assignment.clone(), conn);
}
