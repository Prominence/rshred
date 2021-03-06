extern crate clap;

use clap::{crate_authors, crate_name, crate_version, App, Arg};

use rshred::{ShredConfiguration, Shredder, Verbosity};
use std::process::exit;
use std::str::FromStr;

fn main() {
    let params = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("TODO")
        .arg(
            Arg::with_name("PATH")
                .help("Sets the path of file or directory to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .help("Do shredding operations recursively"),
        )
        .arg(
            Arg::with_name("i")
                .short("i")
                .long("interactive")
                .help("Enables interactive mode"),
        )
        .arg(
            Arg::with_name("k")
                .short("k")
                .long("keep")
                .help("Don't delete files after shredding"),
        )
        .arg(
            Arg::with_name("n")
                .short("n")
                .takes_value(true)
                .help("How many times the file must be overridden"),
        )
        .get_matches();

    let verbosity = match params.occurrences_of("v") {
        1 => Verbosity::Low,
        2 => Verbosity::Average,
        3 => Verbosity::High,
        _ => Verbosity::None,
    };

    let is_recursively = params.is_present("r");
    let is_interactive = params.is_present("i");
    let keep_files = params.is_present("k");
    let iterations_count = if params.is_present("n") {
        let value_option = params.value_of("n");
        match value_option {
            None => {
                println!("No argument passed to the 'n' option!");
                exit(1);
            }
            Some(value) => match u8::from_str(value) {
                Ok(number) => number,
                Err(error) => {
                    println!("{}", error.to_string());
                    exit(1);
                }
            },
        }
    } else {
        3
    };

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let path = params.value_of("PATH").unwrap();

    Shredder::with_options(
        ShredConfiguration::new(path)
            .set_is_interactive(is_interactive)
            .set_is_recursive(is_recursively)
            .set_keep_files(keep_files)
            .set_verbosity(verbosity)
            .set_rewrite_iterations(iterations_count)
            .build(),
    )
    .run();
}
