#[cfg(test)]
mod tests {
    use crate::zettai::test::app_for_at::AppForAT;
    use crate::zettai::test::todolist_owner::ToDoListOwner;
    use std::collections::HashMap;
    use once_cell::sync::Lazy;
    use crate::zettai::business::domain::{ToDoList, User};

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
    static LISTS: Lazy<HashMap<User, Vec<ToDoList>>>= Lazy::new(||
        HashMap::from([
            (FRANK.user(), Vec::from([FRANK_LIST.clone()])),
            (BOB.user(), Vec::from([BOB_LIST.clone()])),
        ])
    );

    #[tokio::test]
    async fn list_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(LISTS.clone()).await;

        FRANK.can_see_the_list(&FRANK_LIST, &app).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(LISTS.clone()).await;

        BOB.can_see_the_list(&FRANK_LIST, &app).await;
    }
}