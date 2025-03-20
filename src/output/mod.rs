use crate::dto::{CharsOrBytes, CountInfo, PrintInputMode, PrintMode};

pub fn display_count_info(file: &str, info: CountInfo, mode: PrintMode) {
    if mode.lines {
        print!("{:>8}", info.lines);
    }
    if mode.words {
        print!("{:>8}", info.words);
    }
    match mode.chars_or_bytes {
        CharsOrBytes::Chars => print!("{:>8}", info.chars),
        CharsOrBytes::Bytes => print!("{:>8}", info.bytes),
        CharsOrBytes::None => {}
    }
    if mode.input_mode != PrintInputMode::Stdin {
        println!(" {}", file);
    } else {
        println!();
    }
}
