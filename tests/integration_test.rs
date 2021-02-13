extern crate rshred;

use ntest::assert_false;
use ntest::timeout;

mod common;

#[test]
#[timeout(10000)]
fn shredding_without_file_deletion() {
    let env_details = common::setup(common::TestDataType::RandomSingleFile);
    match &env_details {
        common::EnvironmentDetails::Single(filename) => {
            let options = rshred::ShredOptions::new(filename.to_owned())
                .set_verbosity(rshred::Verbosity::None)
                .set_is_interactive(false)
                .set_keep_files(true)
                .build();
            rshred::Shredder::with_options(options).run();

            common::check_file_content(filename);
            common::check_file_length(filename);
        }
        common::EnvironmentDetails::Multiple(_) => {
            unreachable!()
        }
    }

    common::cleanup(env_details);
}

#[test]
#[timeout(10000)]
fn shredding_with_file_deletion() {
    let env_details = common::setup(common::TestDataType::RandomSingleFile);
    match &env_details {
        common::EnvironmentDetails::Single(filename) => {
            let options = rshred::ShredOptions::new(filename.to_owned())
                .set_verbosity(rshred::Verbosity::None)
                .set_is_interactive(false)
                .set_keep_files(false)
                .build();
            rshred::Shredder::with_options(options).run();

            let shredded_file_exists = std::path::Path::new(filename).exists();

            assert_false!(shredded_file_exists);
        }
        common::EnvironmentDetails::Multiple(_) => {
            unreachable!()
        }
    }

    common::cleanup(env_details);
}

#[test]
#[timeout(10000)]
fn shredding_directory_recursively() {
    let env_details = common::setup(common::TestDataType::MultipleFiles(
        vec![
            String::from("test3/subdir1/1.txt"),
            String::from("test3/subdir1/2.txt"),
            String::from("test3/subdir1/3.txt"),
            String::from("test3/subdir1/subdir11/1.txt"),
            String::from("test3/subdir1/subdir11/2.txt"),
            String::from("test3/subdir2/subdir1/111.txt"),
            String::from("test3/subdir3/1231.txt"),
            String::from("test3/subdir3/1222.txt"),
            String::from("test3/subdir3/1286.txt"),
            String::from("test3/subdir3/1286/anotherdir/abs.txt"),
            String::from("test3/subdir3/1286/anotherdir/abc.txt"),
        ]
    ));

    match &env_details {
        common::EnvironmentDetails::Single(_) => {
            unreachable!()
        }
        common::EnvironmentDetails::Multiple(files) => {
            let options = rshred::ShredOptions::new(format!("{}/{}", common::TEST_DIR, "test3"))
                .set_verbosity(rshred::Verbosity::None)
                .set_is_interactive(false)
                .set_is_recursive(true)
                .set_keep_files(true)
                .build();
            rshred::Shredder::with_options(options).run();

            for file in files {
                common::check_file_content(file);
                common::check_file_length(file);
            }
        }
    }

    common::cleanup(env_details);
}
