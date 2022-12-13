use std::str::FromStr;
use crate::days::Day;
use crate::util::number::{NumberExtensions, parse_usize};

pub const DAY11: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let mut simulation = Simulation::create(parse_input(input).unwrap(), SimulationVersion::Puzzle1);

    let monkey_business = simulation.play_puzzle(20);

    println!("Monkey business level: {}", monkey_business);
}

fn puzzle2(input: &String) {
    let mut simulation = Simulation::create(parse_input(input).unwrap(), SimulationVersion::Puzzle2);

    let monkey_business = simulation.play_puzzle(10000);

    println!("Monkey business level: {}", monkey_business);
}

#[derive(Debug, Eq, PartialEq)]
enum SimulationVersion {
    Puzzle1,
    Puzzle2
}

#[derive(Debug)]
struct Simulation {
    version: SimulationVersion,
    monkeys: Vec<Monkey>
}

impl Simulation {
    fn create(monkeys: Vec<Monkey>, version: SimulationVersion) -> Self {
        Self { monkeys, version }
    }

    fn play_puzzle(&mut self, rounds: usize) -> usize {
        // Chasing all of the monkeys at once is impossible; you're going to have to focus on the
        // two most active monkeys if you want any hope of getting your stuff back.
        // Count the total number of times each monkey inspects items over 20 rounds:
        for _ in 0..rounds {
            self.play_round();
        }

        // The level of monkey business in this situation can be found by multiplying these together:
        let mut monkey_business: Vec<_> = self.monkeys.iter().map(|m| m.inspect_count).collect();
        monkey_business.sort();
        monkey_business.reverse();
        monkey_business[..2].iter().fold(1, |l,r| l*r)
    }

    fn play_round(&mut self) {
        // For every monkey in order, inspect & yeet all items
        let monkey_ids: Vec<_> = self.monkeys.iter().map(|m| m.id).collect();
        for monkey_id in monkey_ids {
            self.inspect_and_yeet(monkey_id);
        }
    }

