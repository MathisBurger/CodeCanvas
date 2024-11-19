use diesel::{deserialize::Queryable, prelude::Insertable, RunQueryDsl, Selectable};

use crate::schema::notification_targets;

use super::DB;

#[derive(Queryable, Selectable, Clone, Insertable)]
#[diesel(primary_key(notification_id, user_id))]
#[diesel(table_name = notification_targets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NotificationTarget {
    pub notification_id: i32,
    pub user_id: i32,
}

pub struct NotificationTargetRepository;

impl NotificationTargetRepository {
    ///Creates a new notification target
    pub fn create(targets: Vec<NotificationTarget>, conn: &mut DB) {
        diesel::insert_into(notification_targets::table)
            .values(targets)
            .execute(conn)
            .expect("Cannot create new notification relation");
    }
}
