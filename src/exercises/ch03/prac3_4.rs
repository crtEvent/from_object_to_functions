#![allow(unused)]

use std::collections::HashMap;

struct StringTag {
    text: String,
}

trait TagExtension {
    fn tag(self, value: &str) -> (String, StringTag);
}

impl TagExtension for &str {
    fn tag(self, value: &str) -> (String, StringTag) {
        (self.to_string(), StringTag { text: value.to_string() })
    }
}

fn render_template(template: &str, data: HashMap<String, StringTag>) -> String {
    data.iter().fold(template.to_string(), |acc, (key, value)| {
        acc.replace(&format!("{{{}}}", key), &value.text)
    })
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use indoc::indoc;
    use crate::exercises::ch03::prac3_4::{render_template, StringTag, TagExtension};

    #[test]
    fn test_template() {
        let template = indoc! {
        "Happy Birthday {name} {surname}
        from {sender}."
        };

        let data: HashMap<String, StringTag> = HashMap::from([
            "name".tag("Uberto"),
            "surname".tag("Barbini"),
            "sender".tag("PragProg"),
        ]);

        let actual = render_template(template, data);
        let expected = indoc! {
            "Happy Birthday Uberto Barbini
            from PragProg."
        };

        assert_eq!(actual, expected);
    }
}