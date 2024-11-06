use diesel::dsl::{count_star, CountStar};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl, SelectDsl};
use diesel::query_dsl::LoadQuery;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sql_types::HasSqlType;
use diesel::sql_types::SingleValue;
use serde::Serialize;

pub mod assignment;
pub mod assignment_wish;
pub mod code_comment;
pub mod database;
pub mod group;
pub mod group_join_request;
pub mod solution;

pub type DB = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone, Copy)]
pub struct Paginated<T> {
    query: T,
    page: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedModel<T> {
    pub results: Vec<T>,
    pub page: i64,
    pub total: i64,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated { query: self, page }
    }
}

impl<T> Paginated<T> {
    const PAGE_SIZE: i64 = 50;

    pub fn load_and_count_pages<'a, U>(self, conn: &mut DB) -> QueryResult<PaginatedModel<U>>
    where
        T: LoadQuery<'a, PgConnection, U> + LimitDsl + SelectDsl<CountStar> + Clone,
        <T as LimitDsl>::Output: OffsetDsl,
        <<T as LimitDsl>::Output as OffsetDsl>::Output: LoadQuery<'a, PgConnection, U>,
        <T as SelectDsl<CountStar>>::Output: RunQueryDsl<PgConnection>,
        <T as SelectDsl<CountStar>>::Output: QueryId,
        <T as SelectDsl<CountStar>>::Output: QueryFragment<Pg>,
        <T as SelectDsl<CountStar>>::Output: diesel::query_builder::Query,
        i64: diesel::deserialize::FromSql<
            <<T as SelectDsl<CountStar>>::Output as diesel::query_builder::Query>::SqlType,
            Pg,
        >,
        Pg: HasSqlType<
            <<T as SelectDsl<CountStar>>::Output as diesel::query_builder::Query>::SqlType,
        >,
        <<T as SelectDsl<CountStar>>::Output as diesel::query_builder::Query>::SqlType: SingleValue,
    {
        let total = self
            .query
            .clone()
            .select(count_star())
            .get_result::<i64>(conn)?;

        let results = self
            .query
            .limit(Self::PAGE_SIZE)
            .offset((self.page - 1) * Self::PAGE_SIZE)
            .load::<U>(conn)?;

        Ok(PaginatedModel {
            results,
            page: self.page,
            total,
        })
    }
}
