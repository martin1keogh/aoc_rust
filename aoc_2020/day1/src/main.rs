use std::num::ParseIntError;

use aoc::PuzzleInfo;

const TARGET: i32 = 2020;

fn main() -> () {
    aoc::solve_puzzle(PuzzleInfo { year: 2020, day: 1 }, parse_values, part1, part2)
}

fn part1(values: Vec<i32>) -> Option<i32> {
    solve_for(values, 2)
}
fn part2(values: Vec<i32>) -> Option<i32> {
    solve_for(values, 3)
}

fn combinations(values: Vec<i32>, k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![Vec::new()];
    }
    if values.len() <= k {
        return vec![values];
    }
    let (head, tail) = values.split_at(1);

    let without_head = combinations(tail.to_vec(), k);
    let with_head =
        combinations(tail.to_vec(), k - 1).iter()
            .map(|sub| [head, sub].concat())
            .collect::<Vec<Vec<i32>>>()
    ;
    return [with_head, without_head].concat()
}

fn solve_for(values: Vec<i32>, k: usize) -> Option<i32> {
    combinations(values, k).iter()
        .find(|comb| comb.iter().sum::<i32>() == TARGET)
        .map(|t| t.iter().product())
}

fn parse_values(input: &str) -> Result<Vec<i32>, ParseIntError> {
    return input
        .lines()
        .map(|line| line.parse())
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "1721
979
366
299
675
1456";
    const PARSED: &[i32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_parse_values() {
        let result = parse_values(INPUT).unwrap();
        assert_eq!(result, PARSED)
    }

    #[test]
    fn test_part1() {
        let result = part1(PARSED.to_vec()).unwrap();
        assert_eq!(result, 514579)
    }

    #[test]
    fn test_part2() {
        let result = part2(PARSED.to_vec()).unwrap();
        assert_eq!(result, 241861950)
    }

    #[test]
    fn test_combinations_2() {
        let input = vec![1, 2, 3, 4];
        let expected = [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]];
        assert_eq!(combinations(input, 2), expected)
    }

    #[test]
    fn test_combinations_3() {
        let input = vec![1, 2, 3, 4];
        let expected = [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]];
        assert_eq!(combinations(input, 3), expected)
    }
}
