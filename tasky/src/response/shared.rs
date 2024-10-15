use crate::api::UserResponse;
use serde::Serialize;

/// The external user from usernator grpc server
#[derive(Clone, Serialize)]
pub struct User {
    id: u64,
    username: String,
    email: String,
}

/// Converts from user into user response
impl From<UserResponse> for User {
    fn from(val: UserResponse) -> Self {
        User {
            id: val.id,
            username: val.username,
            email: val.email,
        }
    }
}
