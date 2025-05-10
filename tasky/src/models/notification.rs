use crate::schema::group_members;
use crate::schema::notification_targets;
use crate::schema::notifications::dsl;
use crate::schema::notifications::table;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

use super::notification_target::NotificationTarget;
use super::notification_target::NotificationTargetRepository;
use super::DB;

/// notification entry type
#[derive(Queryable, Selectable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub show_until: Option<NaiveDateTime>,
    pub system_wide: bool,
}

/// notification insert type for external calls
pub struct CreateNotification {
    pub title: String,
    pub content: String,
    pub targeted_users: Vec<i32>,
}

/// create model used internally for notifications
#[derive(Insertable)]
#[diesel(table_name = crate::schema::notifications)]
struct InternalCreate {
    pub title: String,
    pub content: String,
    pub show_until: Option<NaiveDateTime>,
    pub system_wide: bool,
}

pub struct NotificationRepository;

impl NotificationRepository {
    /// Creates a new notification
    pub fn create_notification(notification: &CreateNotification, conn: &mut DB) -> Notification {
        let notification_create = InternalCreate {
            title: notification.title.clone(),
            content: notification.content.clone(),
            show_until: None,
            system_wide: false,
        };

        let created = diesel::insert_into(dsl::notifications::table())
            .values(notification_create)
            .returning(Notification::as_returning())
            .get_result::<Notification>(conn)
            .expect("Cannot create new notification");

        let targets: Vec<NotificationTarget> = notification
            .targeted_users
            .iter()
            .map(|u| NotificationTarget {
                notification_id: created.id,
                user_id: *u,
            })
            .collect();

        NotificationTargetRepository::create(targets, conn);
        created
    }

    /// Creates a notification for a group
    pub fn create_notification_for_group(
        title: String,
        content: String,
        group_id: i32,
        conn: &mut DB,
    ) -> Notification {
        let members: Vec<i32> = group_members::dsl::group_members
            .filter(group_members::dsl::group_id.eq(group_id))
            .select(group_members::dsl::member_id)
            .get_results::<i32>(conn)
            .expect("Cannot fetch group members");
        Self::create_notification(
            &CreateNotification {
                title,
                content,
                targeted_users: members,
            },
            conn,
        )
    }

    /// Gets all notifications for a user
    pub fn get_notifications_for_user(user_id: i32, conn: &mut DB) -> Vec<Notification> {
        dsl::notifications
            .left_join(notification_targets::table)
            .filter(notification_targets::user_id.eq(user_id))
            .select(Notification::as_select())
            .order(dsl::created_at.desc())
            .get_results::<Notification>(conn)
            .expect("Cannot get notifications for user")
    }

    /// Removes user from specfic notification
    pub fn remove_user_from_notification(id: i32, user_id: i32, conn: &mut DB) {
        diesel::delete(
            notification_targets::dsl::notification_targets.filter(
                notification_targets::user_id
                    .eq(user_id)
                    .and(notification_targets::notification_id.eq(id)),
            ),
        )
        .execute(conn)
        .expect("Cannot remove user from notification");
    }

    /// Removes user from all notifiations
    pub fn remove_user_from_all_notification(user_id: i32, conn: &mut DB) {
        diesel::delete(
            notification_targets::dsl::notification_targets
                .filter(notification_targets::user_id.eq(user_id)),
        )
        .execute(conn)
        .expect("Cannot remove user from notification");
    }

    /// Creates an system wide notification
    pub fn create_system_wide_notification(
        title: String,
        content: String,
        show_until: NaiveDateTime,
        conn: &mut DB,
    ) {
        diesel::insert_into(table)
            .values(InternalCreate {
                title: title.clone(),
                content: content.clone(),
                show_until: Some(show_until),
                system_wide: true,
            })
            .execute(conn)
            .expect("Cannot create system wide notification");
    }

    /// Gets all system wide notificytions
    pub fn get_system_wide(conn: &mut DB) -> Vec<Notification> {
        dsl::notifications
            .filter(
                dsl::system_wide
                    .eq(true)
                    .and(dsl::show_until.gt(diesel::dsl::now)),
            )
            .get_results::<Notification>(conn)
            .expect("Cannot load all system wide notifications")
    }

    /// Deletes an notification by ID
    pub fn delete(id: i32, conn: &mut DB) {
        diesel::delete(dsl::notifications.filter(dsl::id.eq(id)))
            .execute(conn)
            .expect("Cannot delete notification");
    }
}
