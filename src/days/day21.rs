use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY21: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let monkeys = parse_input(input).unwrap();
    let root = get_monkey_number(&monkeys, &"root").unwrap();
    
    println!("The root monkey yells: {}", root);
}

fn puzzle2(input: &String) {
    let mut monkeys = parse_input(input).unwrap();
    let root = get_monkey(&"root", &monkeys).unwrap();
    
    let human_number = find_humn_number(&monkeys);
    
    let (left, right) = root.operation.get_sides();
    // Validation
    monkeys.iter_mut().find(|m| m.name == "humn").unwrap().operation = Operation::Yell(human_number);
    
    println!("After yelling {}: {} vs {}", human_number, get_monkey_number(&monkeys, &left).unwrap(), get_monkey_number(&monkeys, &right).unwrap());
    
    println!("The human needs to yell: {}", human_number);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Yell(isize),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String)
}

impl Operation {
    fn parse(parser: &mut Parser) -> Result<Self, String> {
        if let Ok(val) = parser.isize() {
            return Ok(Operation::Yell(val))
        }
        
        // Match a name, operator, and another name
        let name1 = parser.str(4)?;
        let op = parser.str(1)?;
        let name2 = parser.str(4)?;
        
        match op.as_str() {
            "+" => Ok(Operation::Add(name1, name2)),
            "-" => Ok(Operation::Subtract(name1, name2)),
            "*" => Ok(Operation::Multiply(name1, name2)),
            "/" => Ok(Operation::Divide(name1, name2)),
            _ => Err(format!("Invalid operator '{}'", op))
        }
    }
    
