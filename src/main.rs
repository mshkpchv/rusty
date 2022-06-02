use std::fs::File;
use std::num::ParseIntError;
// use intiterator::{algo};

fn main() {
    // let file = String::from("test_files/success");
    // let buf = BufReader::new(file);
    // algo(&file,3);
    // println!();
    // let x = [0 as u64; std::u64::MAX as usize];
    println!("{}", i32::MAX / (2 ^ 25));
    match String::from("1212").parse::<i32>() {
        Ok(_) => {}
        Err(err) => {}
    }

    // algo(&String::from("test_files/"),3);


}

