use crate::zettai::business::domain::{ToDoList, User};
use crate::zettai::test::tooling::app_for_at::AppForAT;

#[allow(dead_code)]
pub(crate) struct ToDoListOwner {
    user_name: String,
}

#[allow(dead_code)]
impl ToDoListOwner {
    pub(crate) fn new(user_name: &str) -> Self {
        ToDoListOwner {
            user_name: user_name.to_string(),
        }
    }

    pub(crate) fn user(&self) -> User {
        User {
            name: self.user_name.clone(),
        }
    }

    pub(crate) async fn can_see_the_todo_list(&self, expected_list: &ToDoList, app: &AppForAT) {
        let list = app
            .get_todo_list(&self.user(), &expected_list.list_name.name)
            .await;
        assert_eq!(list.list_name, expected_list.list_name);
        assert_eq!(list.items, expected_list.items);
    }

    pub(crate) async fn can_add_item_to_todo_list(
        &self,
        item: &str,
        list_name: &str,
        app: &AppForAT,
    ) {
        app.add_item_to_list(&self.user(), item, list_name).await;
    }

    pub(crate) async fn can_see_all_todo_lists(&self, all_lists: Vec<ToDoList>, app: &AppForAT) {
        let list_names = app.get_all_todo_lists(&self.user()).await;
        all_lists
            .iter()
            .for_each(|todo| assert_eq!(list_names.contains(&todo.list_name), true));
    }

    pub(crate) async fn can_not_see_any_todo_lists(&self, app: &AppForAT) {
        let lists = app.get_all_todo_lists(&self.user()).await;
        assert_eq!(lists.len(), 0);
    }
}
