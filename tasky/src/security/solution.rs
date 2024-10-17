use crate::models::solution::{NewSolution, Solution};

use super::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};

impl IsGranted for NewSolution {
    fn is_granted(
        &mut self,
        action: super::SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        match action {
            SecurityAction::Create => {
                StaticSecurity::is_granted(super::StaticSecurityAction::IsStudent, user)
                    && self.submitter_id == user.user_id
            }
            _ => false,
        }
    }
}

impl IsGranted for Solution {
    fn is_granted(
        &mut self,
        action: SecurityAction,
        user: &crate::auth_middleware::UserData,
    ) -> bool {
        match action {
            SecurityAction::Create => false,
            SecurityAction::Read => {
                self.submitter_id == user.user_id
                    || (StaticSecurity::is_granted(
                        super::StaticSecurityAction::IsAdminOrTutor,
                        user,
                    ) && user.groups.contains(&self.group_id.unwrap_or(-1)))
            }
            _ => {
                (StaticSecurity::is_granted(super::StaticSecurityAction::IsTutor, user)
                    && user.groups.contains(&self.group_id.unwrap_or(-1)))
                    || StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, user)
            }
        }
    }
}
