use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::read_cursor;

/// task file stored in mongodb
#[derive(Serialize, Deserialize)]
pub struct TaskFile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub file_name: String,
    pub content_size: usize,
    pub content: String,
    pub solution_id: i32,
}

pub struct TaskFileCollection;

impl TaskFileCollection {
    /// Creates many task files
    pub async fn create_many(files: Vec<TaskFile>, mongodb: &mongodb::Database) -> Vec<ObjectId> {
        mongodb
            .collection("task_files")
            .insert_many(files, None)
            .await
            .unwrap()
            .inserted_ids
            .values()
            .map(|x| x.as_object_id().unwrap())
            .collect()
    }

    /// Gets all task files for a solution by solution_id and object_ids
    pub async fn get_for_solution(
        solution_id: i32,
        object_ids: Vec<ObjectId>,
        mongodb: &mongodb::Database,
    ) -> Vec<TaskFile> {
        let cursor = mongodb
            .collection("task_files")
            .find(
                Some(doc! {"solution_id": solution_id, "_id": doc! {"$in": object_ids}}),
                None,
            )
            .await
            .unwrap();
        read_cursor(cursor).await
    }
}
