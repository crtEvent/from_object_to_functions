#[cfg(test)]
mod tests {
    use crate::zettai::business::domain::{ListName, ToDoList, User};
    use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
    use crate::zettai::test::tooling::app_for_at::AppForAT;
    use crate::zettai::test::tooling::todolist_owner::ToDoListOwner;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static ANN: Lazy<ToDoListOwner> = Lazy::new(|| ToDoListOwner::new("ann"));
    static ANN_LIST: Lazy<ToDoList> = Lazy::new(|| ToDoList::new("diy", vec![]));

    static LISTS: Lazy<HashMap<User, HashMap<ListName, ToDoList>>> = Lazy::new(|| {
        HashMap::from([(
            ANN.user(),
            HashMap::from([(ANN_LIST.list_name.clone(), ANN_LIST.clone())]),
        )])
    });

    static FETCHER: Lazy<ToDoListFetcherFromMap> =
        Lazy::new(|| ToDoListFetcherFromMap::new(LISTS.clone()));

    #[tokio::test]
    async fn the_list_owner_can_add_new_items() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        ANN.can_add_item_to_item_list("paint the shelf", "diy", &app)
            .await;
        ANN.can_add_item_to_item_list("fix the gate", "diy", &app)
            .await;
        ANN.can_add_item_to_item_list("change the lock", "diy", &app)
            .await;
        ANN.can_see_the_item_list(
            &ToDoList::new(
                "diy",
                vec!["paint the shelf", "fix the gate", "change the lock"],
            ),
            &app,
        )
        .await;
    }
}
