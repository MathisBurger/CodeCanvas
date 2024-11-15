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
        if files.is_empty() {
            return vec![];
        }
        let vec_len = files.len();
        let result = mongodb
            .collection("task_files")
            .insert_many(files, None)
            .await
            .unwrap()
            .inserted_ids;
        let mut object_ids = Vec::new();
        for i in 0..vec_len {
            if let Some(id) = result.get(&i) {
                object_ids.push(id.as_object_id().unwrap());
            }
        }
        object_ids
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

    pub async fn delete_for_solution_ids(ids: Vec<i32>, mongodb: &mongodb::Database) {
        let query = doc! { "solution_id": { "$in": ids } };
        mongodb
            .collection::<TaskFile>("task_files")
            .delete_many(query, None)
            .await
            .expect("Unable to delete entries");
    }
}
