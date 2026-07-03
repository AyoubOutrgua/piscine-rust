use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Local::now();
        let date_now: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            form_values: (field_name.to_string(), field_value),
            date: date_now,
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }
        let mut has_letter = false;
        let mut has_digit = false;
        let mut has_symbol = false;
        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.clone(), "Password should be at least 8 characters long"));
        }
        for c in self.password.chars() {
            if c.is_ascii_alphabetic() {
                has_letter = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            } else {
                has_symbol = true;
            }
        }
        if !has_digit || !has_letter || !has_symbol {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        }
        Ok(())
    }
}