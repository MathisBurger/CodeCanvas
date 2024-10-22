use tasky::{
    models::solution::{NewSolution, Solution},
    security::{IsGranted, SecurityAction},
};

use super::*;

#[test]
fn test_new_create_as_wrong_student() {
    let user = get_student();
    let mut new = NewSolution {
        submitter_id: 2,
        assignment_id: 1,
        group_id: 1,
        approval_status: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_new_create_as_student() {
    let user = get_student_with_group();
    let mut new = NewSolution {
        submitter_id: 1,
        assignment_id: 1,
        group_id: 1,
        approval_status: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Create, &user), true);
}

#[test]
fn test_new_create_as_tutor() {
    let user = get_tutor_with_group();
    let mut new = NewSolution {
        submitter_id: 1,
        assignment_id: 1,
        group_id: 1,
        approval_status: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_new_create_as_admin() {
    let user = get_admin();
    let mut new = NewSolution {
        submitter_id: 1,
        assignment_id: 1,
        group_id: 1,
        approval_status: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Create, &user), false);
}

#[test]
fn test_read_as_wrong_student() {
    let user = get_student();
    let mut new = Solution {
        id: 1,
        submitter_id: 2,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Read, &user), false);
}

#[test]
fn test_read_as_student() {
    let user = get_student();
    let mut new = Solution {
        id: 1,
        submitter_id: 1,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Read, &user), true);
}

#[test]
fn test_read_as_wrong_tutor() {
    let user = get_tutor();
    let mut new = Solution {
        id: 1,
        submitter_id: 2,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Read, &user), false);
}

#[test]
fn test_read_as_tutor() {
    let user = get_tutor_with_group();
    let mut new = Solution {
        id: 1,
        submitter_id: 1,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Read, &user), true);
}

#[test]
fn test_read_as_admin() {
    let user = get_admin();
    let mut new = Solution {
        id: 1,
        submitter_id: 1,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Read, &user), true);
}

#[test]
fn test_update_as_wrong_student() {
    let user = get_student();
    let mut new = Solution {
        id: 1,
        submitter_id: 2,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_as_student() {
    let user = get_student();
    let mut new = Solution {
        id: 1,
        submitter_id: 1,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_as_wrong_tutor() {
    let user = get_tutor();
    let mut new = Solution {
        id: 1,
        submitter_id: 2,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Update, &user), false);
}

#[test]
fn test_update_as_tutor() {
    let user = get_tutor_with_group();
    let mut new = Solution {
        id: 1,
        submitter_id: 1,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Update, &user), true);
}

#[test]
fn test_update_as_admin() {
    let user = get_admin();
    let mut new = Solution {
        id: 1,
        submitter_id: 1,
        assignment_id: 1,
        group_id: Some(1),
        approval_status: None,
        file_structure: None,
        question_result: None,
        job_id: None,
    };
    assert_eq!(new.is_granted(SecurityAction::Update, &user), true);
}
