use regex::Regex;

use crate::structs::{HttpMethod, HttpRequest, HttpRequestLine};
use std::{collections::HashMap, str::FromStr};

const HTTP_HEAD: &'static str =
    r"(?<method>(GET|POST|PUT|PATCH|DELETE|OPTIONS|HEAD)) (?<path>[\S]+) HTTP.*";

pub fn parse_http_request(lines: &Vec<&str>) -> Result<HttpRequest, ()> {
    let request_line = get_http_request_line(&lines).ok_or(())?;
    let request_headers = get_http_request_headers(&lines);

    Ok(HttpRequest {
        method: request_line.method,
        path: request_line.path,
        headers: request_headers,
    })
}

fn get_http_request_line(lines: &Vec<&str>) -> Option<HttpRequestLine> {
    let http_request_line_regex = Regex::new(HTTP_HEAD).unwrap();
    let http_request_line = lines.first()?;

    if let Some(captures) = http_request_line_regex.captures(http_request_line) {
        let method = HttpMethod::from_str(&captures["method"]).unwrap();
        let path = captures["path"].to_string();
        Some(HttpRequestLine {
            method: method,
            path: path,
        })
    } else {
        None
    }
}

fn get_http_request_headers(lines: &Vec<&str>) -> HashMap<String, String> {
    lines.iter().skip(1).fold(HashMap::new(), |mut acc, &elem| {
        if elem.trim().is_empty() {
            return acc;
        }

        let header = elem.split_once(": ");
        if header == None {
            return acc;
        }
        let (header_name, header_value) = header.unwrap();
        acc.insert(header_name.to_string(), header_value.trim().to_string());

        acc
    })
}
