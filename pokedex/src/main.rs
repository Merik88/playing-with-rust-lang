use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("./src/data.txt")
        .expect("Could not open file!");

    let mut buff_reader = BufReader::new(file);

    let mut content = String::new();
    buff_reader.read_to_string(&mut content)
        .expect("Could not read content to String");

    println!("{}", content);
}
