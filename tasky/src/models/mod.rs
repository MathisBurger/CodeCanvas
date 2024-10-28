use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub mod assignment;
pub mod assignment_wish;
pub mod database;
pub mod group;
pub mod group_join_request;
pub mod solution;

pub type DB = PooledConnection<ConnectionManager<PgConnection>>;
