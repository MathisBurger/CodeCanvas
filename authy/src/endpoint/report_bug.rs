use crate::auth;
use crate::State;
use actix_web::{web, HttpRequest, HttpResponse};
use octorust::auth::Credentials;
use octorust::types::IssuesCreateRequest;
use octorust::types::IssuesCreateRequestLabelsOneOf;
use serde::Deserialize;
use serde::Serialize;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct ReportBugRequest {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct GithubRequest {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}

pub async fn report_bug(
    req: HttpRequest,
    state: web::Data<State>,
    body: web::Json<ReportBugRequest>,
) -> Result<HttpResponse, ApiError> {
    let app_config = &state.as_ref().config;
    auth::get_user_claims(&req, app_config.jwt_secret.clone())?;

    if app_config.github_api_key.is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let github = octorust::Client::new(
        String::from("user-agent-name"),
        Credentials::Token(app_config.github_api_key.clone().unwrap()),
    )
    .unwrap();

    let new_body = format!("THIS BUG IS SUBMITTED BY ITEGRATED CODECANVAS BUG REPORT FEATURE\nWe do not assume liability for content.\n\n{}", body.body.clone());

    github
        .issues()
        .create(
            "MathisBurger",
            "CodeCanvas",
            &IssuesCreateRequest {
                assignee: "MathisBurger".to_string(),
                assignees: vec![],
                body: new_body,
                labels: vec![IssuesCreateRequestLabelsOneOf::String("bug".to_string())],
                milestone: None,
                title: octorust::types::TitleOneOf::String(body.title.clone()),
            },
        )
        .await
        .map_err(|_x| ApiError::BadRequest {
            message: "Bad Request".to_string(),
        })?;

    Ok(HttpResponse::Ok().finish())
}
