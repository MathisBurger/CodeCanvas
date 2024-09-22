use crate::auth_middleware::{UserData, UserRole};

pub mod assignment;
pub mod group;
pub mod group_join_request;

/// Default security actions
#[derive(PartialEq)]
pub enum SecurityAction {
    Create,
    Read,
    Update,
    Delete,
}

/// Trait used to implement security actions on a specific struct
pub trait IsGranted {
    /// Checks if the action is granted with specific user on struct instance
    fn is_granted(&mut self, action: SecurityAction, user: &UserData) -> bool;
}

/// Static security action that is independent from a struct instance
#[derive(PartialEq)]
pub enum StaticSecurityAction {
    IsTutor,
    IsAdmin,
    IsStudent,
}

pub struct StaticSecurity;

impl StaticSecurity {
    /// Checks if the static security action is granted for user
    pub fn is_granted(action: StaticSecurityAction, user: &UserData) -> bool {
        match action {
            StaticSecurityAction::IsAdmin => {
                user.user_roles.contains(&UserRole::RoleAdmin.to_string())
            }
            StaticSecurityAction::IsTutor => {
                user.user_roles.contains(&UserRole::RoleTutor.to_string())
            }
            StaticSecurityAction::IsStudent => {
                user.user_roles.contains(&UserRole::RoleStudent.to_string())
            }
        }
    }
}
