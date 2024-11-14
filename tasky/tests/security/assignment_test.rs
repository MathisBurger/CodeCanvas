use chrono::NaiveDateTime;
use tasky::{
    models::assignment::{Assignment, AssignmentLanguage, CreateAssignment},
    security::{IsGranted, SecurityAction},
};

use super::*;

#[test]
fn test_create_assignment_disabled() {
    let user = get_admin();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_read_assignment_as_wrong_student() {
    let user = get_student();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Read, &user), false);
}

#[test]
fn test_read_assignment_as_student() {
    let user = get_student_with_group();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Read, &user), true);
}

#[test]
fn test_read_assignment_as_wrong_tutor() {
    let user = get_tutor();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Read, &user), false);
}

#[test]
fn test_read_assignment_as_tutor() {
    let user = get_tutor_with_group();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Read, &user), true);
}

#[test]
fn test_read_assignment_as_admin() {
    let user = get_admin();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Read, &user), true);
}

#[test]
fn test_update_assignment_as_wrong_student() {
    let user = get_student();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_assignment_as_student() {
    let user = get_student_with_group();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_assignment_as_wrong_tutor() {
    let user = get_tutor();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_assignment_as_tutor() {
    let user = get_tutor_with_group();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_update_assignment_as_admin() {
    let user = get_admin();
    let mut assignment = Assignment {
        id: 1,
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
        completed_by: vec![],
        file_structure: None,
        runner_cmd: "".to_string(),
        runner_memory: "".to_string(),
        runner_timeout: "".to_string(),
        runner_cpu: "".to_string(),
        question_catalogue: None,
        created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    };
    assert_eq!(assignment.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_create_create_assignment_as_student() {
    let user = get_student_with_group();
    let mut create = CreateAssignment {
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_create_create_assignment_as_wrong_tutor() {
    let user = get_tutor();
    let mut create = CreateAssignment {
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_create_create_assignment_as_tutor() {
    let user = get_tutor_with_group();
    let mut create = CreateAssignment {
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), true);
}

#[test]
fn test_create_create_assignment_as_admin() {
    let user = get_admin();
    let mut create = CreateAssignment {
        title: "".to_string(),
        due_date: Some(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        group_id: 1,
        description: "".to_string(),
        language: AssignmentLanguage::Golang,
    };
    assert_eq!(create.is_granted(SecurityAction::Create, &user), true);
}
