use std::num::ParseIntError;

const INPUT: &'static str = "1721
979
366
299
675
1456";

const TARGET: i32 = 2020;

fn main() -> Result<(), ParseIntError> {
    let values = parse_values(INPUT)?;
    println!("Part 1 result: {:?}", part1(values));
    Ok(())
}

fn part1(values: Vec<i32>) -> Option<i32> {
    for (index, v1) in values.iter().enumerate() {
        for v2 in values.iter().skip(index + 1) {
            if v1 + v2 == TARGET {
                return Some(v1 * v2);
            }
        }
    }
    return None;
}

fn parse_values(input: &str) -> Result<Vec<i32>, ParseIntError> {
    return input
        .lines()
        .map(|line| line.parse())
        .collect();
}
