use bson::doc;
use bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

use super::read_cursor;

#[derive(Serialize, Deserialize)]
pub struct TestFile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    file_name: String,
    content: String,
    assignment_id: i32,
}

impl TestFile {
    pub async fn create(&mut self, mongodb: &mongodb::Database) -> ObjectId {
        let serialized = bson::to_bson(self).unwrap();
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
        mongodb: &mongodb::Database,
    ) -> Vec<TestFile> {
        let mut cursor = mongodb
            .collection("test_files")
            .find(Some(doc! {"assignment_id": assignment_id}), None)
            .await
            .unwrap();
        read_cursor(cursor).await
    }
}
