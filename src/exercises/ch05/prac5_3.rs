#![allow(unused)]
// 연습 문제 5.3: 유니언 타입
// 큰따옴표 내부의 공백을 변경하지 않고, 나머지 모든 공백을 제거하자.

// 문자열을 처음부터 읽는다
// 문자를 하나씩 읽으면서 Vec에 추가한다
// 처음 상태는 OutQuotes 상태
// "가 나오면 InQuotes 상태
// InQuotes 상태에서 "를 만다면 OutQuotes 상태로 변경
// InQuotes 상태에서 \를 만나면 Escape 상태로 변경
// Escape 상태에서 "를 만나면 다시 InQuotes 상태로 변경
// 문자열을 끝 까지 다 읽었을 때 OutQuotes 상태이면 정상
// OutQuotes 상태에서 white_space가 나오면 지운다(구조체의 Vec에 추가하지 않는다)
// InQuotes 상태에서 white_space가 나오면 지우지 않는다(Vec에 추가한다)

use std::cmp::PartialEq;
use std::str::Chars;
use serde::de::Unexpected::Char;

struct JsonCompactor {
    parse_state: ParseState,
    chars: Vec<char>,
}

impl JsonCompactor {
    fn new() -> JsonCompactor {
        JsonCompactor {
            parse_state: ParseState::OutQuotes,
            chars: Vec::new(),
        }
    }

    fn compact(&mut self, c: char) {
        self.parse_state = ParseState::change_state(&self.parse_state, c);

        if self.parse_state == ParseState::OutQuotes && c == ' ' {
            return;
        }
        self.chars.push(c);
    }

    fn as_str(&self) -> String {
        self.chars.iter().collect::<String>()
    }
}

#[derive(PartialEq)]
enum ParseState {
    InQuotes,
    OutQuotes,
    Escaped
}

impl ParseState {
    fn change_state(current_state: &Self, c: char) -> Self {
        match current_state {
            ParseState::OutQuotes => {
                match c {
                    '"' => ParseState::InQuotes,
                    _ => ParseState::OutQuotes
                }
            },
            ParseState::InQuotes => {
                match c {
                    '"' => ParseState::OutQuotes,
                    '\\' => ParseState::Escaped,
                    _ => ParseState::InQuotes
                }
            },
            ParseState::Escaped => {
                match c {
                    '"' => ParseState::InQuotes,
                    _ => ParseState::Escaped
                }
            },
        }
    }
}

fn compact_json(json: &str) -> String {
    let json_compactor = json.chars()
        .fold(JsonCompactor::new(), |mut acc, c| {
            acc.compact(c);
            acc
        });

    json_compactor.as_str()
}

#[cfg(test)]
mod tests {
    use crate::exercises::ch05::prac5_3::compact_json;

    #[test]
    fn test() {
        let json_text = r#"{"my greetings" : "hello world! I\"How are you?!\" 3"}"#;
        let expected = r#"{"my greetings":"hello world! I\"How are you?!\" 3"}"#;

        assert_eq!(compact_json(json_text), expected);
    }
}