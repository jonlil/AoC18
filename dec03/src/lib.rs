extern crate regex;

use std::fs;
use std::ops::{Add, Range};
use std::process;
use regex::Regex;
use std::str::FromStr;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let filename = args[0].clone();
        Ok(Config { filename })
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[derive(Debug)]
struct FabricClaim {
    id: u32,
    rectangle: Rectangle,
}

fn fabric_claim_matcher<'a>(contents: &'a str) -> Result<Vec<FabricClaim>, &'static str> {
    let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    let mut items: Vec<FabricClaim> = vec![];

    for line in contents.lines() {
        for cap in re.captures_iter(&line) {
           let x1: u32 = FromStr::from_str(&cap[2]).unwrap();
           let y1: u32 = FromStr::from_str(&cap[3]).unwrap();
           let p1: Point = Point::new(x1, y1);

           items.push(FabricClaim {
               id: FromStr::from_str(&cap[1]).unwrap(),
               rectangle: Rectangle {
                   p1: Point::new(x1, y1),
                   p2: p1 + Point::new(
                       FromStr::from_str(&cap[4]).unwrap(),
                       FromStr::from_str(&cap[5]).unwrap(),
                   )
               }
           });
        }
    }

    Ok(items)
}

pub fn run(config: Config) {
    let contents = fs::read_to_string("seed")
        .expect("something went wrong reading the file");


    let claims = fabric_claim_matcher(&contents).unwrap_or_else(|err| {
        println!("Problem parsing claims: {}", err);
        process::exit(1)
    });

    //
    let mut reserved_tiles = vec![vec![0u32; 1000]; 1000];
    let mut overlapping_tiles = 0u32;

    println!("{:?}", reserved_tiles);

    for claim in claims.iter() {
        for horizontal in claim.rectangle.p1.x..(claim.rectangle.p2.x) {
            for vertical in claim.rectangle.p1.y..(claim.rectangle.p2.y) {
                reserved_tiles[horizontal as usize][vertical as usize] += 1;

                if reserved_tiles[horizontal as usize][vertical as usize] == 2 {
                    overlapping_tiles += 1;
                }
            }
        }
    }
    println!("Overlapping: {}", overlapping_tiles);

    for claim in claims.iter() {
        let mut overlapped: bool = false;
        for horizontal in claim.rectangle.p1.x..(claim.rectangle.p2.x) {
            for vertical in claim.rectangle.p1.y..(claim.rectangle.p2.y) {
                if reserved_tiles[horizontal as usize][vertical as usize] >= 2 {
                    overlapped = true;
                }
            }
        }

        if !overlapped {
            println!("Claim {} doesn't overlap", claim.id);
        }
    }
}
