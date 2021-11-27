use std::num::ParseIntError;

const TARGET: i32 = 2020;

pub fn part1(values: Vec<i32>) -> Option<i32> {
    for (index, v1) in values.iter().enumerate() {
        for v2 in values.iter().skip(index + 1) {
            if v1 + v2 == TARGET {
                return Some(v1 * v2);
            }
        }
    }
    return None;
}

pub fn parse_values(input: &str) -> Result<Vec<i32>, ParseIntError> {
    return input
        .lines()
        .map(|line| line.parse())
        .collect();
}
