use crate::error::ApiError;
use bson::oid::ObjectId;
use std::str::FromStr;

/// Converts a raw string seterated by commas into an vec of object IDs
pub fn parse_object_ids(raw: String) -> Result<Vec<ObjectId>, ApiError> {
    let raw_ids: Vec<&str> = raw.split(",").collect();
    let mut ids: Vec<ObjectId> = vec![];
    for id in raw_ids {
        ids.push(ObjectId::from_str(id).map_err(|_x| ApiError::BadRequest {
            message: "Invalid ObjectID".to_string(),
        })?);
    }
    Ok(ids)
}
