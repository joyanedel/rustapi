const HTTP_VERSION: &'static str = "1.1";

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
