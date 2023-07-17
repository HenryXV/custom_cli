use std::process;

fn main() {
    if let Err(e) = custom_cli::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
