use regex::Regex;

use crate::structs::{HttpMethod, HttpRequestLine};
use std::str::FromStr;

const HTTP_HEAD: &'static str =
    r"(?<method>(GET|POST|PUT|PATCH|DELETE|OPTIONS|HEAD)) (?<path>[\S]+) HTTP.*";

pub fn get_http_request_line(lines: &Vec<&str>) -> Option<HttpRequestLine> {
    let http_head_regex = Regex::new(HTTP_HEAD).unwrap();
    for &line in lines.iter() {
        let Some(captures) = http_head_regex.captures(line.trim()) else {
            continue;
        };

        return Some(HttpRequestLine {
            method: HttpMethod::from_str(&captures["method"]).unwrap(),
            path: captures["path"].to_string(),
        });
    }

    None
}
