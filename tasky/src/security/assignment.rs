use crate::models::assignment::{Assignment, CreateAssignment};

use super::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};

impl IsGranted for Assignment {
    fn is_granted(
        &mut self,
        action: SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        return match action {
            // Use CreateAssignment for creation instead
            SecurityAction::Create => false,
            SecurityAction::Read => user.groups.contains(&self.group_id),
            SecurityAction::Update => {
                StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, user)
                    && user.groups.contains(&self.group_id)
            }
            SecurityAction::Delete => false,
        };
    }
}

impl IsGranted for CreateAssignment {
    fn is_granted(
        &mut self,
        action: super::SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        if action == SecurityAction::Create {
            return StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, user)
                && user.groups.contains(&self.group_id);
        }
        false
    }
}
