pub mod zettai;

pub mod business {
    pub mod domain;
    pub mod todolist_fetcher;
    pub mod zettai_hub;
}

pub mod response {
    pub mod add_new_item;
    pub mod dto;
    pub mod get_end_page;
    pub mod get_item_list_page;
}

mod test;
