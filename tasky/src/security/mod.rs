use crate::auth_middleware::{UserData, UserRole};

pub mod group;

#[derive(PartialEq)]
pub enum SecurityAction {
    Create,
    Read,
    Update,
    Delete,
}

pub trait IsGranted {
    fn is_granted(&mut self, action: SecurityAction, user: &UserData) -> bool;
}

#[derive(PartialEq)]
pub enum StaticSecurityAction {
    IsTutor,
    IsAdmin,
    IsStudent,
}

pub struct StaticSecurity;

impl StaticSecurity {
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
