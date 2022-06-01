use std::io::{BufRead, BufReader, Read};

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
            Err(_) => Some(String::from("ERROR")),
        };
        return ps
    }
}

impl<R: Read> Iterator for IntLineReader<R> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
            let ss = self.read();
            let rr:Option<Self::Item> = match ss {
                None => None,
                Some(x) => {
                    return match x.parse::<Self::Item>() {
                         Ok(x) => Some(x),
                         Err(_) => return self.next()
                    };
                }
            };
        return rr
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader};
    use crate::IntLineReader;

    #[test]
    fn it_handles_empty_files() {
        let file = File::open("test_files/empty_file").unwrap();
        let mut int_reader = IntLineReader::new(BufReader::new(file));

        assert_eq!(int_reader.next(), None);
    }

    #[test]
    fn it_handles_file_with_one_line() {
        let file = File::open("test_files/one_line_file", ).unwrap();
        let mut int_reader = IntLineReader::new(BufReader::new(file));
        assert_eq!(int_reader.next(), Some(1234));
        assert_eq!(int_reader.next(), None);
    }

    #[test]
    fn it_handles_file_with_multi_lines() {
        let file = File::open("test_files/multi_line_file").unwrap();
        let mut int_reader = IntLineReader::new(BufReader::new(file));

        assert_eq!(int_reader.next(), Some(1234));
        assert_eq!(int_reader.next(), Some(1234));
        assert_eq!(int_reader.next(), Some(1234));
        assert_eq!(int_reader.next(), Some(1234));
        assert_eq!(int_reader.next(), None);

    }

    #[test]
    fn it_handles_file_with_odd_numbers<>() {
        let file = File::open("test_files/multi_line_odd_file").unwrap();
        let mut int_reader = IntLineReader::new(BufReader::new(file));

        assert_eq!(int_reader.next(), Some(1));
        assert_eq!(int_reader.next(), Some(3));
        assert_eq!(int_reader.next(), Some(5));
        assert_eq!(int_reader.next(), Some(7));
        assert_eq!(int_reader.next(), None);

    }

    #[test]
    fn it_handles_file_with_starting_empty_lines<>() {
        let file = File::open("test_files/multi_start_lines").unwrap();
        let mut int_reader = IntLineReader::new(BufReader::new(file));
        assert_eq!(int_reader.next(), Some(1));
        assert_eq!(int_reader.next(), None);
    }

    #[test]
    fn it_handles_file_with_trailing_lines<>() {
        let file = File::open("test_files/multi_end_lines").unwrap();
        let mut int_reader = IntLineReader::new(BufReader::new(file));
        assert_eq!(int_reader.next(), Some(1));
        assert_eq!(int_reader.next(), None);
    }

}