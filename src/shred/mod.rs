use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write, Seek, SeekFrom};
use std::io;
use std::process::exit;

pub struct Shredder {
    options: ShredOptions,
}

impl Shredder {
    pub fn with_options(options: ShredOptions) -> Shredder {
        Shredder {
            options
        }
    }

    pub fn run(&self) {
        let verbosity = &self.options.verbosity;
        let metadata_result = fs::metadata(&self.options.path);
        match &metadata_result {
            Ok(metadata) => {
                if *verbosity >= Verbosity::Average {
                    println!("Is directory: {}", metadata.is_dir());
                    println!("Is file: {}", metadata.is_file());
                    if *verbosity == Verbosity::High {
                        println!("Is recursively: {}", self.options.is_recursive);
                        println!("Is interactive: {}", self.options.is_interactive);
                        println!("Verbosity: {:?}", &self.options.verbosity);
                    }
                }
            }
            Err(_) => {
                println!("No such file or directory.");
                exit(1);
            }
        }
        if *verbosity > Verbosity::None {
            println!("Using input file: {}", &self.options.path);
        }

        if metadata_result.unwrap().is_file() {
            Shredder::shred_file(&self.options);
        } else if self.options.is_recursive {

        } else {
            println!("Target is a directory!");
            exit(1);
        }
    }

    fn shred_file(options: &ShredOptions) {
        match std::fs::canonicalize(&options.path) {
            Ok(path) => {
                let file_length = path.metadata().unwrap().len();
                let absolute_path = path.to_str().unwrap();
                if options.is_interactive {
                    print!("Do you really want to shred '{}'? [Y/n] ", absolute_path);
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input.");
                    let input = input.trim();

                    if input.len() != 1 || !input.to_lowercase().eq("y") {
                        return;
                    }
                }

                match File::create(absolute_path) {
                    Ok(file) => {
                        println!("File's size: {}", file_length);
                        let mut buffer = BufWriter::new(&file);

                        for _ in 0..options.rewrite_iterations {
                            let random_bytes: Vec<u8> = (0..file_length).map(|_| {
                                rand::random::<u8>()
                            }).collect();
                            buffer.write(&random_bytes).unwrap();

                            buffer.flush().unwrap();
                            file.sync_all().unwrap();
                            buffer.seek(SeekFrom::Start(0)).unwrap();
                        }

                        if !options.keep_files {
                            fs::remove_file(absolute_path).unwrap();
                        }
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}

pub struct ShredOptions {
    verbosity: Verbosity,
    is_recursive: bool,
    is_interactive: bool,
    rewrite_iterations: u8,
    keep_files: bool,
    path: String,
}

impl ShredOptions {
    pub fn new(path: String) -> ShredOptions {
        ShredOptions {
            path,
            is_interactive: true,
            is_recursive: false,
            rewrite_iterations: 3,
            keep_files: false,
            verbosity: Verbosity::None,
        }
    }

    pub fn set_verbosity(mut self, verbosity: Verbosity) -> ShredOptions {
        self.verbosity = verbosity;
        self
    }

    pub fn set_is_recursive(mut self, is_recursive: bool) -> ShredOptions {
        self.is_recursive = is_recursive;
        self
    }

    pub fn set_is_interactive(mut self, is_interactive: bool) -> ShredOptions {
        self.is_interactive = is_interactive;
        self
    }

    pub fn set_keep_files(mut self, is_keep_files: bool) -> ShredOptions {
        self.keep_files = is_keep_files;
        self
    }

    pub fn set_rewrite_iterations(mut self, count: u8) -> ShredOptions {
        self.rewrite_iterations = count;
        self
    }

    pub fn build(self) -> ShredOptions {
        self
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
