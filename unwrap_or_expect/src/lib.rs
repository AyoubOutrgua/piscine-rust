pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(url) => {
            match security_level {
                Security::Unknown => url.to_string(),
                Security::Message => url.to_string(),
                Security::Warning => url.to_string(),
                Security::NotFound => url.to_string(),
                // Security::UnexpectedUrl => panic!("{}",url.to_string()),
                Security::UnexpectedUrl => todo!(),
            }
        }
        Err(url) => {
            match security_level {
                Security::Unknown => todo!(),
                // Security::Unknown => panic!("{}",Err),
                Security::Message => panic!("ERROR: program stops"),
                Security::Warning => "WARNING: check the server".to_string(),
                Security::NotFound => format!("Not found: {}",url),
                // Security::UnexpectedUrl => Err,
                Security::UnexpectedUrl => todo!(),
            }
        }
    }
}
