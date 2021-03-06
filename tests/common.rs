use std::fs::File;
use std::io::Write;

use ntest::assert_false;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub const PLAIN_FILE_CONTENT: &str = "some sensitive data";

pub const TEST_DIR: &str = "target/test";

pub fn setup(data_type: TestDataType) -> EnvironmentDetails {
    std::fs::create_dir_all(TEST_DIR).unwrap();

    return match data_type {
        TestDataType::RandomSingleFile => {
            let filename: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
            let tmp_file_path = format!("{}/{}", TEST_DIR, filename);

            prepare_file(&tmp_file_path);

            EnvironmentDetails::single(tmp_file_path)
        }
        TestDataType::MultipleFiles(files) => {
            let files = files
                .iter()
                .map(|file| format!("{}/{}", TEST_DIR, file))
                .collect::<Vec<String>>();
            for file in files.iter() {
                let path = std::path::Path::new(&file);
                let directory = path.parent().unwrap();
                std::fs::create_dir_all(directory).unwrap();

                prepare_file(&file);
            }

            EnvironmentDetails::multiple(files)
        }
    };
}

fn prepare_file(filepath: &str) {
    let mut tmp_file = File::create(&filepath).unwrap();

    tmp_file.write(PLAIN_FILE_CONTENT.as_bytes()).unwrap();
    tmp_file.sync_all().unwrap();
}

pub fn check_file_content(file: &String) {
    // file's content will become an invalid UTF-8 string
    assert_false!(std::fs::read_to_string(file).is_ok());
}

pub fn check_file_length(file: &String) {
    let shredded_file_length = std::fs::metadata(file).unwrap().len();
    // file's size hasn't changed
    assert_eq!(PLAIN_FILE_CONTENT.len() as u64, shredded_file_length);
}

pub fn cleanup(env_details: EnvironmentDetails) {
    match env_details {
        EnvironmentDetails::Single(filename) => {
            if std::path::Path::new(&filename).exists() {
                std::fs::remove_file(&filename).unwrap();
            }
        }
        EnvironmentDetails::Multiple(files) => files.iter().for_each(|file| {
            if std::path::Path::new(file).exists() {
                std::fs::remove_file(file).unwrap();
            }
        }),
    }
}

pub enum EnvironmentDetails {
    Single(String),
    Multiple(Vec<String>),
}

impl EnvironmentDetails {
    pub fn single(file: String) -> EnvironmentDetails {
        EnvironmentDetails::Single(file)
    }
    pub fn multiple(files: Vec<String>) -> EnvironmentDetails {
        EnvironmentDetails::Multiple(files)
    }
}

pub enum TestDataType {
    RandomSingleFile,
    MultipleFiles(Vec<String>),
}
