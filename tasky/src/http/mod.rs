use crate::error::ApiError;
use crate::models::{assignment::Assignment, solution::Solution};
use crate::util::config::AppConfig;
use awc::Client;
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

pub async fn run_task(
    assignment: Assignment,
    solution: Solution,
    config: &AppConfig,
) -> Result<String, ApiError> {
    let client = Client::default();
    let mut res = client
        .post(format!("{}/execute", config.executor_http.clone()).as_str())
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
