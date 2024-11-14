use crate::schema::notifications::dsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

use super::group::GroupRepository;
use super::DB;

/// notification entry type
#[derive(Queryable, Selectable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub targeted_users: Vec<Option<i32>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// notification insert type
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::notifications)]
pub struct CreateNotification {
    pub title: String,
    pub content: String,
    pub targeted_users: Vec<Option<i32>>,
}

pub struct NotificationRepository;

impl NotificationRepository {
    /// Creates a new notification
    pub fn create_notification(notification: &CreateNotification, conn: &mut DB) -> Notification {
        diesel::insert_into(dsl::notifications::table())
            .values(notification)
            .returning(Notification::as_returning())
            .get_result::<Notification>(conn)
            .expect("Cannot create new notification")
    }

    /// Creates a notification for a group
    pub fn create_notification_for_group(
        title: String,
        content: String,
        group_id: i32,
        conn: &mut DB,
    ) -> Notification {
        Self::create_notification(
            &CreateNotification {
                title,
                content,
                targeted_users: GroupRepository::get_by_id(group_id, conn).unwrap().members,
            },
            conn,
        )
    }

    /// Gets all notifications for a user
    pub fn get_notifications_for_user(user_id: i32, conn: &mut DB) -> Vec<Notification> {
        dsl::notifications
            .filter(dsl::targeted_users.contains(vec![Some(user_id)]))
            .get_results::<Notification>(conn)
            .expect("Cannot get notifications for user")
    }

    /// Removes user from specfic notification
    pub fn remove_user_from_notification(id: i32, user_id: i32, conn: &mut DB) {
        sql_query("UPDATE notifications SET targeted_users = array_remove(targeted_users, $1) WHERE id = $2")
            .bind::<Integer, _>(user_id)
            .bind::<Integer, _>(id)
            .execute(conn)
            .expect("Cannot remove user from notification");
        sql_query("DELETE FROM notifications WHERE array_length(targeted_users, 1) IS NULL OR array_length(targeted_users, 1) = 0;").execute(conn).expect("Cannot remove pending notifications");
    }

    /// Removes user from all notifiations
    pub fn remove_user_from_all_notification(user_id: i32, conn: &mut DB) {
        sql_query("UPDATE notifications SET targeted_users = array_remove(targeted_users, $1) WHERE $1 = ANY(targeted_users)")
            .bind::<Integer, _>(user_id)
            .execute(conn)
            .expect("Cannot remove user from notification");
        sql_query("DELETE FROM notifications WHERE array_length(targeted_users, 1) IS NULL OR array_length(targeted_users, 1) = 0;").execute(conn).expect("Cannot remove pending notifications");
    }
}
