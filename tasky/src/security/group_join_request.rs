use crate::models::group_join_request::GroupJoinRequest;

use super::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};

impl IsGranted for GroupJoinRequest {
    fn is_granted(
        &mut self,
        action: super::SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        match action {
            // Is handled in controller
            SecurityAction::Create => false,
            SecurityAction::Update | SecurityAction::Delete | SecurityAction::Read => {
                (StaticSecurity::is_granted(StaticSecurityAction::IsTutor, user)
                    && user.groups.contains(&self.group_id))
                    || StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, user)
            }
        }
    }
}
