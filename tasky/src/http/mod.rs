use crate::error::ApiError;
use crate::models::{assignment::Assignment, solution::Solution};
use crate::util::config::AppConfig;
use awc::Client;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RunTaskRequest {
    pub assignment: Assignment,
    pub solution: Solution,
}

#[derive(Deserialize)]
struct RunTaskResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Job {
    pub id: String,
    pub execution: Vec<Execution>,
}

#[derive(Deserialize, Serialize)]
pub struct Execution {
    pub state: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

pub async fn run_task(
    assignment: Assignment,
    solution: Solution,
    config: &AppConfig,
) -> Result<String, ApiError> {
    let client = Client::default();
    let uri = format!("{}/execute", config.executor_http.clone());
    let mut res = client
        .post(uri)
        .send_json(&RunTaskRequest {
            assignment,
            solution,
        })
        .await
        .map_err(|e| ApiError::InternalServerError {
            message: e.to_string(),
        })?;
    let json = res
        .json::<RunTaskResponse>()
        .await
        .map_err(|e| ApiError::InternalServerError {
            message: e.to_string(),
        })?;
    Ok(json.id)
}

pub async fn get_job(id: &String) -> Result<Job, ApiError> {
    let client = Client::default();
    // TODO: Include config here
    let uri = format!("http://executor:8000/jobs/{}", id.clone());
    let mut res = client
        .get(uri)
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError {
            message: e.to_string(),
        })?;
    let json = res
        .json::<Job>()
        .await
        .map_err(|e| ApiError::InternalServerError {
            message: e.to_string(),
        })?;
    Ok(json)
}
