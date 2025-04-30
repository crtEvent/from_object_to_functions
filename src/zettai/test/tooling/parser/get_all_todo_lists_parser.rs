use crate::zettai::business::domain::ListName;
use regex::Regex;

pub(crate) fn parse_response(html: &str) -> Vec<ListName> {
    extract_list_names(html)
}

fn extract_list_names(html: &str) -> Vec<ListName> {
    let name_td_regex = Regex::new("<td>(.*?)<").unwrap();
    name_td_regex
        .captures_iter(html)
        .map(|cap| ListName::from_trusted(&*cap[1].to_string()))
        .collect()
}
