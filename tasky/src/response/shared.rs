use crate::api::UserResponse;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct User {
    id: u64,
    username: String,
    email: String,
}

impl Into<User> for UserResponse {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            email: self.email,
        }
    }
}
