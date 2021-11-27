use std::num::ParseIntError;

use day1::*;

const INPUT: &'static str = "1721
979
366
299
675
1456";

fn main() -> Result<(), ParseIntError> {
    let values = parse_values(INPUT)?;
    println!("Part 1 result: {:?}", part1(values));
    Ok(())
}
