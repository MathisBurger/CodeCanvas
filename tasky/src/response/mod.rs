use crate::{api::usernator_api_client::UsernatorApiClient, error::ApiError};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use tonic::transport::Channel;

pub mod assignment;
pub mod group;
pub mod group_join_request;
mod shared;

type DB = PooledConnection<ConnectionManager<PgConnection>>;

/// Generic trait used to enrich entities with external data from other data sources
pub trait Enrich<T>
where
    Self: Sized,
{
    /// Enriches the from entity with some data from external APIs
    async fn enrich(
        from: &T,
        client: &mut UsernatorApiClient<Channel>,
        db_conn: &mut DB,
    ) -> Result<Self, ApiError>;
}
