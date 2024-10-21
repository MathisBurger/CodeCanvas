use super::*;

#[actix_web::test]
async fn test_create_join_request_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/create_join_request");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_create_join_request_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/create_join_request");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_create_join_request_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/create_join_request");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_get_join_requests_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_get_join_requests_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_get_join_requests_as_admin() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_approve_join_request_as_student() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests/1/approve");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_approve_join_request_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests/1/approve");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_approve_join_request_as_admin() {
    create_join_request().await;
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests/2/approve");
    req = admin(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
async fn test_reject_join_request_as_student() {
    create_join_request2().await;
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests/3/approve");
    req = student(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

#[actix_web::test]
async fn test_reject_join_request_as_tutor() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/join_requests/3/approve");
    req = tutor(req);
    let resp = test::call_service(&app, req.to_request()).await;
    assert!(resp.status().is_client_error())
}

async fn create_join_request() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/create_join_request");
    req = student2(req);
    test::call_service(&app, req.to_request()).await;
}

async fn create_join_request2() {
    let app = get_app().await;
    let mut req = test::TestRequest::post().uri("/groups/1/create_join_request");
    req = student3(req);
    test::call_service(&app, req.to_request()).await;
}
