extern crate brainf;

use std::env;

fn run() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: bf FILENAME".to_string());
    }

    let filename = &args[1];

    brainf::run_file(filename)
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}
