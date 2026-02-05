use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lower")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    role: Role,
}

impl User {
    pub fn mock() -> Self {
        Self {
            first_name: "First Name".to_string(),
            last_name: "Last Name".to_string(),
            email: "test@test.com".to_string(),
            role: Role::Admin,
        }
    }

    pub fn write(user: &User, file_path: &Path) {
        let mut fh = File::create(&file_path).expect("Failed to create output file.");
        let s = serde_json::to_vec(user).expect("Failed to convert user to vec");
        fh.write_all(&s[..]).expect("Failed to write user to file.")
    }

    pub fn read(file_path: &Path) -> Self {
        let fh = File::open(file_path).expect("Failed to open file.");
        let mut reader = BufReader::new(fh);
        let user: User =
            serde_json::from_reader(&mut reader).expect("Failed to convert json to User.");

        user
    }
}