    fn get_sides(&self) -> (String, String) {
        match self {
            Operation::Yell(_) => panic!("Yell has no sides!"),
            Operation::Add(left, right) => (left.clone(), right.clone()),
            Operation::Subtract(left, right) => (left.clone(), right.clone()),
            Operation::Multiply(left, right) => (left.clone(), right.clone()),
            Operation::Divide(left, right) => (left.clone(), right.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Monkey {
    name: String,
    operation: Operation
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        let name = parser.str(4)?;
        parser.literal(":")?;
        let operation = Operation::parse(&mut parser)?;
        Ok(Monkey { name, operation })
    }
}

fn parse_input(input: &str) -> Result<Vec<Monkey>, String> {
    input.lines().map(|l| l.parse()).collect()
}

fn get_monkey_number(monkeys: &Vec<Monkey>, target: &str) -> Result<isize, String> {
    if let Some(monkey) = get_monkey(target, monkeys) {
        match &monkey.operation {
            Operation::Yell(val) => Ok(*val),
            Operation::Add(left, right) => Ok(get_monkey_number(monkeys, &left)? + get_monkey_number(monkeys, &right)?),
            Operation::Subtract(left, right) => Ok(get_monkey_number(monkeys, &left)? - get_monkey_number(monkeys, &right)?),
            Operation::Multiply(left, right) => Ok(get_monkey_number(monkeys, &left)? * get_monkey_number(monkeys, &right)?),
            Operation::Divide(left, right) => Ok(get_monkey_number(monkeys, &left)? / get_monkey_number(monkeys, &right)?),
        }
    } else {
        Err(format!("No monkey named '{}'", target))
    }
}

fn get_monkey<'a>(target: &str, monkeys: &'a Vec<Monkey>) -> Option<&'a Monkey> {
    monkeys.iter().find(|m| m.name == target)
}

fn depends_on_humn(target: &str, monkeys: &Vec<Monkey>) -> bool {
    if target == "humn" { return true };
    if let Some(monkey) = get_monkey(target, monkeys) {
        match &monkey.operation {
            Operation::Yell(_) => false,
            Operation::Add(l, r) => depends_on_humn(l, monkeys) || depends_on_humn(r, monkeys), 
            Operation::Subtract(l, r) => depends_on_humn(l, monkeys) || depends_on_humn(r, monkeys),
            Operation::Multiply(l, r) => depends_on_humn(l, monkeys) || depends_on_humn(r, monkeys),
            Operation::Divide(l, r) => depends_on_humn(l, monkeys) || depends_on_humn(r, monkeys),
        }
    } else {
        false
    }
}

fn find_humn_number(monkeys: &Vec<Monkey>) -> isize {
    // The 'humn' "monkey" is the player
    // The 'root' monkeys operator is actually equality
    
    // Without brute forcing probably a lot of numbers...
    // We should be able to figure out from root, which side depends on humn. The other side is a known number.
    // For each operation in the tree:
    // - Find out the humn side, compute the other, compute what the humn side needs to be to get the right result
    
    let root = get_monkey(&"root", monkeys).unwrap();
    // Shortcut: root is add in both the example and my data.
    let Operation::Add(left, right) = &root.operation else { panic!("Not the right root type!"); };
    let (human_side, other_side) = if depends_on_humn(left, monkeys) { (left, right) } else { (right, left) };
    let result = get_monkey_number(monkeys, other_side).unwrap();

    get_human_input_to_equal(human_side, result, monkeys)
}

fn get_human_input_to_equal(monkey: &str, target: isize, monkeys: &Vec<Monkey>) -> isize {
    if monkey == "humn" { return target; }
    
    match &get_monkey(monkey, monkeys).unwrap().operation {
        Operation::Yell(_) => panic!("Human side resulted in a yelling monkey?!"),
        Operation::Add(left, right) => {
            let (human_side, other_side) = if depends_on_humn(left, monkeys) { (left, right) } else { (right, left) };
            let new_target = target - get_monkey_number(monkeys, other_side).unwrap();
            return get_human_input_to_equal(human_side, new_target, monkeys);
        }
        Operation::Subtract(left, right) => {
            // 5 - 3 = 2 has different solving for which side is human.
            // 5 => 2 + 3
            // 3 => 5 - 2
            return if depends_on_humn(left, monkeys) {
                // X - A = B => X = A + B
                let new_target = get_monkey_number(monkeys, right).unwrap() + target;
                get_human_input_to_equal(left, new_target, monkeys)
            } else {
                // A - X = B => X = A - B
                let new_target = get_monkey_number(monkeys, left).unwrap() - target;
                get_human_input_to_equal(right, new_target, monkeys)
            }
        }
        Operation::Multiply(left, right) => {
            let (human_side, other_side) = if depends_on_humn(left, monkeys) { (left, right) } else { (right, left) };
            let new_target = target / get_monkey_number(monkeys, other_side).unwrap();
            return get_human_input_to_equal(human_side, new_target, monkeys);
        }
        Operation::Divide(left, right) => {
            // 10 / 2 = 5 has different solving for which side is human
            // 10 => 2 * 5
            // 2 => 10 / 5
            return if depends_on_humn(left, monkeys) {
                // X / A = B => X = A * B
                let new_target = get_monkey_number(monkeys, right).unwrap() * target;
                get_human_input_to_equal(left, new_target, monkeys)
            } else {
                // A / X = B => X = A / B
                let new_target = get_monkey_number(monkeys, left).unwrap() / target;
                get_human_input_to_equal(right, new_target, monkeys)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day21::{find_humn_number, get_monkey_number, Monkey, Operation, parse_input};

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected OK but got: '{}'", result.err().unwrap_or_default());
        
        let monkeys = result.unwrap();
        assert_eq!(Monkey { name: "root".to_string(), operation: Operation::Add("pppw".to_string(), "sjmn".to_string()) }, monkeys[0]);
    }
    
    #[test]
    fn test_get_monkey_number() {
        let monkeys = parse_input(TEST_INPUT).unwrap();
        assert_eq!(Ok(152), get_monkey_number(&monkeys, &"root"));
    }
    
    #[test]
    fn test_find_humn_number() {
        let monkeys = parse_input(TEST_INPUT).unwrap();
        assert_eq!(301, find_humn_number(&monkeys));
    }
    
    const TEST_INPUT: &str = "\
        root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32\n\
    ";
}