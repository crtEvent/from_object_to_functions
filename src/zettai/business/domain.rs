use once_cell::sync::Lazy;
use regex::Regex;

const URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9-]+$").unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToDoList {
    pub list_name: ListName,
    pub items: Vec<ToDoItem>,
}
impl ToDoList {
    pub fn new(list_name: &str, items: Vec<&str>) -> Self {
        ToDoList {
            list_name: ListName { name: list_name.to_string() },
            items: items.into_iter()
                .map(|item|
                    ToDoItem { description: item.to_string() }
                )
                .collect()
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ListName {
    pub name: String,
}

impl ListName {
    fn from_trusted(name: &str) -> Self {
        ListName { name: name.to_string() }
    }
    fn from_untrusted(name: &str) -> Option<Self> {
        if URL_REGEX.is_match(name) && (1..=40).contains(&name.len()) {
            Some(ListName { name: name.to_string() })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToDoItem {
    pub description: String,
}

// #[derive(Debug, Clone, Copy)]
// pub enum ToDoStatus {
//     Todo,
//     InProgress,
//     Done,
//     Blocked,
// }

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use rand::{rng, Rng};
    use rand::rngs::ThreadRng;
    use rand::seq::IndexedRandom;
    use super::*;

    const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const DIGITS: &str = "0123456789";
    const VALID_CHARSET: Lazy<String> = Lazy::new(|| {
        format!("{}{}{}", UPPER_CASE, LOWER_CASE, DIGITS)
    });
    const INVALID_CHARSET: Lazy<String> = Lazy::new(|| {
        " !@#$%^&*()_+={}[]|:;'<>,./?".to_string()
    });

    #[test]
    fn valid_names_are_alphanumeric_hyphen_between_3_and_40_chars_length() {
        for _ in 0..100 {
            let random_name: String = random_string_generator(&VALID_CHARSET, 3, 40);
            assert_eq!(
                ListName::from_untrusted(&random_name),
                Some(ListName::from_trusted(&random_name))
            );
        }
    }
    #[test]
    fn name_cannot_be_empty() {
        assert_eq!(ListName::from_untrusted(""), None);
    }
    #[test]
    fn names_longer_than_40_chars_are_not_valid() {
        for _ in 0..100 {
            let random_name: String = random_string_generator(&VALID_CHARSET, 41, 200);
            assert_eq!(ListName::from_untrusted(&random_name), None);
        }
    }
    #[test]
    fn invalid_chars_are_not_allowed_in_the_name() {
        for _ in 0..100 {
            let random_name: String = substitute_random_char(
                &random_string_generator(&VALID_CHARSET, 3, 40),
                &INVALID_CHARSET
            );
            assert_eq!(
                ListName::from_untrusted(&random_name),
                None
            );
        }
    }

    fn random_string_generator(char_set: &String, min_len: u8, max_len: u8) -> String {
        let mut rng: ThreadRng = rng();
        let string_length = if max_len > min_len {
            rng.random_range(min_len..=max_len)
        } else {
            min_len
        };

        (0..string_length)
            .map(|_| {
                *char_set
                    .as_bytes()
                    .choose(&mut rng)
                    .unwrap() as char
            })
            .collect()
    }

    fn substitute_random_char(original: &String, substitute: &String) -> String {
        if original.is_empty() || substitute.is_empty() {
            return original.to_string();
        }

        let mut original_chars: Vec<char> = original.chars().collect();
        let mut rng: ThreadRng = rng();
        let random_position = rng.random_range(0..original_chars.len());

        let replacement_char = substitute
            .chars()
            .nth(rng.random_range(0..substitute.len()))
            .unwrap();

        original_chars[random_position] = replacement_char;
        original_chars.into_iter().collect()
    }
}