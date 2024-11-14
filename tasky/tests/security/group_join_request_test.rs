use chrono::NaiveDateTime;
use tasky::{
    models::group_join_request::GroupJoinRequest,
    security::{IsGranted, SecurityAction},
};

use super::*;

#[test]
fn test_create_permitted() {
    let admin = get_admin();
    let tutor = get_tutor_with_group();
    let student = get_student();
    let mut req = GroupJoinRequest {
        group_id: 1,
        requestor: 1,
        id: 1,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(req.is_granted(SecurityAction::Create, &admin), false);
    assert_eq!(req.is_granted(SecurityAction::Create, &tutor), false);
    assert_eq!(req.is_granted(SecurityAction::Create, &student), false);
}

#[test]
fn test_student_perms() {
    let user = get_student();
    let mut req = GroupJoinRequest {
        group_id: 1,
        requestor: 1,
        id: 1,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(req.is_granted(SecurityAction::Read, &user), false);
    assert_eq!(req.is_granted(SecurityAction::Update, &user), false);
    assert_eq!(req.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_wrong_tutor_perms() {
    let user = get_tutor();
    let mut req = GroupJoinRequest {
        group_id: 1,
        requestor: 1,
        id: 1,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(req.is_granted(SecurityAction::Read, &user), false);
    assert_eq!(req.is_granted(SecurityAction::Update, &user), false);
    assert_eq!(req.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_tutor_perms() {
    let user = get_tutor_with_group();
    let mut req = GroupJoinRequest {
        group_id: 1,
        requestor: 1,
        id: 1,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(req.is_granted(SecurityAction::Read, &user), true);
    assert_eq!(req.is_granted(SecurityAction::Update, &user), true);
    assert_eq!(req.is_granted(SecurityAction::Delete, &user), true);
}

#[test]
fn test_admin_perms() {
    let user = get_admin();
    let mut req = GroupJoinRequest {
        group_id: 1,
        requestor: 1,
        id: 1,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(req.is_granted(SecurityAction::Read, &user), true);
    assert_eq!(req.is_granted(SecurityAction::Update, &user), true);
    assert_eq!(req.is_granted(SecurityAction::Delete, &user), true);
}
