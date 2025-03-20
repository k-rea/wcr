#[cfg(test)]
mod tests {
    use crate::count::count;

    #[test]
    fn test_count() {
        let input = "Hello, World!\nHello, Rust!";
        let mut reader = std::io::BufReader::new(input.as_bytes());
        let info = count(&mut reader).unwrap();
        assert_eq!(info.lines, 2);
        assert_eq!(info.words, 4);
        assert_eq!(info.chars, 26);
        assert_eq!(info.bytes, 26);
    }
}