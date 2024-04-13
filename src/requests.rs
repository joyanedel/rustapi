use regex::Regex;

const HTTP_VERSION: &'static str = "1.1";
const HTTP_HEAD: &'static str = "(GET|POST|PUT|PATCH|DELETE|OPTIONS|HEAD) ([\\S])+ HTTP.*";

pub fn format_request(method: String, path: String) -> Result<String, ()> {
    let method = verify_method(&method)?;
    let head = format!("{} {} HTTP/{}\r\n", method, path, HTTP_VERSION);

    Ok(head)
}

fn verify_method(method: &str) -> Result<&str, ()> {
    match method.to_uppercase().as_str() {
        "GET" => Ok("GET"),
        "POST" => Ok("POST"),
        "PUT" => Ok("PUT"),
        "DELETE" => Ok("DELETE"),
        "OPTIONS" => Ok("OPTIONS"),
        _ => Err(()),
    }
}

pub fn get_http_head<'a>(lines: &Vec<&'a str>) -> Option<&'a str> {
    let http_head_regex = Regex::new(HTTP_HEAD).unwrap();
    for &line in lines.iter() {
        if !http_head_regex.is_match(line.trim()) {
            continue;
        };

        return Some(&line);
    }

    None
}
