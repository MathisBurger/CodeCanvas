use chrono::NaiveDateTime;
use tasky::routes::assignment::{CreateAssignmentRequest, UpdateAssignmentRequest};

use super::*;

#[actix_web::test]
async fn test_get_all_group_assignments_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_get_all_group_assignments_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_get_all_group_assignments_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_create_assignment_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/groups/1/assignments")
        .set_json(CreateAssignmentRequest {
            title: "".to_string(),
            due_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            description: "".to_string(),
            language: tasky::models::assignment::AssignmentLanguage::Java,
        });
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_create_assignment_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/groups/1/assignments")
        .set_json(CreateAssignmentRequest {
            title: "".to_string(),
            due_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            description: "".to_string(),
            language: tasky::models::assignment::AssignmentLanguage::Java,
        });
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_create_assignment_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/groups/1/assignments")
        .set_json(CreateAssignmentRequest {
            title: "".to_string(),
            due_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            description: "".to_string(),
            language: tasky::models::assignment::AssignmentLanguage::Java,
        });
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_get_assignment_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments/1");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_get_assignment_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments/1");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_get_assignment_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments/1");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_update_assignment_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/groups/1/assignments/1/update")
        .set_json(UpdateAssignmentRequest {
            title: "".to_string(),
            due_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            description: "".to_string(),
        });
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_update_assignment_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/groups/1/assignments/1/update")
        .set_json(UpdateAssignmentRequest {
            title: "".to_string(),
            due_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            description: "".to_string(),
        });
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_update_assignment_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/groups/1/assignments/1/update")
        .set_json(UpdateAssignmentRequest {
            title: "".to_string(),
            due_date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            description: "".to_string(),
        });
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}
