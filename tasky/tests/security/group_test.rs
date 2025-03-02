use crate::security::get_student;
use crate::security::get_student_with_group;
use crate::security::get_tutor;
use crate::security::get_tutor_with_group;

use super::get_admin;
use chrono::NaiveDateTime;
use tasky;
use tasky::models::group::CreateGroup;
use tasky::models::group::Group;
use tasky::models::group::JoinRequestPolicy;
use tasky::security::IsGranted;
use tasky::security::SecurityAction;

#[test]
fn test_create_group() {
    let admin = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Create, &admin), false);
}

#[test]
fn test_read_group_as_admin() {
    let admin = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), true);
}

#[test]
fn test_read_group_as_tutor() {
    let admin = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), true);
}

#[test]
fn test_read_group_as_wrong_tutor() {
    let admin = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), false);
}

#[test]
fn test_read_group_as_student() {
    let admin = get_student_with_group();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), true);
}

#[test]
fn test_read_group_as_wrong_student() {
    let admin = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), false);
}

#[test]
fn test_update_as_admin() {
    let user = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_update_as_tutor() {
    let user = get_tutor_with_group();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_update_as_wrong_tutor() {
    let user = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_as_student() {
    let user = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_delete_as_admin() {
    let user = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_delete_as_tutor() {
    let user = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_delete_as_wrong_tutor() {
    let user = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_delete_as_student() {
    let user = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        tutor: 2,
        join_policy: JoinRequestPolicy::Request,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        verified: false,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_create_create_as_student() {
    let user = get_student();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_create_create_as_tutor() {
    let user = get_tutor();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), true);
}

#[test]
fn test_create_create_as_admin() {
    let user = get_admin();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_create_create_pending() {
    let user = get_admin();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        join_policy: JoinRequestPolicy::Request,
    };
    assert_eq!(create.is_granted(SecurityAction::Read, &user), false);
    assert_eq!(create.is_granted(SecurityAction::Update, &user), false);
    assert_eq!(create.is_granted(SecurityAction::Delete, &user), false);
}
