use crate::zettai::business::domain::{ToDoList, User};
use crate::zettai::test::app_for_at::AppForAT;

#[allow(dead_code)]
pub(super) struct ToDoListOwner {
    user_name: String,
}

#[allow(dead_code)]
impl ToDoListOwner {
    pub(super) fn new(user_name: &str) -> Self {
        ToDoListOwner { user_name: user_name.to_string() }
    }

    pub(super) fn user(&self) -> User {
        User { name: self.user_name.clone() }
    }

    pub(super) async fn can_see_the_list(&self, expected_list: &ToDoList, app: &AppForAT) {
        let list = app.get_to_do_list(&self.user(), &expected_list.list_name.name).await;
        assert_eq!(list.list_name, expected_list.list_name);
        assert_eq!(list.items, expected_list.items);
    }

    pub(super) async fn can_add_item_to_list(&self, item: &str, list_name: &str, app: &AppForAT) {
        app.add_item_to_list(&self.user(), item, list_name).await;
    }

}