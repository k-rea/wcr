pub mod output;
pub mod dto;
pub mod count;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::error::Error;
use std::io::BufRead;
use clap::Parser;

use count::count;
use dto::{Args, PrintMode, CountInfo};
use output::display_count_info;
use crate::dto::PrintInputMode;



pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mode: PrintMode = (&args).into();

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;

    for file in args.files {
        let mut reader = match open(&file) {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("{}: {}", file, e);
                continue;
            }
        };
        let info = count(&mut reader)?;

        total_lines += info.lines;
        total_words += info.words;
        total_chars += info.chars;
        total_bytes += info.bytes;

        display_count_info(&file, info, mode);
    }

    if mode.input_mode == PrintInputMode::MultiFiles {
        display_count_info("total", CountInfo {
            lines: total_lines,
            words: total_words,
            chars: total_chars,
            bytes: total_bytes,
        }, mode);
    }

    Ok(())
}


fn open(path: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match path {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(std::io::BufReader::new(File::open(path)?)))
    }
}
