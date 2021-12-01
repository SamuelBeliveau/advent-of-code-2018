use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("cannot read file");

    contents
}
