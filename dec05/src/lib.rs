use std::fs;
use std::str::FromStr;
use std::str;

#[cfg(test)]
mod tests {
    use crate::{Pairs};
    #[test]
    fn it_can_react() {
        assert!(can_react(&String::from("a"), &String::from("A")));
        assert!(can_react(&String::from("A"), &String::from("a")));
        assert!(can_react(&String::from("b"), &String::from("B")));
        assert!(can_react(&String::from("B"), &String::from("b")));
    }

    #[test]
    fn it_can_not_react() {
        assert!(!can_react(&String::from("b"), &String::from("c")));
        assert!(!can_react(&String::from("B"), &String::from("B")));
        assert!(!can_react(&String::from("b"), &String::from("b")));
    }

    #[test]
    fn it_can_iterate() {
        let content = "cdabcdef".to_string();
        let mut v = content.as_bytes().to_vec();
        v.reverse();

        println!("{:?}", v);

        let mut pairs = Pairs {
            pairs: v,
        };

        assert_eq!(&pairs.next(), &Some((99, 100)));
        assert_eq!(&pairs.next(), &Some((100, 97)));
        assert_eq!(&pairs.next(), &Some((97, 98)));
        assert_eq!(&pairs.next(), &Some((98, 99)));
        assert_eq!(&pairs.next(), &Some((99, 100)));
        assert_eq!(&pairs.next(), &Some((100, 101)));
        assert_eq!(&pairs.next(), &Some((101, 102)));
        assert_eq!(&pairs.next(), &Some((102, 0)));
        assert_eq!(&pairs.next(), &Some((0, 0)));
    }
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let filename = args[0].clone();
        Ok(Config { filename })
    }
}

fn can_react(a: &str, b: &str) -> bool {
    if a.to_lowercase() != b.to_lowercase() {
        return false;
    }

    if a == b {
        false
    } else {
        true
    }
}

struct Pairs {
    pairs: Vec<u8>,
}

impl Pairs {
    fn new(pairs: Vec<u8>) -> Pairs {
        let mut v = pairs.to_owned();
        v.reverse();

        Pairs {
            pairs: v,
        }
    }
}

impl ExactSizeIterator for Pairs {
    fn len(&self) -> usize {
        self.pairs.len()
    }
}

impl Iterator for Pairs {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let a = match self.pairs.pop() {
            Some(ba) => ba,
            None => 0_u8,
        };

        let b = match self.pairs.pop() {
            Some(ab) => ab,
            None => 0_u8,
        };

        if b == 0_u8 || a == 0_u8 {
            return None;
        } else {
            self.pairs.push(b.to_owned());
        }

        return Some((a, b));
    }
}

pub fn run(_config: Config) {
    let mut contents = fs::read_to_string("seed")
        .expect("something went wrong reading the file");

    let mut done: bool = false;

    let pairs = Pairs::new(contents.as_bytes().to_vec());

    while !done {
        let pairs = Pairs::new(contents.as_bytes().to_vec());
        let prev_length = pairs.len().to_owned();
        let mut results: String = "".to_string();
        let mut index: usize = 0 as usize;
        let mut has_reacted = false;

        for pair in pairs {
            index += 1 as usize;
            if has_reacted == true {
                has_reacted = false;
                continue;
            }

            let a = str::from_utf8(&[pair.0]).unwrap().to_owned();
            let b = str::from_utf8(&[pair.1]).unwrap().to_owned();

            if !can_react(&a, &b) {
                results += &a.to_owned();
            } else {
                has_reacted = true;
            }

            if index == prev_length - 1 {
                results += &b.to_owned();
            }
        }

        if prev_length == results.len() {
            done = true;
        }
        contents = results.to_owned();
    }

    println!("{}", contents);
}
