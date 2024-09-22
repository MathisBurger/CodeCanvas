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
impl Into<User> for UserResponse {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            email: self.email,
        }
    }
}
