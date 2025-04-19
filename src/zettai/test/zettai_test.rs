#[cfg(test)]
mod tests {
    use crate::zettai::test::app_for_at::AppForAT;
    use crate::zettai::test::todolist_owner::ToDoListOwner;
    use std::collections::HashMap;
    use once_cell::sync::Lazy;
    use crate::zettai::business::domain::{ListName, ToDoList, User};
    use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;

    static FRANK: Lazy<ToDoListOwner> = Lazy::new(||
        ToDoListOwner::new("frank")
    );
    static FRANK_LIST: Lazy<ToDoList> = Lazy::new(||
        ToDoList::new(
            "shopping",
            vec!("carrots", "apples", "milk")
        )
    );
    static BOB: Lazy<ToDoListOwner> = Lazy::new(||
        ToDoListOwner::new("bob")
    );
    static BOB_LIST: Lazy<ToDoList> = Lazy::new(||
        ToDoList::new(
    "gardening",
    vec!("fix the fence", "mowing the lawn",)
        )
    );
    static ANN: Lazy<ToDoListOwner> = Lazy::new(||
        ToDoListOwner::new("ann")
    );
    static ANN_LIST: Lazy<ToDoList> = Lazy::new(||
        ToDoList::new(
            "diy",
            vec!()
        )
    );
    static LISTS: Lazy<HashMap<User, HashMap<ListName, ToDoList>>>= Lazy::new(||
        HashMap::from([
            (FRANK.user(), HashMap::from([(FRANK_LIST.list_name.clone(), FRANK_LIST.clone())])),
            (BOB.user(), HashMap::from([(BOB_LIST.list_name.clone(), BOB_LIST.clone())])),
            (ANN.user(), HashMap::from([(ANN_LIST.list_name.clone(), ANN_LIST.clone())])),
        ])
    );

    static FETCHER: Lazy<ToDoListFetcherFromMap> = Lazy::new(||
        ToDoListFetcherFromMap::new(LISTS.clone())
    );

    #[tokio::test]
    async fn list_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        FRANK.can_see_the_list(&FRANK_LIST, &app).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        BOB.can_see_the_list(&FRANK_LIST, &app).await;
    }

    #[tokio::test]
    async fn the_list_owner_can_add_new_items() {
        let app = AppForAT {};
        app.start_the_application(FETCHER.clone()).await;

        ANN.can_add_item_to_list("paint the shelf", "diy", &app).await;
        ANN.can_add_item_to_list("fix the gate", "diy", &app).await;
        ANN.can_add_item_to_list("change the lock", "diy", &app).await;
        ANN.can_see_the_list(
            &ToDoList::new(
                "diy",
                vec!("paint the shelf", "fix the gate", "change the lock")),
            &app
        ).await;
    }
}