use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io;
use std::process::exit;

pub struct Shredder {
    path: String,
    is_recursively: bool,
    is_interactive: bool,
    verbosity: Verbosity,
}

impl Shredder {
    pub fn new(path: String, is_recursively: bool, is_interactive: bool, verbosity: Verbosity) -> Shredder {
        Shredder {
            path,
            is_recursively,
            is_interactive,
            verbosity
        }
    }

    pub fn run(&self) {
        match fs::metadata(&self.path) {
            Ok(metadata) => {
                if self.verbosity >= Verbosity::Average {
                    println!("Is directory: {}", metadata.is_dir());
                    println!("Is file: {}", metadata.is_file());
                    if self.verbosity == Verbosity::High {
                        println!("Is recursively: {}", self.is_recursively);
                        println!("Is interactive: {}", self.is_interactive);
                        println!("Verbosity: {:?}", &self.verbosity);
                    }
                }
            }
            Err(_) => {
                println!("Provided path is invalid!");
                exit(1);
            }
        }
        if self.verbosity > Verbosity::None {
            println!("Using input file: {}", &self.path);
        }

        if fs::metadata(&self.path).unwrap().is_file() {
            Shredder::shred_file(&self.path, self.is_interactive);
        } else if self.is_recursively {

        } else {
            println!("Target is a directory!");
            exit(1);
        }
    }

    fn shred_file(path: &String, is_interactive: bool) {
        match std::fs::canonicalize(path) {
            Ok(path) => {
                if is_interactive {
                    let file_length = path.metadata().unwrap().len();
                    let absolute_path = path.to_str().unwrap();
                    println!("Do you really want to shred '{}'? [Y/n]", absolute_path);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input.");
                    let input = input.trim();

                    if input.len() == 1 && input.to_lowercase().eq("y") {
                        match File::create(absolute_path) {
                            Ok(file) => {
                                println!("File's size: {}", file_length);
                                let mut buffer = BufWriter::new(&file);

                                let random_bytes: Vec<u8> = (0..file_length).map(|_| {
                                   rand::random::<u8>()
                                }).collect();
                                buffer.write(&random_bytes).unwrap();

                                buffer.flush().unwrap();
                                file.sync_all().unwrap();
                            }
                            Err(error) => {
                                println!("{}", error);
                            }
                        }

                    }
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}


#[derive(Debug, Eq)]
pub enum Verbosity {
    None,
    Low,
    Average,
    High
}

impl Verbosity {
    pub fn discriminant(&self) -> i8 {
        match self {
            Verbosity::None => 0,
            Verbosity::Low => 1,
            Verbosity::Average => 2,
            Verbosity::High => 3,
        }
    }
}

impl Ord for Verbosity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.discriminant().cmp(&other.discriminant())
    }
}

impl PartialOrd for Verbosity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.discriminant().cmp(&other.discriminant()))
    }
}

impl PartialEq for Verbosity {
    fn eq(&self, other: &Self) -> bool {
        self.discriminant() == other.discriminant()
    }
}
