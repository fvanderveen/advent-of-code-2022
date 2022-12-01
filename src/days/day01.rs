use crate::days::Day;
use crate::util::number::parse_i32;

pub const DAY1: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let backpacks = parse_input(input).unwrap();

    let result = find_most_calories(backpacks).unwrap();
    println!("Most total calories carried: {}", result);
}
fn puzzle2(input: &String) {
    let backpacks = parse_input(input).unwrap();

    let result = get_top_three_calories(backpacks);
    println!("Top three calories summed: {}", result);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Backpack {
    food_calories: Vec<i32>
}

fn parse_input(input: &str) -> Result<Vec<Backpack>, String> {
    let mut result: Vec<Backpack> = vec![];
    let mut current: Vec<i32> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                result.push(Backpack { food_calories: current });
                current = vec![];
            }
            continue;
        }

        match parse_i32(line) {
            Ok(num) => current.push(num),
            Err(e) => return Err(e)
        }
    }

    if !current.is_empty() {
        result.push(Backpack { food_calories: current });
    }

    Ok(result)
}

fn find_most_calories(backpacks: Vec<Backpack>) -> Option<i32> {
    backpacks.into_iter().map(|bp| bp.food_calories.into_iter().sum()).max()
}

fn get_top_three_calories(backpacks: Vec<Backpack>) -> i32 {
    let mut totals: Vec<i32> = backpacks.into_iter().map(|bp| bp.food_calories.into_iter().sum()).collect();
    totals.sort();
    totals.reverse();
    totals.into_iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{find_most_calories, get_top_three_calories, parse_input};

    const TEST_INPUT: &str = &"\
    1000\n\
    2000\n\
    3000\n\
    \n\
    4000\n\
    \n\
    5000\n\
    6000\n\
    \n\
    7000\n\
    8000\n\
    9000\n\
    \n\
    10000\n";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected a successful result");

        let backpacks = result.unwrap();
        assert_eq!(backpacks.len(), 5, "Result should have 5 backpacks");
        assert_eq!(backpacks[0].food_calories, vec![1000, 2000, 3000]);
        assert_eq!(backpacks[1].food_calories, vec![4000]);
        assert_eq!(backpacks[2].food_calories, vec![5000, 6000]);
        assert_eq!(backpacks[3].food_calories, vec![7000, 8000, 9000]);
        assert_eq!(backpacks[4].food_calories, vec![10000]);
    }

    #[test]
    fn test_find_most_calories() {
        let result = find_most_calories(parse_input(TEST_INPUT).unwrap());

        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_get_top_three_calories() {
        let result = get_top_three_calories(parse_input(TEST_INPUT).unwrap());

        assert_eq!(result, 45000);
    }
}