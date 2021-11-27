use day1;

const INPUT: &'static str = "1721
979
366
299
675
1456";
const PARSED: &[i32] = &[1721, 979, 366, 299, 675, 1456];

#[test]
fn test_parse_values() {
    let result = day1::parse_values(INPUT).unwrap();
    assert_eq!(result, PARSED)
}

#[test]
fn test_part1() {
    let result = day1::part1(PARSED.to_vec()).unwrap();
    assert_eq!(result, 514579)
}
