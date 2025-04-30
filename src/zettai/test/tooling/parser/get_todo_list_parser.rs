use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, ToDoStatus};
use chrono::NaiveDate;
use regex::Regex;

pub(crate) fn parse_response(html: &str) -> ToDoList {
    let list_name = extract_list_name(html);
    let items = extract_todo_items(html);

    ToDoList {
        list_name: ListName { name: list_name },
        items,
    }
}

fn extract_list_name(html: &str) -> String {
    let name_regex = Regex::new("<h2>(.*?)<").unwrap();
    name_regex
        .captures(html)
        .map(|cap| cap[1].to_string())
        .unwrap_or_default()
}

fn extract_todo_items(html: &str) -> Vec<ToDoItem> {
    let items_td_regex = Regex::new("<td>(.*?)<").unwrap();
    let mut caps_iter = items_td_regex.captures_iter(html);

    let mut items = Vec::new();

    while let (Some(cap1), Some(cap2), Some(cap3)) =
        (caps_iter.next(), caps_iter.next(), caps_iter.next())
    {
        items.push(ToDoItem {
            description: cap1[1].to_string(),
            due_date: NaiveDate::parse_from_str(&*cap2[1].to_string(), "%Y-%m-%d").unwrap(),
            state: ToDoStatus::from_str(&*cap3[1].to_string()),
        });
    }

    items
}
