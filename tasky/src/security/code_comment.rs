use crate::models::code_comment::CreateCodeComment;

use super::{IsGranted, SecurityAction, StaticSecurity};

impl IsGranted for CreateCodeComment {
    fn is_granted(
        &mut self,
        action: super::SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        if action == SecurityAction::Create {
            return StaticSecurity::is_granted(super::StaticSecurityAction::IsAdmin, user)
                || (StaticSecurity::is_granted(super::StaticSecurityAction::IsTutor, user)
                    && user.groups.contains(&self.group_id));
        }
        false
    }
}
