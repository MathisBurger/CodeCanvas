use tasky::auth_middleware::UserData;
use tasky::auth_middleware::UserRole;
use tasky::security::StaticSecurity;
use tasky::security::StaticSecurityAction;

pub mod assignment_test;
pub mod group_join_request_test;
pub mod group_test;
pub mod solution_test;

pub fn get_admin() -> UserData {
    UserData {
        user_id: 1,
        user_roles: vec![UserRole::RoleAdmin],
        groups: vec![],
    }
}

pub fn get_tutor() -> UserData {
    UserData {
        user_id: 1,
        user_roles: vec![UserRole::RoleTutor],
        groups: vec![],
    }
}

pub fn get_tutor_with_group() -> UserData {
    UserData {
        user_id: 1,
        user_roles: vec![UserRole::RoleTutor],
        groups: vec![1],
    }
}

pub fn get_student() -> UserData {
    UserData {
        user_id: 1,
        user_roles: vec![UserRole::RoleStudent],
        groups: vec![],
    }
}

pub fn get_student_with_group() -> UserData {
    UserData {
        user_id: 1,
        user_roles: vec![UserRole::RoleStudent],
        groups: vec![1],
    }
}

#[test]
fn test_is_admin() {
    let student = get_student();
    let tutor = get_tutor();
    let admin = get_admin();
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, &student),
        false
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, &tutor),
        false
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, &admin),
        true
    );
}

#[test]
fn test_is_tutor() {
    let student = get_student();
    let tutor = get_tutor();
    let admin = get_admin();
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsTutor, &student),
        false
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsTutor, &tutor),
        true
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsTutor, &admin),
        false
    );
}

#[test]
fn test_is_admin_or_tutor() {
    let student = get_student();
    let tutor = get_tutor();
    let admin = get_admin();
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, &student),
        false
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, &tutor),
        true
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, &admin),
        true
    );
}

#[test]
fn test_is_student() {
    let student = get_student();
    let tutor = get_tutor();
    let admin = get_admin();
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsStudent, &student),
        true
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsStudent, &tutor),
        false
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::IsStudent, &admin),
        false
    );
}

#[test]
fn test_can_view_file_structure() {
    let student = get_student();
    let tutor = get_tutor();
    let admin = get_admin();
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::CanViewTestStructure, &student),
        false
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::CanViewTestStructure, &tutor),
        true
    );
    assert_eq!(
        StaticSecurity::is_granted(StaticSecurityAction::CanViewTestStructure, &admin),
        true
    );
}
