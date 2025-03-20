use std::error::Error;
use std::io::BufRead;

use crate::dto::CountInfo;

pub fn count(reader: &mut impl BufRead) -> Result<CountInfo, Box<dyn Error>> {
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut bytes = 0;
    let mut size:usize;
    let mut line: String = String::new();

    loop {
        size = reader.read_line(&mut line)?;
        if size == 0 {
            break;
        }

        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count();
        bytes += size;

        line.clear();
    }

    Ok(CountInfo {
        lines,
        words,
        chars,
        bytes,
    })

}
