use crate::days::Day;
use crate::util::number::parse_isize;

pub const DAY10: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let program = parse_input(input).unwrap();
    let signals = execute_for_puzzle_1(&program);
    let signal_sum = signals.iter().take(6).sum::<isize>();

    println!("Sum of 6 target signals = {}", signal_sum);
}

fn puzzle2(input: &String) {
    let program = parse_input(input).unwrap();
    let pixels = execute_for_puzzle_2(&program);

    println!("Puzzle 2; screen output:");
    for line in pixels {
        for pixel in line {
            print!("{}", pixel);
        }
        print!("\n");
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Noop,
    Add(isize),
}

fn parse_input(input: &str) -> Result<Vec<Operation>, String> {
    input.lines().map(|l|
        if l == "noop" {
            Ok(Operation::Noop)
        } else if l.starts_with("addx ") {
            Ok(Operation::Add(parse_isize(&l[5..])?))
        } else {
            Err(format!("Invalid operation: '{}'", l))
        }
    ).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct CpuState {
    reg_x: isize,
    cycle_count: usize,
}

fn execute_program<C>(program: &Vec<Operation>, mut cycle_callback: C) -> CpuState
    where C: FnMut(CpuState) -> ()
{
    // The CPU has a single register, X, which starts with the value 1. It supports only two instructions:
    //
    // addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
    // noop takes one cycle to complete. It has no other effect.
    let mut reg_x: isize = 1;
    let mut cycle_count: usize = 0;

    for operation in program {
        match operation {
            Operation::Noop => {
                cycle_count += 1;
                cycle_callback(CpuState { reg_x, cycle_count });
            }
            Operation::Add(value) => {
                cycle_count += 1;
                cycle_callback(CpuState { reg_x, cycle_count });
                cycle_count += 1;
                cycle_callback(CpuState { reg_x, cycle_count });
                reg_x += value;
            }
        }
    }

    CpuState { reg_x, cycle_count }
}

fn execute_for_puzzle_1(program: &Vec<Operation>) -> Vec<isize> {
    let mut results = vec![];
    execute_program(program, |state| {
        if state.cycle_count >= 20 && (state.cycle_count - 20) % 40 == 0 {
            results.push(state.cycle_count as isize * state.reg_x);
        }
    });
    results
}

fn execute_for_puzzle_2(program: &Vec<Operation>) -> [[char;40];6] {
    let mut results = [['.'; 40]; 6];

    // It seems like the X register controls the horizontal position of a sprite. Specifically, the
    // sprite is 3 pixels wide, and the X register sets the horizontal position of the middle of that
    // sprite.
    // (In this system, there is no such thing as "vertical position": if the sprite's horizontal
    //  position puts its pixels where the CRT is currently drawing, then those pixels will be drawn.)
    execute_program(program, |state| {
        // Out of bounds of the screen.
        if state.cycle_count > 240 {
            return;
        }

        let line = (state.cycle_count-1) / 40;
        let column = (state.cycle_count-1) % 40;
        let value = ((column as isize - 1)..=(column as isize + 1)).contains(&state.reg_x);
        results[line][column] = if value { '#' } else { '.' };
    });

    results
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{execute_for_puzzle_1, execute_for_puzzle_2, Operation, parse_input};

    #[test]
    fn test_parse_input() {
        assert_eq!(Ok(vec![Operation::Noop, Operation::Add(3), Operation::Add(-5)]), parse_input(
            "\
                noop\n\
                addx 3\n\
                addx -5\n\
            "
        ));
    }

    #[test]
    fn test_execute_for_puzzle_1() {
        let program = parse_input(TEST_INPUT).unwrap();
        let result = execute_for_puzzle_1(&program);

        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], result);
    }

    #[test]
    fn test_execute_for_puzzle_2() {
        let program = parse_input(TEST_INPUT).unwrap();
        let result = execute_for_puzzle_2(&program);

        assert_eq!([
            ['#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.'],
            ['#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.'],
            ['#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.'],
            ['#','#','#','#','#','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.'],
            ['#','#','#','#','#','#','.','.','.','.','.','.','#','#','#','#','#','#','.','.','.','.','.','.','#','#','#','#','#','#','.','.','.','.','.','.','#','#','#','#'],
            ['#','#','#','#','#','#','#','.','.','.','.','.','.','.','#','#','#','#','#','#','#','.','.','.','.','.','.','.','#','#','#','#','#','#','#','.','.','.','.','.']
        ], result)
    }

    const TEST_INPUT: &str = "\
        addx 15\n\
        addx -11\n\
        addx 6\n\
        addx -3\n\
        addx 5\n\
        addx -1\n\
        addx -8\n\
        addx 13\n\
        addx 4\n\
        noop\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx -35\n\
        addx 1\n\
        addx 24\n\
        addx -19\n\
        addx 1\n\
        addx 16\n\
        addx -11\n\
        noop\n\
        noop\n\
        addx 21\n\
        addx -15\n\
        noop\n\
        noop\n\
        addx -3\n\
        addx 9\n\
        addx 1\n\
        addx -3\n\
        addx 8\n\
        addx 1\n\
        addx 5\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        addx -36\n\
        noop\n\
        addx 1\n\
        addx 7\n\
        noop\n\
        noop\n\
        noop\n\
        addx 2\n\
        addx 6\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        addx 1\n\
        noop\n\
        noop\n\
        addx 7\n\
        addx 1\n\
        noop\n\
        addx -13\n\
        addx 13\n\
        addx 7\n\
        noop\n\
        addx 1\n\
        addx -33\n\
        noop\n\
        noop\n\
        noop\n\
        addx 2\n\
        noop\n\
        noop\n\
        noop\n\
        addx 8\n\
        noop\n\
        addx -1\n\
        addx 2\n\
        addx 1\n\
        noop\n\
        addx 17\n\
        addx -9\n\
        addx 1\n\
        addx 1\n\
        addx -3\n\
        addx 11\n\
        noop\n\
        noop\n\
        addx 1\n\
        noop\n\
        addx 1\n\
        noop\n\
        noop\n\
        addx -13\n\
        addx -19\n\
        addx 1\n\
        addx 3\n\
        addx 26\n\
        addx -30\n\
        addx 12\n\
        addx -1\n\
        addx 3\n\
        addx 1\n\
        noop\n\
        noop\n\
        noop\n\
        addx -9\n\
        addx 18\n\
        addx 1\n\
        addx 2\n\
        noop\n\
        noop\n\
        addx 9\n\
        noop\n\
        noop\n\
        noop\n\
        addx -1\n\
        addx 2\n\
        addx -37\n\
        addx 1\n\
        addx 3\n\
        noop\n\
        addx 15\n\
        addx -21\n\
        addx 22\n\
        addx -6\n\
        addx 1\n\
        noop\n\
        addx 2\n\
        addx 1\n\
        noop\n\
        addx -10\n\
        noop\n\
        noop\n\
        addx 20\n\
        addx 1\n\
        addx 2\n\
        addx 2\n\
        addx -6\n\
        addx -11\n\
        noop\n\
        noop\n\
        noop\n\
    ";
}