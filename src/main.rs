fn main() {
    if let Err(e) = wcr::run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
