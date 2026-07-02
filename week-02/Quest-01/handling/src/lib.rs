use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(path)
    .unwrap();
    write!(file, "{}",content).unwrap();
}
