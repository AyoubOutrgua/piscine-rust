use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        let short_flag = name.chars().next().unwrap();
        Self {
            short_hand: format!("-{}",short_flag),
            long_hand: format!("--{}",name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(func) = self.flags.get(input) {
            match func(argv[0], argv[1]) {
                Ok(result) => Ok(result),
                Err(err) => Err(err.to_string()),
            }
        } else {
            Err("invalid flag".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match a.parse::<f64>() {
        Ok(n1) => {
            match b.parse::<f64>() {
                Ok (n2) => {
                    let res = n1 / n2;
                    Ok(res.to_string())
                },
                Err(e2) => Err(e2)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match a.parse::<f64>() {
        Ok(n1) => {
            match b.parse::<f64>() {
                Ok (n2) => {
                    let res = n1 / n2;
                    Ok(res.to_string())
                },
                Err(e2) => Err(e2)
            }
        }
        Err(e) => Err(e),
    }
}