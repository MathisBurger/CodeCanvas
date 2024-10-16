use super::get_admin;
use tasky;
use tasky::models::Group;
use tasky::security::SecurityAction;

#[test]
fn test_create_group() {
    let admin = get_admin();
    let group = Group {
        id: 1,
        title: "group".to_string(),
        members: vec![],
        tutor: 1,
    };
    assert_eq!(group.is_granted(SecurityAction::Create, &admin), false);
}
