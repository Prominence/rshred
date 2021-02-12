use std::fs::File;
use std::io::Write;

pub const PLAIN_FILE_CONTENT: &str = "some sensitive data";

pub const TEST_FILE: &str = "target/test/test.txt";
const TEST_DIR: &str = "target/test";

pub fn setup() {
    std::fs::create_dir_all(TEST_DIR).unwrap();
    let mut file = File::create("target/test/test.txt").unwrap();
    file.write(PLAIN_FILE_CONTENT.as_bytes()).unwrap();
}

pub fn cleanup() {
    std::fs::remove_file(TEST_FILE).unwrap();
    std::fs::remove_dir(TEST_DIR).unwrap();
}