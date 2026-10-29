mod plugin;

use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut input) {
        eprintln!("failed to read stdin: {e}");
        std::process::exit(1);
    }

    match plugin::run(&input) {
        Ok(json) => print!("{json}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
