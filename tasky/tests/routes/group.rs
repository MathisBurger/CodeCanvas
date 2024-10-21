use super::*;
use tasky::routes::group::CreateGroupRequest;

#[actix_web::test]
async fn test_create_group_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_payload(CreateGroupRequest {
            title: "name".to_string(),
        });
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_create_group_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_payload(CreateGroupRequest {
            title: "name".to_string(),
        });
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_create_group_as_tutor_duplicate_name() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_payload(CreateGroupRequest {
            title: "name".to_string(),
        });
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_create_group_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::post()
        .uri("/create_group")
        .set_payload(CreateGroupRequest {
            title: "name2".to_string(),
        });
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_get_all_groups_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_all_groups_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_all_groups_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_my_groups_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/my_groups");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_my_groups_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/my_groups");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_my_groups_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/my_groups");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_group_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_get_group_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_group_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::get().uri("/groups/1");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success());
}
