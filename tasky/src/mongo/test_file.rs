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
        if files.is_empty() {
            return vec![];
        }
        let vec_len = files.len();
        let result = mongodb
            .collection("test_files")
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

    /// Deletes all entries with assignment ID
    pub async fn delete_for_assignment_ids(ids: Vec<i32>, mongodb: &mongodb::Database) {
        let query = doc! { "assignment_id": { "$in": ids } };
        mongodb
            .collection::<TestFile>("test_files")
            .delete_many(query, None)
            .await
            .expect("Unable to delete entries");
    }
}
