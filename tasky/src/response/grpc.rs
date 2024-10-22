use crate::models::group::Group;

impl From<Group> for crate::tasky_grpc::Group {
    fn from(val: Group) -> Self {
        crate::tasky_grpc::Group {
            id: u64::try_from(val.id).unwrap(),
            title: val.title.clone(),
            member_count: u64::try_from(val.members.len()).unwrap(),
            tutor_id: u64::try_from(val.tutor).unwrap(),
        }
    }
}
