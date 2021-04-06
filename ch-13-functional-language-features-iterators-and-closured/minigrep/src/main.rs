use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let config = parse_config(&args);
    // Pass the iterator env::args() instead of collecting it into a vector
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // We did not use unwrap or else because the function doesn't return anything on success
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
