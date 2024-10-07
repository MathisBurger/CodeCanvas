use crate::error::ApiError;
use crate::models::{assignment::Assignment, solution::Solution};
use crate::util::config::AppConfig;
use awc::Client;
use serde::{Deserialize, Serialize};

/// Request to run a task in executor
#[derive(Serialize)]
struct RunTaskRequest {
    pub assignment: Assignment,
    pub solution: Solution,
}

/// Response of RunTaskRequest
#[derive(Deserialize)]
struct RunTaskResponse {
    pub id: String,
}

/// A job run by the executor
#[derive(Deserialize, Serialize)]
pub struct Job {
    pub id: String,
    pub execution: Vec<Execution>,
}

/// Execution cycle of a executor job
#[derive(Deserialize, Serialize)]
pub struct Execution {
    pub state: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

/// Sends a http request to the executor to run a task (job)
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

/// Gets a job from executor
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
