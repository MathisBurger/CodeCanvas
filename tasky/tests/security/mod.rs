use tasky::auth_middleware::UserData;
use tasky::auth_middleware::UserRole;

pub mod group_test;

pub fn get_admin() -> UserData {
    UserData {
        user_id: 1,
        user_roles: vec![UserRole::RoleAdmin],
        groups: vec![],
    }
}
