use clap::Parser;
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short, long, default_value = "false")]
    lines: bool,
    #[arg(short, long, default_value = "false")]
    words: bool,
    #[arg(short('m'), long, default_value = "false", conflicts_with = "bytes")]
    chars: bool,
    #[arg(short('c'), long, default_value = "false")]
    bytes: bool,
}
#[derive(Debug, Copy, Clone)]
pub enum CharsOrBytes {
    Chars,
    Bytes,
    None
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PrintInputMode {
    Stdin,
    MultiFiles,
    SingleFile,
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct PrintMode {
    pub input_mode: PrintInputMode,
    pub lines: bool,
    pub words: bool,
    pub chars_or_bytes: CharsOrBytes,
}

impl From<&Args> for PrintMode {
    fn from(args: &Args) -> Self {
        let input_mode = match args.files.as_slice() {
            [file] if file == "-" => PrintInputMode::Stdin,
            [_] => PrintInputMode::SingleFile,
            _ => PrintInputMode::MultiFiles,
        };
        if !args.lines && !args.words && !args.chars && !args.bytes {
            return PrintMode {
                input_mode,
                lines: true,
                words: true,
                chars_or_bytes: CharsOrBytes::Bytes,
            };
        }
        let chars_or_bytes = match (args.chars, args.bytes) {
            (true, true) => CharsOrBytes::Bytes,
            (true, false) => CharsOrBytes::Chars,
            (false, true) => CharsOrBytes::Bytes,
            (false, false) => CharsOrBytes::None,
        };
        PrintMode {
            input_mode,
            lines: args.lines,
            words: args.words,
            chars_or_bytes,
        }
    }
}

#[derive(Debug)]
pub struct CountInfo {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}
