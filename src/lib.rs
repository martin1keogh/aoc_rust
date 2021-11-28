use std::env;
use std::fmt::{Display, Formatter};

pub mod data;

pub struct PuzzleInfo {
    pub year: i16,
    pub day: i8,
}

impl Display for PuzzleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Puzzle AoC {}, day {}", self.year, self.day)
    }
}


pub fn solve_puzzle<T, E: std::fmt::Debug>(
    pi: PuzzleInfo,
    parser: fn(&str) -> Result<T, E>,
    part1: fn(T) -> Option<i32>,
    part2: fn(T) -> Option<i32>,
) -> () {
    let session = env::var("SESSION").unwrap();
    let data = data::get_data(&pi, session.as_str()).unwrap();

    match parser(data.as_str()).map(part1) {
        Ok(Some(res)) => println!("{} part1's solution: {}", pi, res),
        Ok(None) => println!("{} part1, solution not found", pi),
        Err(error) => println!("{} part1 failed: {:?}", pi, error),
    };

    match parser(data.as_str()).map(part2) {
        Ok(Some(res)) => println!("{} part2's solution: {}", pi, res),
        Ok(None) => println!("{} part2, solution not found", pi),
        Err(error) => println!("{} part2 failed: {:?}", pi, error),
    }
}
