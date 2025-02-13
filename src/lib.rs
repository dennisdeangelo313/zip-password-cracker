pub mod zip_cracker {
    use zip::read::ZipArchive;
    use std::fs::File;
    use std::io::{Read, Write};

    pub fn crack_zip(zipfile: &str, passwords: &[String]) -> Option<String> {
        let file = File::open(zipfile).expect("Unable to open zip file");
        let mut archive = ZipArchive::new(file).expect("Unable to read zip archive");

        for password in passwords {
            if let Ok(mut file) = archive.by_name("protected_file.txt") {
                if file.set_password(password).is_ok() {
                    let mut buffer = Vec::new();
                    if file.read_to_end(&mut buffer).is_ok() {
                        return Some(password.clone());
                    }
                }
            }
        }
        None
    }
}