pub mod zettai;

pub mod business {
    pub mod domain;
    pub mod todolist_fetcher;
    pub mod zettai_hub;
    pub mod domain_error;
}

pub mod response {
    pub mod add_new_item;
    pub mod dto;
    pub mod get_all_todo_lists;
    pub mod get_end_page;
    pub mod get_todo_list;
    pub mod create_new_todo_list;
    pub mod get_error_page;
}

mod test {
    mod tooling {
        pub(crate) mod app_for_at;
        pub(crate) mod todolist_owner;
        mod parser {
            pub(crate) mod get_all_todo_lists_parser;
            pub(crate) mod get_todo_list_parser;
        }
    }
    mod test_add_new_item;
    mod test_get_all_todo_lists;
    mod test_get_todo_list;
    mod test_create_new_todo_list;
}
