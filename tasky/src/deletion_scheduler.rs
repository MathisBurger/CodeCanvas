use crate::{
    models::DB, schema::group_join_requests::dsl as join_request_dsl,
    schema::notifications::dsl as notification_dsl,
};
use actix_rt::time;
use diesel::dsl::IntervalDsl;
use diesel::prelude::*;
use std::time::Duration;

/// Schedules deletion of entities after specific amount of time
pub async fn scheduler(conn: &mut DB) {
    let mut interval = time::interval(Duration::from_secs(3600 * 24));
    loop {
        diesel::delete(
            join_request_dsl::group_join_requests
                .filter(join_request_dsl::updated_at.lt(diesel::dsl::now - 20_i32.days())),
        )
        .execute(conn)
        .expect("Cannot delete join requests");

        diesel::delete(
            notification_dsl::notifications
                .filter(notification_dsl::created_at.lt(diesel::dsl::now - 20_i32.days())),
        )
        .execute(conn)
        .expect("Cannot delete notifications");
        diesel::delete(
            notification_dsl::notifications
                .filter(notification_dsl::show_until.lt(diesel::dsl::now)),
        )
        .execute(conn)
        .expect("Cannot delete system wide notifications");
        interval.tick().await;
    }
}
