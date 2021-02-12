use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::io;
use std::process::exit;

use walkdir::WalkDir;

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
        let metadata_result = fs::metadata(&self.options.raw_path);
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
            println!("Using input file: {}", &self.options.raw_path);
        }

        if metadata_result.unwrap().is_file() {
            Shredder::shred_file(&self.options, &self.options.raw_path);
        } else if self.options.is_recursive {
            Shredder::shred_dir(&self.options, &self.options.raw_path);
        } else {
            println!("Target is a directory!");
            exit(1);
        }
    }

    fn shred_file(options: &ShredOptions, path: &str) {
        if options.verbosity > Verbosity::Low {
            println!("Trying to shred {}", path);
        }
        match std::fs::canonicalize(path) {
            Ok(path) => {
                let file_length = path.metadata().unwrap().len();
                let absolute_path = path.to_str().unwrap();
                if options.is_interactive {
                    if !Shredder::user_prompt(absolute_path) {
                        return;
                    }
                }

                match File::create(absolute_path) {
                    Ok(file) => {
                        if options.verbosity > Verbosity::Low {
                            println!("File's size: {}", file_length);
                        }
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

    fn shred_dir(options: &ShredOptions, dir: &str) {
        let mut files_count = 0;
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.metadata().unwrap().is_file() {
                Shredder::shred_file(options, entry.path().to_str().unwrap());
                files_count = files_count + 1;
            }
        }
        if options.verbosity != Verbosity::None {
            println!("Processed {} files.", files_count);
        }
    }

    fn user_prompt(path: &str) -> bool {
        print!("Do you really want to shred '{}'? [Y/n] ", path);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input = input.trim();

        if input.len() != 1 || !input.to_lowercase().eq("y") {
            return false;
        }
        true
    }
}

pub struct ShredOptions {
    verbosity: Verbosity,
    is_recursive: bool,
    is_interactive: bool,
    rewrite_iterations: u8,
    keep_files: bool,
    raw_path: String,
}

impl ShredOptions {
    pub fn new(path: String) -> ShredOptions {
        ShredOptions {
            raw_path: path,
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
    High,
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

#[cfg(test)]
mod tests {
    use crate::Verbosity;

    #[test]
    fn verbosity_check() {
        assert_eq!(Verbosity::None, Verbosity::None);
        assert!(Verbosity::None < Verbosity::Low, true);
        assert!(Verbosity::Low < Verbosity::Average, true);
        assert!(Verbosity::Average < Verbosity::High, true);
        assert!(Verbosity::None < Verbosity::High, true);
    }
}