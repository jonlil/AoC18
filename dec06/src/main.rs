use std::fs;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Grid {
    boundry: (
        Point<i32>,
        Point<i32>,
    )
}

impl Grid {
    fn new(b1: Point<i32>, b2: Point<i32>) -> Grid {
        Grid { boundry: (b1, b2) }
    }
}

impl Iterator for Grid {
    type Item = Point<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        Some( Point {
            x: 0,
            y: 0,
        } )
    }
}

fn read_seed_file() -> String {
    fs::read_to_string("seed").expect("something went wrong reading the file")
}

fn parse_points(line: &str, index: u8) -> Point<i32> {
    let re = Regex::new(r"(?P<x>\d+),\s{1}(?P<y>\d+)").unwrap();

    match re.captures(line) {
        Some(data) => {
            Point {
                x: FromStr::from_str(&data["x"]).unwrap(),
                y: FromStr::from_str(&data["y"]).unwrap(),
            }
        },
        None => panic!("Regex capture problem")
    }
}

fn get_points() -> Result<Vec<Point<i32>>, &'static str> {
    let mut points: Vec<Point<i32>> = Vec::new();
    let mut index: u8 = 0;

    Ok( read_seed_file().lines().map(|x| {
        index += 1;
        parse_points(x, index)
    }).collect() )
}

fn main() {
    let points = get_points().unwrap();

    let greatest_x = &points.iter().map(|c| { c.x } ).max().unwrap();
    let smallest_x = &points.iter().map(|c| { c.x } ).min().unwrap();
    let greatest_y = &points.iter().map(|c| { c.y } ).max().unwrap();
    let smallest_y = &points.iter().map(|c| { c.y } ).min().unwrap();

    let grid = Grid::new(
        Point { x: (*smallest_x - 1), y: (*smallest_y - 1) },
        Point { x: (*greatest_x + 1), y: (*greatest_y + 1) },
    );

    println!("{:?}", grid);
}
