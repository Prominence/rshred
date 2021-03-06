use std::cmp::Ordering;
use std::fs;
use std::fs::{File, Metadata};
use std::io;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::process::exit;

use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::{Display, Formatter, Result};
use std::path::Path;
use walkdir::WalkDir;

const BATCH_SIZE: usize = 8192;

pub struct Shredder<'a> {
    configuration: ShredConfiguration<'a>,
}

impl<'a> Shredder<'a> {
    pub fn with_options(configuration: ShredConfiguration<'a>) -> Shredder<'a> {
        Shredder { configuration }
    }

    pub fn run(&self) {
        let configuration = &self.configuration;
        let verbosity = &configuration.verbosity;

        let metadata_result = fs::metadata(&configuration.raw_path);
        if let Err(_) = metadata_result {
            println!("No such file or directory.");
            exit(1);
        }
        let metadata = metadata_result.unwrap();

        if verbosity == &Verbosity::High {
            println!("{}", configuration);
        }
        if verbosity == &Verbosity::Low {
            println!("Using input file: {}", &configuration.raw_path);
        }
        if verbosity >= &Verbosity::Average {
            println!("Is directory: {}", metadata.is_dir());
            println!("Is file: {}", metadata.is_file());
        }

        if metadata.is_file() {
            Shredder::run_file_shredding(&configuration, &configuration.raw_path, metadata);
        } else {
            if self.configuration.is_recursive {
                Shredder::run_directory_shredding(&configuration, &configuration.raw_path);
                fs::remove_dir_all(Path::new(&configuration.raw_path)).unwrap();
            } else {
                println!("Target is a directory!");
                exit(1);
            }
        }
    }

    fn run_file_shredding(
        configuration: &ShredConfiguration,
        relative_path: &str,
        metadata: Metadata,
    ) -> bool {
        if configuration.verbosity > Verbosity::Low {
            println!("Trying to shred {}", relative_path);
        }
        return match std::fs::canonicalize(relative_path) {
            Ok(path) => {
                let file_length = metadata.len();
                let absolute_path = path.to_str().unwrap();
                if configuration.is_interactive {
                    if !Shredder::user_prompt(absolute_path) {
                        return false;
                    }
                }

                match File::create(absolute_path) {
                    Ok(file) => {
                        if configuration.verbosity > Verbosity::Low {
                            println!("File's size: {}", file_length);
                        }

                        println!("{}", absolute_path);
                        <Shredder<'a>>::shred_file(
                            &file,
                            file_length,
                            absolute_path,
                            &configuration.rewrite_iterations,
                        );

                        if !configuration.keep_files {
                            fs::remove_file(absolute_path).unwrap();
                        }
                        if configuration.verbosity > Verbosity::None {
                            println!("File '{}' shredded!", absolute_path);
                        }
                        true
                    }
                    Err(error) => {
                        println!("{}", error);
                        false
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
                false
            }
        };
    }

    fn shred_file(file: &File, file_length: u64, absolute_path: &str, iterations: &u8) {
        let mut buffer = BufWriter::new(file);

        let pb = ProgressBar::new(file_length);
        pb.set_message(absolute_path);
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})")
            .progress_chars("#>-"));

        for i in 0..*iterations {
            pb.set_prefix(&format!("Iteration {}/{}", i + 1, *iterations));
            let mut bytes_processed = 0;
            while bytes_processed < file_length {
                let bytes_to_write = if file_length - bytes_processed > BATCH_SIZE as u64 {
                    BATCH_SIZE
                } else {
                    (file_length - bytes_processed) as usize
                };

                let random_bytes: Vec<u8> =
                    (0..bytes_to_write).map(|_| rand::random::<u8>()).collect();
                buffer.write(&random_bytes).unwrap();

                bytes_processed = bytes_processed + bytes_to_write as u64;

                pb.set_position(bytes_processed);
            }
            pb.finish_with_message("shredded");

            buffer.flush().unwrap();
            file.sync_all().unwrap();
            buffer.seek(SeekFrom::Start(0)).unwrap();
        }
    }

    fn run_directory_shredding(configuration: &ShredConfiguration, relative_path: &str) {
        let mut files_count = 0;
        for entry in WalkDir::new(relative_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.metadata().unwrap().is_file() {
                let file_path = entry.path().to_str().unwrap();
                if Shredder::run_file_shredding(
                    configuration,
                    file_path,
                    fs::metadata(file_path).unwrap(),
                ) {
                    files_count = files_count + 1;
                }
            }
        }
        if configuration.verbosity != Verbosity::None {
            println!("Processed {} files.", files_count);
        }
    }

    fn user_prompt(path: &str) -> bool {
        print!("Do you really want to shred '{}'? [Y/n] ", path);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let input = input.trim();

        if input.len() != 1 || !input.to_lowercase().eq("y") {
            return false;
        }
        true
    }
}

pub struct ShredConfiguration<'a> {
    verbosity: Verbosity,
    is_recursive: bool,
    is_interactive: bool,
    rewrite_iterations: u8,
    keep_files: bool,
    raw_path: &'a str,
}

impl<'a> ShredConfiguration<'a> {
    pub fn new(path: &'a str) -> ShredConfiguration<'a> {
        ShredConfiguration {
            raw_path: path,
            is_interactive: true,
            is_recursive: false,
            rewrite_iterations: 3,
            keep_files: false,
            verbosity: Verbosity::None,
        }
    }

    pub fn set_verbosity(mut self, verbosity: Verbosity) -> ShredConfiguration<'a> {
        self.verbosity = verbosity;
        self
    }

    pub fn set_is_recursive(mut self, is_recursive: bool) -> ShredConfiguration<'a> {
        self.is_recursive = is_recursive;
        self
    }

    pub fn set_is_interactive(mut self, is_interactive: bool) -> ShredConfiguration<'a> {
        self.is_interactive = is_interactive;
        self
    }

    pub fn set_keep_files(mut self, is_keep_files: bool) -> ShredConfiguration<'a> {
        self.keep_files = is_keep_files;
        self
    }

    pub fn set_rewrite_iterations(mut self, count: u8) -> ShredConfiguration<'a> {
        self.rewrite_iterations = count;
        self
    }

    pub fn build(self) -> ShredConfiguration<'a> {
        self
    }
}

impl<'a> Display for ShredConfiguration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Shredding configuration:")?;
        writeln!(f, "{}", format!("Verbosity: {}", self.verbosity))?;
        writeln!(f, "{}", format!("Is recursive: {}", self.is_recursive))?;
        writeln!(f, "{}", format!("Is interactive: {}", self.is_interactive))?;
        writeln!(
            f,
            "{}",
            format!("Rewrite iterations: {}", self.rewrite_iterations)
        )?;
        writeln!(f, "{}", format!("Keep files: {}", self.keep_files))?;
        writeln!(f, "{}", format!("Path: {}", self.raw_path))?;

        Ok(())
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

impl Display for Verbosity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string_value = match self {
            Verbosity::None => "None",
            Verbosity::Low => "Low",
            Verbosity::Average => "Average",
            Verbosity::High => "High",
        };
        write!(f, "{}", string_value)?;
        Ok(())
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
