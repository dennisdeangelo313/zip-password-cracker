use clap::{App, Arg};
use rayon::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use zip::read::ZipArchive;

fn main() {
    let matches = App::new("ZIP Password Cracker")
        .version("1.0")
        .author("dennisdeangelo313")
        .about("Cracks password protected zip files")
        .arg(Arg::new("zipfile")
            .about("The path to the zip file")
            .required(true)
            .index(1))
        .arg(Arg::new("passwords")
            .about("The path to the password list")
            .required(true)
            .index(2))
        .get_matches();

    let zipfile = matches.value_of("zipfile").unwrap();
    let passwords_file = matches.value_of("passwords").unwrap();

    let passwords = read_passwords(passwords_file);
    let result = crack_zip(zipfile, &passwords);

    match result {
        Some(password) => println!("Password found: {}", password),
        None => println!("Password not found"),
    }
}

fn read_passwords(file_path: &str) -> Vec<String> {
    let mut passwords = Vec::new();
    let file = File::open(file_path).expect("Unable to open password file");
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(password) = line {
            passwords.push(password);
        }
    }
    passwords
}

fn crack_zip(zipfile: &str, passwords: &[String]) -> Option<String> {
    let file = File::open(zipfile).expect("Unable to open zip file");
    let mut archive = ZipArchive::new(file).expect("Unable to read zip archive");

    passwords.par_iter().find_map_any(|password| {
        if let Ok(mut file) = archive.by_name("protected_file.txt") {
            if file.set_password(password).is_ok() {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    return Some(password.clone());
                }
            }
        }
        None
    })
}