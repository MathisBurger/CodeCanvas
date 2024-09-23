use crate::models::group::Group;

impl Into<crate::tasky_grpc::Group> for Group {
    fn into(self) -> crate::tasky_grpc::Group {
        crate::tasky_grpc::Group {
            id: u64::try_from(self.id).unwrap(),
            title: self.title.clone(),
            member_count: u64::try_from(self.members.len()).unwrap(),
            tutor_id: u64::try_from(self.tutor).unwrap(),
        }
    }
}
