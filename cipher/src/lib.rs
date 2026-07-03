#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res = String::new();
    for c in original.chars() {
        match c {
            'a'..='z' => {
                let cipher = (b'a' + b'z' - c as u8) as char;
                res.push(cipher);
            },
            'A'..='Z' => {
                let cipher = (b'A' + b'Z' - c as u8) as char;
                res.push(cipher);
            }
            _ => {
                res.push(c)
            }
        }
    }
    if res == ciphered {
        return Ok(());
    }
    Err(CipherError { expected: res})
}