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

    /*let mut root_cert_store = RootCertStore::empty();
    root_cert_store.add_server_trust_anchors(TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    // Configure the Rustls client with the root certificates
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth(); // No client-side authentication

    // Initialize Actix Client with the custom Rustls connector
    let client = Client::builder()
        .connector(Connector::new().rustls(Arc::new(config)))
        .finish();

    let resp = client
        .post("https://api.github.com/repos/MathisBurger/CodeCanvas/issues")
        .append_header((
            "Authorization",
            format!("Bearer {}", app_config.github_api_key.clone().unwrap()),
        ))
        .append_header(("X-GitHub-Api-Version", "2022-11-28"))
        .append_header(("Accept", "application/vnd.github+json"))
        .send_json(&GithubRequest {
            title: body.title.clone(),
            body: body.body.clone(),
            labels: vec!["bug".to_string()],
        })
        .await
        .map_err(|_x| ApiError::BadRequest {
            message: "Generel Error".to_string(),
        })?;*/

    let github = octorust::Client::new(
        String::from("user-agent-name"),
        Credentials::Token(app_config.github_api_key.clone().unwrap()),
    )
    .unwrap();

    github
        .issues()
        .create(
            "MathisBurger",
            "CodeCanvas",
            &IssuesCreateRequest {
                assignee: "MathisBurger".to_string(),
                assignees: vec![],
                body: body.body.clone(),
                labels: vec![IssuesCreateRequestLabelsOneOf::String("bug".to_string())],
                milestone: None,
                title: octorust::types::TitleOneOf::String(body.title.clone()),
            },
        )
        .await
        .map_err(|_x| ApiError::BadRequest {
            message: "Bad Request".to_string(),
        })?;

    return Ok(HttpResponse::Ok().finish());
}
