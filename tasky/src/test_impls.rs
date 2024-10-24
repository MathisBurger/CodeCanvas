use actix_web::web::Bytes;
use serde::Serialize;

use crate::routes::group::CreateGroupRequest;

pub fn json_bytes<T>(structure: T) -> Vec<u8>
where
    T: Serialize,
{
    let mut bytes: Vec<u8> = Vec::new();
    serde_json::to_writer(&mut bytes, &structure).unwrap();
    bytes
}

impl From<CreateGroupRequest> for Bytes {
    fn from(val: CreateGroupRequest) -> Bytes {
        Bytes::from(json_bytes(val))
    }
}
