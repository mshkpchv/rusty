use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use rand::Rng;

pub struct IntLineReader<R> {
    reader: BufReader<R>,
}

impl<R: Read> IntLineReader<R> {
    pub fn new(r: BufReader<R>) -> IntLineReader<R> {
        IntLineReader { reader: r }
    }

    fn read(&mut self) -> Option<String> {
        let mut buf = String::new();
        let ps = match self.reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                return Some(buf)
            }
            // Err(e) => Some(e),
            Err(e) => Some(String::from("problem")),
        };
        ps
    }
}

impl<R: Read> Iterator for IntLineReader<R> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let option = self.read();
        let rr:Option<i32> = match option {
            None => None,
            Some(x) => {
               // let rr = match x.parse::<i32>() {
               //      Ok(x) => Some(x),
               //      Err(_) => None
               //  };
               //  rr
                return Some(x.parse::<i32>().unwrap())
            }
        };
        rr
    }
}

// impl Iterator for IntLine {
//     type Item = usize;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.open {
//             self.reader = BufReader::new(&self.f)
//         }
//         let mut line = String::new();
//         let len = line.len();
//         return Some(len);
//     }
// }
// impl<B: BufRead> Iterator for IntLine<B> {
//     type Item = Result<String>;
//
//     fn next(&mut self) -> Option<Result<String>> {
//         let mut buf = String::new();
//         match self.reader.read_line(&mut buf) {
//             Ok(0) => None,
//             Ok(_n) => {
//                 if buf.ends_with('\n') {
//                     buf.pop();
//                     if buf.ends_with('\r') {
//                         buf.pop();
//                     }
//                 }
//                 Some(Ok(buf))
//             }
//             Err(e) => Some(Err(e)),
//         }
//     }
// }