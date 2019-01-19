mod grid;

use std::fs;
use std::str::FromStr;
use regex::Regex;
use self::grid::Grid2D;
use std::cmp;

#[cfg(test)]
mod tests {
    use crate::{
        Point,
        get_edge_coordinates,
    };

    fn distance_between_coordinates(p1: (i32, i32), p2: (i32, i32)) -> i32 {
        Point {
            x: p1.0,
            y: p1.1,
        }.manhattan_distance(Point {
            x: p2.0,
            y: p2.1,
        })
    }

    #[test]
    fn it_calculates_distance() {
        assert_eq!(distance_between_coordinates((1, 1), (1, 1)), 0);
        assert_eq!(distance_between_coordinates((2, 1), (1, 1)), 1);
        assert_eq!(distance_between_coordinates((1, 1), (2, 1)), 1);
    }

    #[test]
    fn it_calculate_grid_boundries() {
        assert_eq!(get_edge_coordinates(&vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 8, y: 9 },
        ]), (9, 9))
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, p2: Point) -> i32 {
        ((self.x - p2.x) + (self.y - p2.y)).abs()
    }
}

fn read_seed_file() -> String {
    fs::read_to_string("seed").expect("something went wrong reading the file")
}

fn parse_points(line: &str, index: u8) -> Point {
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

fn get_points() -> Result<Vec<Point>, &'static str> {
    let mut points: Vec<Point> = Vec::new();
    let mut index: u8 = 0;

    Ok( read_seed_file().lines().map(|x| {
        index += 1;
        parse_points(x, index)
    }).collect() )
}

fn get_edge_coordinates(points: &Vec<Point>) -> (i32, i32) {
    let x = points.iter().map(|c| { c.x } ).max().unwrap();
    let y = points.iter().map(|c| { c.y } ).max().unwrap();

    (cmp::max(x, y), cmp::max(x, y))
}

fn main() {
    let points = get_points().unwrap();
    let grid_offset = get_edge_coordinates(&points);

    let mut grid = Grid2D::new_sized(
        grid_offset.0 as usize,
        grid_offset.1 as usize,
        &0_i32,
    );

    let p1 = Point {
        x: 5,
        y: 5,
    };

    println!("{}", p1.manhattan_distance(Point {
        x: 6,
        y: 5,
    }));
}
