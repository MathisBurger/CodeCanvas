use chrono::NaiveDateTime;
use serial_test::serial;
use tasky::routes::assignment::{CreateAssignmentRequest, UpdateAssignmentRequest};

use super::*;

#[actix_web::test]
#[serial]
async fn test_a_get_all_group_assignments_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_b_get_all_group_assignments_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_c_get_all_group_assignments_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_d_create_assignment_as_student() {
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
#[serial]
async fn test_e_create_assignment_as_tutor() {
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
    let body_bytes = test::read_body(resp).await;
    println!(
        "BODY BYTES: {}",
        String::from_utf8(body_bytes.to_vec()).unwrap(),
    );
    //assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_f_create_assignment_as_admin() {
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
#[serial]
async fn test_g_get_assignment_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments/1");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_h_get_assignment_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments/1");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_i_get_assignment_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1/assignments/1");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_j_update_assignment_as_student() {
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
#[serial]
async fn test_k_update_assignment_as_tutor() {
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
#[serial]
async fn test_l_update_assignment_as_admin() {
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
