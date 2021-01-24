extern crate clap;
use clap::{App, Arg};
use std::fs;
use std::process::exit;

fn main() {
    let params = App::new("shred")
        .version("0.1.0")
        .author("Alexey Zinchenko <alexey.zinchenko@protonmail.com>")
        .about("TODO")
        .arg(Arg::with_name("PATH")
            .help("Sets the path of file or directory to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("r")
            .short("r")
            .help("Do shredding operations recursively"))
        .arg(Arg::with_name("i")
            .short("i")
            .long("interactive")
            .help("Enables interactive mode")
        )
        .get_matches();

    let verbosity = match params.occurrences_of("v") {
        1 => Verbosity::Low,
        2 => Verbosity::Average,
        3 => Verbosity::High,
        _ => Verbosity::None
    };

    let is_recursively = params.is_present("r");
    let is_interactive = params.is_present("i");

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let path = params.value_of("PATH").unwrap();
    match fs::metadata(path) {
        Ok(metadata) => {
            println!("Exists: OK");
            println!("Is directory: {}", metadata.is_dir());
            println!("Is file: {}", metadata.is_file());
            println!("Is recursively: {}", is_recursively);
            println!("Is interactive: {}", is_interactive);
            println!("Verbosity: {:?}", verbosity);
        }
        Err(_) => {
            println!("Provided path is invalid!");
            exit(1);
        }
    }
    println!("Using input file: {}", path);
}

#[derive(Debug)]
enum Verbosity {
    None,
    Low,
    Average,
    High
}