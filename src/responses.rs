const HTTP_VERSION: &str = "1.1";

pub fn format_response(body: String, status_code: u16) -> Result<String, ()> {
    verify_status_code(status_code)?;
    Ok(format!(
        "HTTP/{} {} OK\r\n\r\n{}\r\n",
        HTTP_VERSION, status_code, body
    ))
}

fn verify_status_code(status_code: u16) -> Result<(), ()> {
    match status_code {
        100..=599 => Ok(()),
        _ => Err(()),
    }
}
