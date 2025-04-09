#![allow(unused)]

fn build_char_at_pos(s: &str) -> impl Fn(usize) -> Option<char> {
    let s = s.to_string();
    move |i| s.chars().nth(i)
}

#[cfg(test)]
mod tests {
    use crate::exercises::ch03::prac3_3::build_char_at_pos;

    #[test]
    fn test_build_char_at_pos() {
        let my_char_at_pos = build_char_at_pos("rust");
        assert_eq!('r', my_char_at_pos(0).unwrap());
        assert_eq!('u', my_char_at_pos(1).unwrap());
        assert_eq!('s', my_char_at_pos(2).unwrap());
        assert_eq!('t', my_char_at_pos(3).unwrap());
    }
}