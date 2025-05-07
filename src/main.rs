use minigrep;
use std::process;

fn main() {
    let config = minigrep::read().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("{}", err);
        process::exit(1);
    };
}
