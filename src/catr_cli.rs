use std::io::{BufRead, Read};
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};

use clap::Parser;

type CatrCliResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
#[command(
    name = "catr",
    version = "0.1.0",
    about = "Very Simple implementation of GNU cat",
    long_about = "Very simple implementation of GNU cat : only some options has been added to this version"
)]
pub struct CatrCli {
    #[arg(short = 'n', long = "number", help = "number all output lines")]
    number: bool,
    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "number nonempty output lines, overrides -n"
    )]
    non_blank_number: bool,
    files: Vec<PathBuf>,
}

impl CatrCli {
    pub fn print_content(&self) -> CatrCliResult {
        if self.files.is_empty() {
            return self.print_from_stdin();
        }

        for file in &self.files {
            self.print_from_file(file)?
        }

        Ok(())
    }

    fn _print_from_stdin_readline(&self) -> CatrCliResult {
        let mut counter = 1;
        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            let line = buffer.trim_end();
            if self.non_blank_number && line.is_empty() {
                println!();
                continue;
            }

            if self.non_blank_number || self.number {
                println!("{: >6}  {}", counter, line);
                counter += 1;
            } else {
                println!("{}", line);
            }
        }
    }

    fn print_string<T: Read>(&self, reader: io::BufReader<T>) -> CatrCliResult {
        let mut result = String::new();
        let mut counter = 1;
        let lines = reader.lines();
        for line in lines {
            let line = line?;
            if self.non_blank_number && line.is_empty() {
                result.push_str("\n");
                continue;
            }

            if self.non_blank_number || self.number {
                result.push_str(format!("{: >6}\t{}\n", counter, line).as_str());
            } else {
                result.push_str(format!("{}\n", line).as_str());
            }

            counter += 1;
        }

        print!("{}", result);

        Ok(())
    }

    fn print<T: Read>(&self, reader: io::BufReader<T>) -> CatrCliResult {
        let mut counter = 1;
        let lines = reader.lines();
        for line in lines {
            let line = line?;
            if self.non_blank_number && line.is_empty() {
                println!();
                continue;
            }

            if self.non_blank_number || self.number {
                println!("{: >6}\t{}\n", counter, line);
            } else {
                println!("{}\n", line);
            }

            counter += 1;
        }

        Ok(())
    }

    fn print_from_stdin(&self) -> CatrCliResult {
        let reader = io::BufReader::new(io::stdin());
        self.print(reader)?;
        Ok(())
    }

    fn print_from_file(&self, file_path: &PathBuf) -> CatrCliResult {
        if !file_path.exists() {
            println!(
                "catr: {} : No such file or directory",
                file_path.to_string_lossy()
            )
        }
        if file_path.is_dir() {
            println!("catr: {} : Is a directory", file_path.to_string_lossy())
        }

        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        self.print_string(reader)?;
        Ok(())
    }
}
