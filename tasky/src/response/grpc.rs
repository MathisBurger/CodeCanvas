use crate::models::group::Group;

impl From<Group> for crate::tasky_grpc::Group {
    fn from(val: Group) -> Self {
        crate::tasky_grpc::Group {
            id: u64::try_from(val.id).unwrap(),
            title: val.title.clone(),
            // - deprecated
            // This field is deprecated and not used anymore
            // It will we removed in the future
            member_count: 0,
            tutor_id: u64::try_from(val.tutor).unwrap(),
        }
    }
}
