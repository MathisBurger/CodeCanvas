use crate::auth_middleware::UserData;
use crate::models::group::{CreateGroup, Group};

use super::{IsGranted, SecurityAction, StaticSecurity, StaticSecurityAction};

impl IsGranted for Group {
    fn is_granted(&mut self, action: super::SecurityAction, user: &UserData) -> bool {
        match action {
            // You will need to use CreateGroup
            SecurityAction::Create => false,
            SecurityAction::Read => {
                StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, user)
                    || (StaticSecurity::is_granted(StaticSecurityAction::IsTutor, user)
                        && self.tutor == user.user_id)
                    || (StaticSecurity::is_granted(StaticSecurityAction::IsStudent, user)
                        && user.groups.contains(&self.id))
            }
            SecurityAction::Update => {
                (StaticSecurity::is_granted(StaticSecurityAction::IsTutor, user)
                    && user.groups.contains(&self.id))
                    || StaticSecurity::is_granted(StaticSecurityAction::IsAdmin, user)
            }
            SecurityAction::Delete => false,
        }
    }
}

impl IsGranted for CreateGroup {
    fn is_granted(&mut self, action: SecurityAction, user: &UserData) -> bool {
        if action == SecurityAction::Create {
            return StaticSecurity::is_granted(StaticSecurityAction::IsTutor, user)
                && self.tutor == user.user_id;
        }
        false
    }
}
