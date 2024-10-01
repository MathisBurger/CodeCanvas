use bson::doc;
use bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

use super::read_cursor;

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
    pub async fn create_many(files: Vec<TestFile>, mongodb: &mongodb::Database) -> Vec<ObjectId> {
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

    pub async fn create(file: &TestFile, mongodb: &mongodb::Database) -> ObjectId {
        let serialized = bson::to_bson(file).unwrap();
        let document = serialized.as_document().unwrap();
        mongodb
            .collection("test_files")
            .insert_one(document.to_owned(), None)
            .await
            .unwrap()
            .inserted_id
            .as_object_id()
            .unwrap()
    }

    pub async fn get_for_assignment(
        assignment_id: i32,
        object_ids: Vec<ObjectId>,
        mongodb: &mongodb::Database,
    ) -> Vec<TestFile> {
        let mut cursor = mongodb
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