use crate::days::Day;
use crate::util::collection::CollectionExtension;

pub const DAY3: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let duplicates_sum: u32 = parse_input(input).unwrap().iter().map(|r| r.get_duplicate_priority_sum().unwrap()).sum();

    println!("Sum of duplicate item priorities: {}", duplicates_sum);
}

fn puzzle2(input: &String) {
    let badge_ids = find_badge_item_ids(&parse_input(input).unwrap());

    let result = badge_ids.into_iter().map(|c| get_item_priority(c).unwrap()).sum::<u32>();
    println!("Sum of badge item types: {}", result);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rucksack {
    compartment_a: Vec<char>,
    compartment_b: Vec<char>
}

impl Rucksack {
    fn get_duplicates(&self) -> Vec<char> {
        self.compartment_a.iter().filter(|c| self.compartment_b.contains(c)).cloned().collect::<Vec<_>>().deduplicate()
    }

    fn get_duplicate_priority_sum(&self) -> Result<u32, String> {
        self.get_duplicates().iter().cloned().map(|c| get_item_priority(c)).sum()
    }

    fn get_item_types(&self) -> Vec<char> {
        self.compartment_a.iter().chain(self.compartment_b.iter()).cloned().collect::<Vec<_>>().deduplicate()
    }
}

fn parse_input(input: &str) -> Result<Vec<Rucksack>, String> {
    input.lines().map(|l| parse_rucksack(l)).collect()
}

fn parse_rucksack(input: &str) -> Result<Rucksack, String> {
    let chars: Vec<_> = input.trim().chars().collect();
    if chars.len() % 2 != 0 {
        return Err(format!("Expected an even length of items for rucksack '{}'", input));
    }
    let split_point = chars.len() / 2;

    Ok(Rucksack {
        compartment_a: chars[..split_point].to_vec(),
        compartment_b: chars[split_point..].to_vec()
    })
}

fn get_item_priority(item: char) -> Result<u32, String> {
    match item {
        'a'..='z' => Ok(item as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(item as u32 - 'A' as u32 + 27),
        _ => Err(format!("Invalid rucksack item: {}", item))
    }
}

fn find_badge_item_ids(rucksacks: &Vec<Rucksack>) -> Vec<char> {
    // The rucksacks are ordered and each set of three correspond to a group.
    // Each group has a badge, that badge should be the only item type shared by all three members
    // of the group.

    let mut badge_ids = vec![];

    for i in (0..rucksacks.len()).step_by(3) {
        let (first, second, third) = (&rucksacks[i], &rucksacks[i+1], &rucksacks[i+2]);
        let result = first.get_item_types().into_iter().filter(|t| second.get_item_types().contains(t) && third.get_item_types().contains(t)).collect::<Vec<_>>();
        if result.len() != 1 {
            eprintln!("Unexpected situation, not exactly one item shared between group: {} ({}, {}, {})",
                      result.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
                      i, i+1, i+2);
        }
        badge_ids.push(result[0]);
    }

    badge_ids
}

#[cfg(test)]
mod tests {
    use crate::days::day03::{find_badge_item_ids, get_item_priority, parse_input};

    const TEST_INPUT: &str = "\
        vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw\n\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);

        assert!(result.is_ok(), "Expected a successful parse");
        let rucksacks = result.unwrap();

        assert_eq!(6, rucksacks.len(), "Expected to have read 6 rucksacks.");
        assert_eq!(12, rucksacks[0].compartment_a.len());
        assert_eq!(12, rucksacks[0].compartment_b.len());
        assert_eq!(vec!['v','J','r','w','p','W','t','w','J','g','W','r'], rucksacks[0].compartment_a);
    }

    #[test]
    fn test_get_item_priority() {
        // To help prioritize item rearrangement, every item type can be converted to a priority:
        //
        // Lowercase item types a through z have priorities 1 through 26.
        // Uppercase item types A through Z have priorities 27 through 52.
        assert_eq!(Ok(1), get_item_priority('a'));
        assert_eq!(Ok(5), get_item_priority('e'));
        assert_eq!(Ok(26), get_item_priority('z'));
        assert_eq!(Ok(27), get_item_priority('A'));
        assert_eq!(Ok(51), get_item_priority('Y'));
    }

    #[test]
    fn test_get_duplicates() {
        let duplicates: Vec<_> = parse_input(TEST_INPUT).unwrap().iter().flat_map(|r| r.get_duplicates()).collect();

        assert_eq!(vec!['p', 'L', 'P', 'v', 't', 's'], duplicates);
    }

    #[test]
    fn test_get_duplicate_priority_sum() {
        let result: Result<u32, _> = parse_input(TEST_INPUT).unwrap().iter().map(|r| r.get_duplicate_priority_sum()).sum();

        assert_eq!(Ok(157), result);
    }

    #[test]
    fn test_find_badge_item_ids() {
        let result = find_badge_item_ids(&parse_input(TEST_INPUT).unwrap());

        assert_eq!(vec!['r', 'Z'], result);
    }
}