#[cfg(test)]
mod tests {
    use crate::zettai::business::domain::{ListName, ToDoList, User};
    use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
    use crate::zettai::test::tooling::app_for_at::AppForAT;
    use crate::zettai::test::tooling::todolist_owner::ToDoListOwner;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static CAROL: Lazy<ToDoListOwner> = Lazy::new(|| ToDoListOwner::new("carol"));
    static CAROL_LIST_WORK: Lazy<ToDoList> =
        Lazy::new(|| ToDoList::new("work", vec!["meeting", "spreadsheet"]));
    static CAROL_LIST_HOME: Lazy<ToDoList> = Lazy::new(|| ToDoList::new("home", vec!["buy food"]));
    static CAROL_LIST_FRIENDS: Lazy<ToDoList> =
        Lazy::new(|| ToDoList::new("friends", vec!["buy present", "book restaurant"]));
    static EMMA: Lazy<ToDoListOwner> = Lazy::new(|| ToDoListOwner::new("emma"));

    static LISTS: Lazy<HashMap<User, HashMap<ListName, ToDoList>>> = Lazy::new(|| {
        HashMap::from([
            (
                CAROL.user(),
                HashMap::from([
                    (CAROL_LIST_WORK.list_name.clone(), CAROL_LIST_WORK.clone()),
                    (CAROL_LIST_HOME.list_name.clone(), CAROL_LIST_HOME.clone()),
                    (
                        CAROL_LIST_FRIENDS.list_name.clone(),
                        CAROL_LIST_FRIENDS.clone(),
                    ),
                ]),
            ),
            (EMMA.user(), HashMap::from([])),
        ])
    });

    static FETCHER: Lazy<ToDoListFetcherFromMap> =
        Lazy::new(|| ToDoListFetcherFromMap::new(LISTS.clone()));

    #[tokio::test]
    async fn new_users_have_no_todo_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        EMMA.can_not_see_any_todo_lists(&app).await;
    }

    #[tokio::test]
    async fn only_owners_can_see_all_their_todo_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        let carol_todo_lists = vec![
            CAROL_LIST_WORK.clone(),
            CAROL_LIST_HOME.clone(),
            CAROL_LIST_FRIENDS.clone(),
        ];

        CAROL.can_see_all_todo_lists(carol_todo_lists, &app).await;
    }
}
