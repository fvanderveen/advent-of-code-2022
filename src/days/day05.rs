use std::fmt;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY5: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let (mut field, moves) = parse_input(input).unwrap();

    for m in moves {
        field.apply_move(&m);
    }

    println!("{:?}", field);
    println!("Containers on top: {}", field.get_items_on_top());
}
fn puzzle2(input: &String) {
    let (mut field, moves) = parse_input(input).unwrap();

    for m in moves {
        field.apply_move_with_order(&m);
    }

    println!("{:?}", field);
    println!("Containers on top: {}", field.get_items_on_top());
}

#[derive(Clone, Eq, PartialEq)]
struct Field {
    stacks: Vec<Vec<char>>
}

impl Field {
    fn apply_move(&mut self, mov: &Move) {
        for _ in 0..mov.count {
            if let Some(val) = self.stacks[mov.from-1].pop() {
                self.stacks[mov.to-1].push(val)
            }
        }
    }

    fn apply_move_with_order(&mut self, mov: &Move) {
        let mut to_move: Vec<char> = vec![];
        for _ in 0..mov.count {
            if let Some(val) = self.stacks[mov.from-1].pop() {
                to_move.push(val);
            }
        }
        for i in (0..to_move.len()).rev() {
            self.stacks[mov.to-1].push(to_move[i]);
        }
    }

    fn get_items_on_top(&self) -> String {
        self.stacks.iter().filter_map(|s| s.last()).map(|c| c.to_string()).collect::<Vec<_>>().concat()
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = self.stacks.iter().map(|s| s.len()).max().unwrap();
        for line in (0..lines).rev() {
            for s in 0..self.stacks.len() {
                let stack = &self.stacks[s];
                if stack.len() > line {
                    let caption = format!("[{}]", stack[line]);
                    f.write_str(caption.as_str())?;
                } else {
                    f.write_str("   ")?;
                }
                f.write_str(if s + 1 < self.stacks.len() { " " } else { "\n" })?;
            }
        }

        for s in 0..self.stacks.len() {
            f.write_str(format!(" {} ", s+1).as_str())?;
            f.write_str(if s + 1 < self.stacks.len() { " " } else { "\n" })?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}

fn parse_input(input: &str) -> Result<(Field, Vec<Move>), String> {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(format!("Expected a field and move set separated by newline, but got {} parts.", parts.len()));
    }

    let field = parse_field(parts[0])?;
    let moves = parts[1].lines().map(|l| parse_move(l)).collect::<Result<Vec<_>, _>>()?;
    Ok((field, moves))
}

fn parse_field(input: &str) -> Result<Field, String> {
    let lines = input.lines().collect::<Vec<_>>();
    // Pre-parse the last line to know the amount of stacks to create and handle
    let label_line = lines.last().unwrap();
    let labels = label_line.split_whitespace().map(|p| parse_usize(p)).collect::<Result<Vec<_>, String>>()?;

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..labels.len() {
        // Ensure we have enough stacks, even for empty ones.
        stacks.push(vec![]);
    }

    for li in (0..lines.len()-1).rev() {
        let line = lines[li];
        for si in 0..labels.len() {
            let offset = si * 4;
            if line.len() < offset + 3 {
                break; // No more line data, ignore rest of columns.
            }
            let item = &line[offset..offset+3].trim();
            if !item.is_empty() {
                // Assume correct format.
                stacks[si].push(item.chars().nth(1).unwrap());
            }
        }
    }

    Ok(Field { stacks })
}

fn parse_move(input: &str) -> Result<Move, String> {
    // move <count> from <from> to <to>
    let parts = input.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 6 {
        return Err(format!("Expected line to have 6 parts, got {}. '{}'", parts.len(), input));
    }

    let count = parse_usize(parts[1])?;
    let from = parse_usize(parts[3])?;
    let to = parse_usize(parts[5])?;
    Ok(Move { count, from, to })
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{Move, parse_field, parse_input, parse_move};

    const TEST_INPUT: &str = "\
        \x20   [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
        \x201   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2\n\
    ";

    #[test]
    fn test_serde_field() {
        let res = parse_field("\
            \x20   [D]    \n\
            [N] [C]    \n\
            [Z] [M] [P]\n\
            \x201   2   3 \n\
        ");
        assert!(res.is_ok());
        let field = res.unwrap();
        assert_eq!(3, field.stacks.len());
        assert_eq!(vec!['Z', 'N'], field.stacks[0]);
        assert_eq!(vec!['M', 'C', 'D'], field.stacks[1]);
        assert_eq!(vec!['P'], field.stacks[2]);

        assert_eq!("\
            \x20   [D]    \n\
            [N] [C]    \n\
            [Z] [M] [P]\n\
            \x201   2   3 \n\
        ", format!("{:?}", field))
    }

    #[test]
    fn test_parse_move() {
        assert_eq!(Ok(Move { count: 3, from: 1, to : 3 }), parse_move("move 3 from 1 to 3"));
        assert_eq!(Ok(Move { count: 1, from: 1, to : 2 }), parse_move("move 1 from 1 to 2"));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());

        let (field, moves) = result.unwrap();
        assert_eq!(3, field.stacks.len());
        assert_eq!(4, moves.len());
    }

    #[test]
    fn test_run_game() {
        let (mut field, moves) = parse_input(TEST_INPUT).unwrap();

        field.apply_move(&moves[0]);
        assert_eq!(vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']], field.stacks);
        field.apply_move(&moves[1]);
        assert_eq!(vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']], field.stacks);
        field.apply_move(&moves[2]);
        assert_eq!(vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']], field.stacks);
        field.apply_move(&moves[3]);
        assert_eq!(vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']], field.stacks);

        assert_eq!("CMZ", field.get_items_on_top());
    }

    #[test]
    fn test_run_game_2() {
        let (mut field, moves) = parse_input(TEST_INPUT).unwrap();

        field.apply_move_with_order(&moves[0]);
        assert_eq!(vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']], field.stacks);
        field.apply_move_with_order(&moves[1]);
        assert_eq!(vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']], field.stacks);
        field.apply_move_with_order(&moves[2]);
        assert_eq!(vec![vec!['M', 'C'], vec![], vec!['P', 'Z', 'N', 'D']], field.stacks);
        field.apply_move_with_order(&moves[3]);
        assert_eq!(vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']], field.stacks);

        assert_eq!("MCD", field.get_items_on_top());
    }
}