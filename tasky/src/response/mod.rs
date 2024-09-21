use crate::{api::usernator_api_client::UsernatorApiClient, error::ApiError};
use tonic::transport::Channel;

pub mod group;
mod shared;

pub trait Enrich<F, T> {
    async fn enrich(from: &F, client: &mut UsernatorApiClient<Channel>) -> Result<T, ApiError>;
}
