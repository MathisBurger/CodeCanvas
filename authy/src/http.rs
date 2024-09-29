use crate::config::AppConfig;
use crate::error::ApiError;
use actix_web::dev::ResourcePath;
use actix_web::{web, HttpRequest, HttpResponse};
use awc::Client;

pub(crate) struct ProxyClient {
    config: AppConfig,
}

impl ProxyClient {
    /// Creates a new instance
    pub(crate) fn new(config: AppConfig) -> Self {
        ProxyClient { config }
    }

    /// Proxies a generic http request to a specific microservice
    pub async fn proxy_request(
        &self,
        req: &HttpRequest,
        payload: web::Payload,
        headers: Vec<(&str, String)>,
    ) -> Result<HttpResponse, ApiError> {
        let service_uri = self.get_request_path(req)?;
        let client = Client::new();
        let mut request = client.request_from(service_uri, req.head()).no_decompress();
        for header in headers {
            request = request.insert_header(header)
        }
        let response = request.send_stream(payload).await.map_err(|e| {
            error!(target: "proxy", "{}", format!("Error while proxy request: {}", e));
            ApiError::BadRequest {
                message: "Cannot proxy request".to_string(),
            }
        })?;
        let mut client_resp = HttpResponse::build(response.status());
        for (header_name, header_value) in response.headers().iter().filter(|(h, _)| {
            *h != "connection" && *h != "X-CodeCanvas-UserId" && *h != "X-CodeCanvas-UserRoles"
        }) {
            client_resp.insert_header((header_name.clone(), header_value.clone()));
        }
        Ok(client_resp.streaming(response))
    }

    /// gets the request path for the local service
    fn get_request_path(&self, req: &HttpRequest) -> Result<String, ApiError> {
        let key = self.get_service_key(&req.path().path().to_string())?;
        let location = self.get_service_location(&key)?;
        let mut fullpath = format!(
            "{}?{}",
            req.path().to_string(),
            req.query_string().to_string()
        );
        fullpath = fullpath.replace(format!("/{}", key).as_str(), "");
        return Ok(format!("{}{}", location, fullpath));
    }

    /// Gets the ID of the service
    fn get_service_key(&self, path: &String) -> Result<String, ApiError> {
        let spl: Vec<&str> = path.split("/").collect();
        let first = spl.get(1);
        if first.is_none() {
            return ApiError::BadRequest {
                message: "Cannot get service key for proxy".to_string(),
            }
            .into();
        }
        Ok(first.unwrap().to_string())
    }

    /// Gets the local location of a microservice by key
    fn get_service_location(&self, key: &String) -> Result<&String, ApiError> {
        if !self.config.service_locations.contains_key(key) {
            return ApiError::BadRequest {
                message: "Cannot get service location".to_string(),
            }
            .into();
        }
        Ok(self.config.service_locations.get(key).unwrap())
    }
}
