use crate::security::get_student;
use crate::security::get_tutor;
use crate::security::get_tutor_with_group;

use super::get_admin;
use tasky;
use tasky::models::group::CreateGroup;
use tasky::models::group::Group;
use tasky::security::IsGranted;
use tasky::security::SecurityAction;

#[test]
fn test_create_group() {
    let admin = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 1,
    };
    assert_eq!(group.is_granted(SecurityAction::Create, &admin), false);
}

#[test]
fn test_read_group_as_admin() {
    let admin = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 1,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), true);
}

#[test]
fn test_read_group_as_tutor() {
    let admin = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 1,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), true);
}

#[test]
fn test_read_group_as_wrong_tutor() {
    let admin = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), false);
}

#[test]
fn test_read_group_as_student() {
    let admin = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![Some(1)],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), true);
}

#[test]
fn test_read_group_as_wrong_student() {
    let admin = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Read, &admin), false);
}

#[test]
fn test_update_as_admin() {
    let user = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_update_as_tutor() {
    let user = get_tutor_with_group();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 1,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_update_as_wrong_tutor() {
    let user = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_as_student() {
    let user = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_delete_as_admin() {
    let user = get_admin();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_delete_as_tutor() {
    let user = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 1,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_delete_as_wrong_tutor() {
    let user = get_tutor();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_delete_as_student() {
    let user = get_student();
    let mut group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 2,
    };
    assert_eq!(group.is_granted(SecurityAction::Delete, &user), false);
}

#[test]
fn test_create_create_as_student() {
    let user = get_student();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        members: vec![],
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_create_create_as_tutor() {
    let user = get_tutor();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        members: vec![],
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), true);
}

#[test]
fn test_create_create_as_admin() {
    let user = get_admin();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        members: vec![],
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_create_create_pending() {
    let user = get_admin();
    let mut create = CreateGroup {
        title: "".to_string(),
        tutor: 1,
        members: vec![],
    };
    assert_eq!(create.is_granted(SecurityAction::Read, &user), false);
    assert_eq!(create.is_granted(SecurityAction::Update, &user), false);
    assert_eq!(create.is_granted(SecurityAction::Delete, &user), false);
}
