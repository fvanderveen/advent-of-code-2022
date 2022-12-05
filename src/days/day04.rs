use std::ops::RangeInclusive;
use crate::days::Day;
use crate::util::number::parse_i32;

pub const DAY4: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let pairs = parse_input(input).unwrap();

    let result = pairs.iter().filter(|p| p.has_range_fully_contained_in_other()).count();
    println!("There are {} pairs where one of the elfs can be lazy!", result);
}
fn puzzle2(input: &String) {
    let pairs = parse_input(input).unwrap();

    let result = pairs.iter().filter(|p| p.has_any_range_overlap()).count();
    println!("There are {} pairs where any part of the range overlaps.", result);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CleaningPair {
    left: RangeInclusive<i32>,
    right: RangeInclusive<i32>
}

impl CleaningPair {
    fn has_range_fully_contained_in_other(&self) -> bool {
        self.left.start() <= self.right.start() && self.left.end() >= self.right.end() ||
            self.right.start() <= self.left.start() && self.right.end() >= self.left.end()
    }

    fn has_any_range_overlap(&self) -> bool {
        self.left.contains(self.right.start()) || self.left.contains(self.right.end()) ||
            self.right.contains(self.left.start()) || self.right.contains(self.left.end())
    }
}

fn parse_input(input: &str) -> Result<Vec<CleaningPair>, String> {
    input.trim().lines().map(|l| parse_pair(l)).collect()
}

fn parse_pair(input: &str) -> Result<CleaningPair, String> {
    let parts = input.trim().split(',').collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(format!("Expected line with two ranges, but got '{}'", input))
    }

    let left = parse_assignment(parts[0])?;
    let right = parse_assignment(parts[1])?;
    Ok(CleaningPair { left, right })
}

fn parse_assignment(input: &str) -> Result<RangeInclusive<i32>, String> {
    let parts = input.split('-').collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(format!("Expected an assignment range, but got '{}'", input));
    }

    let start = parse_i32(parts[0])?;
    let end = parse_i32(parts[1])?;

    Ok(start..=end)
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{CleaningPair, parse_input};

    const TEST_INPUT: &str = "\
        2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8\n\
        ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);

        assert!(result.is_ok(), "Expected a valid parse result");
        let pairs = result.unwrap();
        assert_eq!(6, pairs.len());
        assert_eq!(2..=4, pairs[0].left);
        assert_eq!(6..=8, pairs[0].right);
        assert_eq!(2..=8, pairs[3].left);
        assert_eq!(3..=7, pairs[3].right);
    }

    #[test]
    fn test_has_range_fully_contained_in_other() {
        assert_eq!(false, CleaningPair { left: 2..=4, right: 6..=8 }.has_range_fully_contained_in_other());
        assert_eq!(false, CleaningPair { left: 2..=3, right: 4..=5 }.has_range_fully_contained_in_other());
        assert_eq!(false, CleaningPair { left: 5..=7, right: 7..=9 }.has_range_fully_contained_in_other());
        assert_eq!(true, CleaningPair { left: 2..=8, right: 3..=7 }.has_range_fully_contained_in_other());
        assert_eq!(true, CleaningPair { left: 6..=6, right: 4..=6 }.has_range_fully_contained_in_other());
        assert_eq!(false, CleaningPair { left: 2..=6, right: 4..=8 }.has_range_fully_contained_in_other());
    }

    #[test]
    fn test_has_any_range_overlap() {
        assert_eq!(false, CleaningPair { left: 2..=4, right: 6..=8 }.has_any_range_overlap());
        assert_eq!(false, CleaningPair { left: 2..=3, right: 4..=5 }.has_any_range_overlap());
        assert_eq!(true, CleaningPair { left: 5..=7, right: 7..=9 }.has_any_range_overlap());
        assert_eq!(true, CleaningPair { left: 2..=8, right: 3..=7 }.has_any_range_overlap());
        assert_eq!(true, CleaningPair { left: 6..=6, right: 4..=6 }.has_any_range_overlap());
        assert_eq!(true, CleaningPair { left: 2..=6, right: 4..=8 }.has_any_range_overlap());
    }
}