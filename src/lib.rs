extern crate core;

use std::cmp::Ordering;
use std::io::{Error, ErrorKind};
use itertools::izip;
use std::num::ParseIntError;

pub struct CorrectLines<I> {
    iter: I,
}

impl<I> CorrectLines<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for CorrectLines<I>
    where
        I: Iterator<Item=Result<String, Error>>
{
    type Item = Result<i32, ParseIntError>;

    fn next(&mut self) -> Option<Self::Item> {
        let ss = self.iter.next();
        let rr = match ss {
            Some(x) => {
                return match x {
                    Ok(x) => Some(x.parse::<i32>()),
                    Err(err) => panic!("Problem with reading line {}", err)
                };
            }
            None => None
        };
        return rr;
    }
}

pub trait CorrectLinesIterator<T>: Iterator<Item=T> + Sized {
    fn to_int(self) -> CorrectLines<Self> {
        CorrectLines::new(self)
    }
}

impl<T, I: Iterator<Item=T>> CorrectLinesIterator<T> for I {}

fn algo<T>(iterator1: T, iterator2: T, skip_val: i32) -> Result<(i32, i32), Error>
    where T: Iterator<Item=(usize, Result<i32, ParseIntError>)>
{
    let iterator2 = iterator2.skip(skip_val as usize);

    let mut current_max = 0;
    let mut target_i: i32 = -1;
    let mut target_j: i32 = -1;

    for ((i_line, i_result), (j_line, j_result)) in izip!(iterator1,iterator2) {
        match (i_result, j_result) {
            (Ok(l1), Ok(l2)) => {
                let current_sum = l1 + l2;
                match current_sum.cmp(&current_max) {
                    Ordering::Greater => {
                        target_i = i_line as i32;
                        target_j = j_line as i32;
                        current_max = current_sum;
                    }
                    _ => {}
                }
            }
            _ => { continue; }
        }
    }

    return match (target_i, target_j)
    {
        (-1, _) => { Err(Error::from(ErrorKind::InvalidData)) }
        (_, -1) => { Err(Error::from(ErrorKind::InvalidData)) }
        (x, y) => Ok((x, y))
    };
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind};
    use std::num::{ParseIntError};
    use crate::{algo, CorrectLinesIterator};

    fn to_int_result(x: i32) -> Result<i32, ParseIntError> {
        Ok(x)
    }

    fn to_string_result(s: &String) -> Result<String, Error> {
        Ok(s.to_string())
    }

    #[test]
    fn test_with_normal_range() {
        let min = 0;
        let max = 100;
        let skip = 3;

        let range1 = (min..max).map(to_int_result).enumerate();
        let range2 = (min..max).map(to_int_result).enumerate();

        let res = algo(range2, range1, skip);

        assert_eq!(res.unwrap(), (max - skip - 1, max - 1));
    }

    #[test]
    fn test_handles_empty_iterator_from_range() {
        let min = 0;
        let max = 1;
        let skip = 3;

        let range1 = (min..max).map(to_int_result).enumerate();
        let range2 = (min..max).map(to_int_result).enumerate();

        let res = algo(range2, range1, skip);
        match res {
            Ok(_) => { panic!() }
            Err(err) => { assert_eq!(err.kind(), ErrorKind::InvalidData) }
        }
    }

    #[test]
    fn test_with_one_empty_iterator() {
        let min = 0;
        let max = 1;
        let skip = 3;

        let range1 = (10..10000).map(to_int_result).enumerate();
        let range2 = (min..max).map(to_int_result).enumerate();

        let res = algo(range1, range2, skip);
        match res {
            Ok(_) => { panic!() }
            Err(err) => { assert_eq!(err.kind(), ErrorKind::InvalidData) }
        }
    }

    #[test]
    fn test_handles_asc_iterators() {
        let file = String::from("test_files/asc");
        let f1 = File::open(&file).unwrap();
        let f2 = File::open(&file).unwrap();

        let skip = 5;
        let max = 10;

        let range1 = BufReader::new(f1).lines().to_int().enumerate();
        let range2 = BufReader::new(f2).lines().to_int().enumerate();

        match algo(range1, range2, skip) {
            Ok(res) => { assert_eq!(res, (max - skip - 1, max - 1)) }
            Err(_) => { panic!("do not expect it") }
        }
    }

    #[test]
    fn test_handles_desc_iterators() {
        let file = String::from("test_files/desc");
        let f1 = File::open(&file).unwrap();
        let f2 = File::open(&file).unwrap();
        let skip = 5;
        let first = 0;

        let range1 = BufReader::new(f1).lines().to_int().enumerate();
        let range2 = BufReader::new(f2).lines().to_int().enumerate();

        match algo(range1, range2, skip) {
            Ok(res) => { assert_eq!(res, (first,skip)) }
            Err(_) => { panic!("do not expect it") }
        }
    }

    #[test]
    fn test_handles_random_numbers() {
        let file = String::from("test_files/random");
        let f1 = File::open(&file).unwrap();
        let f2 = File::open(&file).unwrap();
        let skip = 10;
        let range1 = BufReader::new(f1).lines().to_int().enumerate();
        let range2 = BufReader::new(f2).lines().to_int().enumerate();

        match algo(range1, range2, skip) {
            Ok((i,j)) => { }
            Err(_) => { panic!("do not expect it") }
        }
    }


    #[test]
    fn test_handles_iterator_greater_than_max_arr_size() {
        let min: i32 = 0;
        let max = i32::MAX / (2 ^ 10);
        let skip = 1000000;
        let range1 = (min..max).map(to_int_result).enumerate();
        let range2 = (min..max).map(to_int_result).enumerate();

        let res = algo(range2, range1, skip);

        assert_eq!(res.unwrap(), (max - skip - 1, max - 1));
    }

    #[test]
    fn test_handles_comment_iterators_algo() {
        let a = vec![String::from("str"), String::from("u123"), String::from("32.22"),
                     String::from("1.22")];
        let iterator1 = a.iter().map(to_string_result).to_int().enumerate();
        let iterator2 = a.iter().map(to_string_result).to_int().enumerate();
        let skip = 0;
        match algo(iterator1, iterator2, skip) {
            Ok(_) => { panic!() }
            Err(_) => {}
        }
    }

    // int iterator tests

    #[test]
    fn it_handles_empty_files() {
        let file = "test_files/empty_file".to_string();
        let f = File::open(file);
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut int_iter = BufReader::new(f).lines().to_int().enumerate();
        let result = int_iter.next();
        assert_eq!(result, None);
    }

    #[test]
    fn it_handles_file_with_one_line() {
        let file = File::open("test_files/one_line_file").unwrap();
        let mut int_reader = BufReader::new(file).lines().to_int().enumerate();
        assert_eq!(int_reader.next(), Some((0, Ok(1234))));
        match int_reader.next() {
            None => { panic!() }
            Some((line, el)) => {
                assert_eq!(line, 1);
                assert_eq!(el.is_err(), true)
            }
        }
        assert_eq!(int_reader.next(),None)
    }

    #[test]
    fn test_with_odd_lines_files() {
        let file = File::open("test_files/odd_lines").unwrap();
        let int_reader = BufReader::new(file).lines().to_int().enumerate();
        let mut index = 0;
        for el in int_reader {
            let (line, res) = el;
            assert_eq!(line, index);
            if index % 2 == 0 {
                assert_eq!(res.is_ok(), true)
            } else {
                assert_eq!(res.is_err(), true)
            }

            index = index + 1;
        }
    }

    #[test]
    fn test_int_adapter_with_vector() {
        let a = vec![String::from("1"), String::from("1.2")];
        let mut int_iter = a.iter().map(to_string_result).to_int();
        let next = int_iter.next();
        match next {
            None => panic!(),
            Some(res) => {
                match res {
                    Ok(el) => { assert_eq!(el, 1) }
                    Err(_) => panic!()
                }
            }
        }
        let next = int_iter.next();
        match next {
            None => panic!(),
            Some(res) => {
                assert_eq!(res.is_err(), true)
            }
        }
        let next = int_iter.next();
        assert_eq!(next, None)
    }
}
