use crate::models::assignment_wish::{AssignmentWish, CreateAssignmentWish};

use super::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};

impl IsGranted for CreateAssignmentWish {
    fn is_granted(
        &mut self,
        action: super::SecurityAction,
        _user: &crate::auth_middleware::UserData,
    ) -> bool {
        if action == SecurityAction::Create {
            return true;
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
                StaticSecurity::is_granted(StaticSecurityAction::IsAdminOrTutor, user)
            }
            SecurityAction::Update => false,
            _ => true,
        }
    }
}
