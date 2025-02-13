use std::fs::File;
use std::io::{self, BufRead};

pub fn read_passwords(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut passwords = Vec::new();

    for line in reader.lines() {
        passwords.push(line?);
    }
    Ok(passwords)
}