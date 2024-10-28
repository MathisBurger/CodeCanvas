use crate::models::assignment_wish::{AssignmentWish, CreateAssignmentWish};

use super::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};

impl IsGranted for CreateAssignmentWish {
    fn is_granted(
        &mut self,
        action: super::SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        if action == SecurityAction::Create {
            return user.groups.contains(&self.group_id);
        }
        return false;
    }
}

impl IsGranted for AssignmentWish {
    fn is_granted(
        &mut self,
        action: SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        match action {
            SecurityAction::Delete => {
                StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, user)
                    || (user.groups.contains(&self.group_id)
                        && StaticSecurity::is_granted(StaticSecurityAction::IsTutor, user))
            }
            SecurityAction::Update => false,
            _ => {
                user.groups.contains(&self.group_id)
                    || StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, user)
            }
        }
    }
}
