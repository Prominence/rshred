use std::fs::File;
use std::io::Write;
use rand::Rng;
use rand::distributions::Alphanumeric;

pub const PLAIN_FILE_CONTENT: &str = "some sensitive data";

const TEST_DIR: &str = "target/test";

pub fn setup() -> EnvironmentDetails {
    std::fs::create_dir_all(TEST_DIR).unwrap();

    let filename: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let tmp_file_path = format!("{}/{}", TEST_DIR, filename);
    let mut tmp_file = File::create(&tmp_file_path).unwrap();

    tmp_file.write(PLAIN_FILE_CONTENT.as_bytes()).unwrap();
    tmp_file.sync_all().unwrap();

    println!("{}", filename);

    EnvironmentDetails::new(&tmp_file_path)
}

pub fn cleanup(env_details: EnvironmentDetails) {
    if std::path::Path::new(&env_details.test_file_path).exists() {
        std::fs::remove_file(&env_details.test_file_path).unwrap();
    }
}

pub struct EnvironmentDetails {
    pub test_file_path: String,
}

impl EnvironmentDetails {
    pub fn new(filepath: &str) -> EnvironmentDetails {
        EnvironmentDetails {
            test_file_path: filepath.to_string(),
        }
    }
}