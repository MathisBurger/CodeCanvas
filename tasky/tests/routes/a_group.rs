use super::*;
use actix_http::StatusCode;
use serial_test::serial;
use tasky::{models::group::JoinRequestPolicy, routes::group::CreateGroupRequest};

#[actix_web::test]
#[serial]
async fn test_a_create_group_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_json(CreateGroupRequest {
            title: "name123".to_string(),
            join_policy: JoinRequestPolicy::Request,
        });
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
#[serial]
async fn test_b_create_group_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_json(CreateGroupRequest {
            title: "name".to_string(),
            join_policy: JoinRequestPolicy::Request,
        });
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
#[serial]
async fn test_c_create_group_as_tutor_duplicate_name() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_json(CreateGroupRequest {
            title: "name".to_string(),
            join_policy: JoinRequestPolicy::Request,
        });
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status() == StatusCode::FOUND);
}

#[actix_web::test]
#[serial]
async fn test_d_create_group_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_json(CreateGroupRequest {
            title: "name2".to_string(),
            join_policy: JoinRequestPolicy::Request,
        });
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
#[serial]
async fn test_e_get_all_groups_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups?page=1");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_f_get_all_groups_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups?page=1");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_g_get_all_groups_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups?page=1");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_h_get_my_groups_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/my_groups?page=1");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_i_get_my_groups_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/my_groups?page=1");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_j_get_my_groups_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/my_groups?page=1");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_k_get_group_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
#[serial]
async fn test_l_get_group_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_m_get_group_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}
