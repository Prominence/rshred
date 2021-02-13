extern crate rshred;

use ntest::timeout;
use ntest::assert_false;

mod common;

#[test]
#[timeout(1000)]
fn shredding_without_file_deletion() {
    let env_details = common::setup();

    let initial_file_length = std::fs::metadata(&env_details.test_file_path).unwrap().len();

    let options = rshred::ShredOptions::new(env_details.test_file_path.to_string())
        .set_verbosity(rshred::Verbosity::None)
        .set_is_interactive(false)
        .set_keep_files(true)
        .build();
    rshred::Shredder::with_options(options).run();

    let shredded_file_length = std::fs::metadata(&env_details.test_file_path).unwrap().len();
    // file's size hasn't changed
    assert_eq!(initial_file_length, shredded_file_length);

    // file's content will become an invalid UTF-8 string
    assert_false!(std::fs::read_to_string(&env_details.test_file_path).is_ok());

    common::cleanup(env_details);
}

#[test]
#[timeout(1000)]
fn shredding_with_file_deletion() {
    let env_details = common::setup();

    let options = rshred::ShredOptions::new(env_details.test_file_path.to_string())
        .set_verbosity(rshred::Verbosity::None)
        .set_is_interactive(false)
        .set_keep_files(false)
        .build();
    rshred::Shredder::with_options(options).run();

    let shredded_file_exists = std::path::Path::new(&env_details.test_file_path).exists();

    assert_false!(shredded_file_exists);

    common::cleanup(env_details);
}