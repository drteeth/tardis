use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".tardis.log")
        .unwrap();

    let args: Vec<String> = env::args().collect();

    for arg in args { // each String is moved into s here...
        if let Err(e) = writeln!(file, "{:?}", arg) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
