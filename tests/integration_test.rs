extern crate rshred;

use ntest::timeout;
use ntest::assert_false;

mod common;

#[test]
#[timeout(1000)]
fn test_basic_shredding() {
    common::setup();

    let initial_file_length = std::fs::metadata(common::TEST_FILE).unwrap().len();

    let options = rshred::ShredOptions::new(common::TEST_FILE.to_string())
        .set_verbosity(rshred::Verbosity::None)
        .set_is_interactive(false)
        .set_keep_files(true)
        .build();
    rshred::Shredder::with_options(options).run();

    let shredded_file_length = std::fs::metadata(common::TEST_FILE).unwrap().len();
    // file's size hasn't changed
    assert_eq!(initial_file_length, shredded_file_length);

    // file's content will become an invalid UTF-8 string
    assert_false!(std::fs::read_to_string(common::TEST_FILE).is_ok());

    common::cleanup();
}