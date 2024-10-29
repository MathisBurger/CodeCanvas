use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::read_cursor;

/// The test file stored in mongodb
#[derive(Serialize, Deserialize)]
pub struct TestFile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub file_name: String,
    pub content_size: usize,
    pub content: String,
    pub assignment_id: i32,
}

pub struct TestFileCollection;

impl TestFileCollection {
    /// Creates many test files
    pub async fn create_many(files: Vec<TestFile>, mongodb: &mongodb::Database) -> Vec<ObjectId> {
        if files.len() == 0 {
            return vec![];
        }
        mongodb
            .collection("test_files")
            .insert_many(files, None)
            .await
            .unwrap()
            .inserted_ids
            .values()
            .map(|x| x.as_object_id().unwrap())
            .collect()
    }

    /// gets all for an assignment
    pub async fn get_for_assignment(
        assignment_id: i32,
        object_ids: Vec<ObjectId>,
        mongodb: &mongodb::Database,
    ) -> Vec<TestFile> {
        let cursor = mongodb
            .collection("test_files")
            .find(
                Some(doc! {"assignment_id": assignment_id, "_id": doc! {"$in": object_ids}}),
                None,
            )
            .await
            .unwrap();
        read_cursor(cursor).await
    }
}
