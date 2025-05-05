#[cfg(test)]
mod tests {
    use crate::zettai::business::domain::{ListName, ToDoList, User};
    use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
    use crate::zettai::test::tooling::app_for_at::AppForAT;
    use crate::zettai::test::tooling::todolist_owner::ToDoListOwner;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static DYLAN: Lazy<ToDoListOwner> = Lazy::new(|| ToDoListOwner::new("dylan"));

    static GARDENING_LIST: Lazy<ToDoList> = Lazy::new(|| ToDoList::new("gardening", vec![]));
    static MUSIC_LIST: Lazy<ToDoList> = Lazy::new(|| ToDoList::new("music", vec![]));

    static LISTS: Lazy<HashMap<User, HashMap<ListName, ToDoList>>> =
        Lazy::new(|| HashMap::from([(DYLAN.user(), HashMap::from([]))]));

    static FETCHER: Lazy<ToDoListFetcherFromMap> =
        Lazy::new(|| ToDoListFetcherFromMap::new(LISTS.clone()));

    #[tokio::test]
    async fn users_can_create_new_todo_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        DYLAN.can_not_see_any_todo_lists(&app).await;
        DYLAN.can_create_new_list("gardening", &app).await;
        DYLAN.can_create_new_list("music", &app).await;
        DYLAN.can_see_the_todo_list(&GARDENING_LIST, &app).await;
        DYLAN.can_see_the_todo_list(&MUSIC_LIST, &app).await;
    }
}
