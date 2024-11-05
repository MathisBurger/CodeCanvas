use diesel::prelude::*;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use diesel::query_dsl::LoadQuery;
use diesel::r2d2::{ConnectionManager, PooledConnection};

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

#[derive(Debug, Clone)]
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
        // Ensure the original query can load results of type `U` with a `PgConnection`
        T: LoadQuery<'a, PgConnection, U> + LimitDsl,
        // Ensure the limited query can be offset
        <T as LimitDsl>::Output: OffsetDsl,
        // Ensure the fully chained query (limit + offset) supports loading with `PgConnection`
        <<T as LimitDsl>::Output as OffsetDsl>::Output: LoadQuery<'a, PgConnection, U>,
    {
        let total = diesel::select(diesel::dsl::count_star()).get_result::<i64>(conn)?;
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
