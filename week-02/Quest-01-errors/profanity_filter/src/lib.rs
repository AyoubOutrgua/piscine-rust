pub fn check_ms(message: &str) -> Result<&str, &str> {
  let mut result: Result<&str, &str> = Ok(message);
  if message.is_empty() || message.contains("stupid") {
    result = Err("ERROR: illegal");
    return result;
  } 
  result
}