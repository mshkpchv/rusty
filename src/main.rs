use std::fs::File;
use std::io::{BufReader};
use intiterator::{IntLineReader};

fn main() {
    let file = File::open("test/foo.txt").unwrap();
    let int_reader = IntLineReader::new(BufReader::new(file));
    for line in int_reader {
        println!("{}", line);
    }
}

