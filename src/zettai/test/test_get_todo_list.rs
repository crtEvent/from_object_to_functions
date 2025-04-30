#[cfg(test)]
mod tests {
    use crate::zettai::business::domain::{ListName, ToDoList, User};
    use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
    use crate::zettai::test::tooling::app_for_at::AppForAT;
    use crate::zettai::test::tooling::todolist_owner::ToDoListOwner;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static FRANK: Lazy<ToDoListOwner> = Lazy::new(|| ToDoListOwner::new("frank"));
    static FRANK_LIST: Lazy<ToDoList> =
        Lazy::new(|| ToDoList::new("shopping", vec!["carrots", "apples", "milk"]));
    static BOB: Lazy<ToDoListOwner> = Lazy::new(|| ToDoListOwner::new("bob"));
    static BOB_LIST: Lazy<ToDoList> =
        Lazy::new(|| ToDoList::new("gardening", vec!["fix the fence", "mowing the lawn"]));

    static LISTS: Lazy<HashMap<User, HashMap<ListName, ToDoList>>> = Lazy::new(|| {
        HashMap::from([
            (
                FRANK.user(),
                HashMap::from([(FRANK_LIST.list_name.clone(), FRANK_LIST.clone())]),
            ),
            (
                BOB.user(),
                HashMap::from([(BOB_LIST.list_name.clone(), BOB_LIST.clone())]),
            ),
        ])
    });

    static FETCHER: Lazy<ToDoListFetcherFromMap> =
        Lazy::new(|| ToDoListFetcherFromMap::new(LISTS.clone()));

    #[tokio::test]
    async fn list_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        FRANK.can_see_the_todo_list(&FRANK_LIST, &app).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        BOB.can_see_the_todo_list(&FRANK_LIST, &app).await;
    }
}