    fn inspect_and_yeet(&mut self, monkey_id: usize) {
        // To keep values a bit manageable (and this code fast), we can leverage the following maths:
        // - n^y mod n = 0
        // - (a + b) mod n = (a mod n) + (b mod n)
        // From this, we can see that if we find the LCM of the divisors (X) used by the monkeys,
        // we get a value that will yield 0 for all `X mod n` operations of the monkeys. As such
        // we will only need to store the remainder (R) of the new value from that value, as:
        // (X + R) mod n = (X mod n) + (R mod n) = 0 + (R mod n) = R mod n!
        let lcm = self.monkeys.iter().map(|m| m.test.div_by).collect::<Vec<_>>().lcm();

        let mut yeets = vec![];

        if let Some(monkey) = self.monkeys.iter_mut().find(|m| m.id == monkey_id) {
            let items_to_yeet = monkey.items.clone();
            monkey.items.clear();
            for item in items_to_yeet {
                // Increase worry value of item based on operation
                let mut value = monkey.operation.apply(item);
                monkey.inspect_count += 1;
                if self.version == SimulationVersion::Puzzle1 {
                    // Divide by three (rounding down) in relief the item is fine
                    value /= 3;
                }

                value = value % lcm;

                let to_yeet = value.clone();
                // Decide where to yeet it:
                let target = if value % monkey.test.div_by == 0 { monkey.test.true_to } else { monkey.test.false_to };
                // Note: I'd really want to just yeet this to the target monkey, but rust doesn't allow
                // me to get a second mutable monkey in the same scope. Which kinda makes sense, given
                // this being a loop and all..
                yeets.push((target, to_yeet));
            }
        }

        for (target_id, value) in yeets {
            if let Some(monkey) = self.monkeys.iter_mut().find(|m| m.id == target_id)  {
                monkey.items.push(value);
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspect_count: usize
}

#[derive(Debug, Eq, PartialEq)]
enum OperationValue {
    Input,
    Value(usize)
}

impl OperationValue {
    fn get(&self, input: usize) -> usize {
        match self {
            OperationValue::Input => input,
            OperationValue::Value(val) => val.clone()
        }
    }
}

impl FromStr for OperationValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(OperationValue::Input),
            _ => Ok(OperationValue::Value(parse_usize(s)?))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add(OperationValue, OperationValue),
    Multiply(OperationValue, OperationValue)
}

impl Operation {
    fn apply(&self, input: usize) -> usize {
        match self {
            Operation::Add(lhs, rhs) => lhs.get(input) + rhs.get(input),
            Operation::Multiply(lhs, rhs) => lhs.get(input) * rhs.get(input)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Test {
    div_by: usize,
    true_to: usize,
    false_to: usize,
}

fn parse_input(input: &str) -> Result<Vec<Monkey>, String> {
    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();

    // A monkey is defined by 6 lines. (Seriously, not going to make this more generic :joy:)
    // A line with `Monkey {ID}:`
    fn get_monkey_id(line: &str) -> Result<usize, String> {
        if line.starts_with("Monkey ") && line.ends_with(":") {
            Ok(parse_usize(line[7..line.len()-1].trim())?)
        } else {
            Err(format!("Not a monkey identifier: '{}'", line))
        }
    }
    // A line with starting items
    fn get_starting_items(line: &str) -> Result<Vec<usize>, String> {
        if line.starts_with("Starting items: ") {
            Ok(line[16..].split(",").map(|i| parse_usize(i.trim())).collect::<Result<Vec<_>, _>>()?)
        } else {
            Err(format!("Not a starting items line: '{}'", line))
        }
    }
    // A line with an operation (new = {something} {operation} {something})
    fn get_operation(line: &str) -> Result<Operation, String> {
        if !line.starts_with("Operation: new = ") {
            return Err(format!("Not an operation line: '{}'", line))
        }

        let parts: Vec<_> = line[17..].trim().split(" ").collect();
        if parts.len() != 3 {
            return Err(format!("Not an valid operation line: '{}'", line))
        }

        let left: OperationValue = parts[0].parse()?;
        let right = parts[2].parse()?;
        match parts[1] {
            "+" => Ok(Operation::Add(left, right)),
            "*" => Ok(Operation::Multiply(left, right)),
            _ => Err(format!("Invalid operation: '{}'", parts[1]))
        }
    }
    // A line with a test (divisible by ##)
    // A line for the true branch (If true: throw to monkey {ID})
    // A line for the false branch (If false: throw to monkey {ID})
    fn get_test(test_line: &str, true_line: &str, false_line: &str) -> Result<Test, String> {
        let div_by = if test_line.starts_with("Test: divisible by ") { parse_usize(test_line[19..].trim())? } else { return Err(format!("Not a test line '{}'", test_line)); };
        let true_to = if true_line.starts_with("If true: throw to monkey ") { parse_usize(true_line[25..].trim())? } else { return Err(format!("Not a true line '{}'", true_line)); };
        let false_to = if false_line.starts_with("If false: throw to monkey ") { parse_usize(false_line[26..].trim())? } else { return Err(format!("Not a false line '{}'", true_line)); };

        Ok(Test { div_by, true_to, false_to })
    }

    let mut monkeys = vec![];

    for i in (0..lines.len()).step_by(7) {
        if lines.len() < i + 5 {
            break;
        }

        let id = get_monkey_id(lines[i])?;
        let items = get_starting_items(lines[i+1])?;
        let operation = get_operation(lines[i+2])?;
        let test = get_test(lines[i+3], lines[i+4], lines[i+5])?;
        monkeys.push(Monkey { id, items, operation, test, inspect_count: 0 })
    }

    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use crate::days::day11::{Operation, OperationValue, parse_input, Simulation, SimulationVersion, Test};

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        // assert!(result.is_ok());

        let monkeys = result.unwrap();
        assert_eq!(4, monkeys.len());
        assert_eq!(0, monkeys[0].id);
        assert_eq!(vec![79, 98], monkeys[0].items);
        assert_eq!(Operation::Multiply(OperationValue::Input, OperationValue::Value(19)), monkeys[0].operation);
        assert_eq!(Test { div_by: 23, true_to: 2, false_to: 3 }, monkeys[0].test);

        assert_eq!(1, monkeys[1].id);
        assert_eq!(vec![54, 65, 75, 74], monkeys[1].items);
        assert_eq!(Operation::Add(OperationValue::Input, OperationValue::Value(6)), monkeys[1].operation);
        assert_eq!(Test { div_by: 19, true_to: 2, false_to: 0 }, monkeys[1].test);

        assert_eq!(2, monkeys[2].id);
        assert_eq!(vec![79, 60, 97], monkeys[2].items);
        assert_eq!(Operation::Multiply(OperationValue::Input, OperationValue::Input), monkeys[2].operation);
        assert_eq!(Test { div_by: 13, true_to: 1, false_to: 3 }, monkeys[2].test);

        assert_eq!(3, monkeys[3].id);
        assert_eq!(vec![74], monkeys[3].items);
        assert_eq!(Operation::Add(OperationValue::Input, OperationValue::Value(3)), monkeys[3].operation);
        assert_eq!(Test { div_by: 17, true_to: 0, false_to: 1 }, monkeys[3].test);
    }

    #[test]
    fn test_inspect_and_yeet() {
        let mut simulation = Simulation::create(parse_input(TEST_INPUT).unwrap(), SimulationVersion::Puzzle1);
        simulation.inspect_and_yeet(0);
        assert_eq!(0, simulation.monkeys[0].items.len());
        assert_eq!(vec![74, 500, 620], simulation.monkeys[3].items);
    }

    #[test]
    fn test_play_puzzle1_round() {
        let mut simulation = Simulation::create(parse_input(TEST_INPUT).unwrap(), SimulationVersion::Puzzle1);

        simulation.play_round();

        assert_eq!(vec![20, 23, 27, 26], simulation.monkeys[0].items);
        assert_eq!(vec![2080, 25, 167, 207, 401, 1046], simulation.monkeys[1].items);
        assert_eq!(0, simulation.monkeys[2].items.len());
        assert_eq!(0, simulation.monkeys[3].items.len());
    }

    #[test]
    fn test_play_puzzle1() {
        let mut simulation = Simulation::create(parse_input(TEST_INPUT).unwrap(), SimulationVersion::Puzzle1);

        let result = simulation.play_puzzle(20);

        assert_eq!(10605, result);
    }

    #[test]
    fn test_play_puzzle2() {
        let mut simulation = Simulation::create(parse_input(TEST_INPUT).unwrap(), SimulationVersion::Puzzle2);

        let result = simulation.play_puzzle(10000);

        assert_eq!(2713310158, result);
    }

    const TEST_INPUT: &str = "\
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    ";
}