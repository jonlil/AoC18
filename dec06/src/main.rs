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
        Point<usize>,
        Point<usize>,
    )
}

impl Grid {
    fn new(b1: Point<usize>, b2: Point<usize>) -> Grid {
        Grid { boundry: (b1, b2) }
    }
}

impl Iterator for Grid {
    type Item = Point<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        Some( Point {
            x: 0 as usize,
            y: 0 as usize,
        } )
    }
}

fn read_seed_file() -> String {
    fs::read_to_string("seed").expect("something went wrong reading the file")
}

fn parse_points(line: &str, index: u8) -> Point<i64> {
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

fn get_points() -> Result<Vec<Point<i64>>, &'static str> {
    let mut points: Vec<Point<i64>> = Vec::new();
    let mut index: u8 = 0;

    Ok( read_seed_file().lines().map(|x| {
        index += 1;
        return parse_points(x, index);
    }).collect() )
}

fn main() {
    let points = get_points().unwrap();

    let greatest_x = &points.iter().map(|c| { c.x } ).max().unwrap();
    let greatest_y = &points.iter().map(|c| { c.y } ).max().unwrap();

    let grid = Grid::new(
        Point { x: 0 as usize, y: 0 as usize },
        Point { x: *greatest_x as usize, y: *greatest_y as usize },
    );

    println!("{:?}", grid);
}
