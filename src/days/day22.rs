use std::fmt;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};
use crate::util::parser::Parser;

pub const DAY22: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();
    let password = puzzle.get_password(false);
    
    println!("Our password: {}", password);
}
fn puzzle2(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    let password = puzzle.get_password(true);
    println!("Our password on a cube: {}", password);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#")
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Move {
    Forward(usize),
    Right,
    Left
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Puzzle {
    map: Grid<Tile>,
    moves: Vec<Move>
}

impl Puzzle {
    fn get_password(&self, solve_on_cube: bool) -> isize {
        let start_y = 1;
        let start_x = self.map.bounds.x().find(|x| self.map.has(&(*x, start_y).into())).unwrap();
        let mut pos: Point = (start_x, start_y).into();
        let mut direction = Directions::Right;
        
        for mov in &self.moves {
            match mov {
                Move::Forward(amount) => {
                    for _ in 0..*amount {
                        let (next, dir) = if !solve_on_cube {
                            (self.get_next_in_direction(&direction, &pos), direction)
                        } else {
                            self.get_next_on_cube(&direction, &pos)
                        };
                        match self.map.get(&next) {
                            Some(Tile::Empty) => { pos = next; direction = dir; },
                            Some(Tile::Wall) => { break; },
                            _ => panic!("Halpz! {} going {:?} from {}", next, direction, pos)
                        }
                    }
                },
                Move::Right => {
                    direction = match direction {
                        Directions::Top => Directions::Right,
                        Directions::Right => Directions::Bottom,
                        Directions::Bottom => Directions::Left,
                        Directions::Left => Directions::Top,
                        _ => panic!("Invalid direction?!")
                    }
                },
                Move::Left => {
                    direction = match direction {
                        Directions::Top => Directions::Left,
                        Directions::Right => Directions::Top,
                        Directions::Bottom => Directions::Right,
                        Directions::Left => Directions::Bottom,
                        _ => panic!("Invalid direction?!")
                    }
                }
            }
        }
        
        println!("Ended at {} facing {:?}", pos, direction);
        let facing_value = match direction {
            Directions::Right => 0,
            Directions::Bottom => 1,
            Directions::Left => 2,
            Directions::Top => 3,
            _ => panic!("Invalid direction!?")
        };
        
        pos.y * 1000 + pos.x * 4 + facing_value
    }
    
    fn get_next_in_direction(&self, direction: &Directions, from: &Point) -> Point {
        let next_point: Point = match direction {
            Directions::Top => *from + (0, -1),
            Directions::Right => *from + (1, 0),
            Directions::Bottom => *from + (0, 1),
            Directions::Left => *from + (-1, 0),
            _ => panic!("Invalid direction")
        };
        if self.map.has(&next_point) { return next_point; }
        // If the map does not have the point, we will need to wrap around
        match direction {
            Directions::Top => self.map.bounds.y().rev().find(|y| self.map.has(&(from.x, *y).into())).map(|y| (from.x, y).into()).unwrap(),
            Directions::Right => self.map.bounds.x().find(|x| self.map.has(&(*x, from.y).into())).map(|x| (x, from.y).into()).unwrap(),
            Directions::Bottom => self.map.bounds.y().find(|y| self.map.has(&(from.x, *y).into())).map(|y| (from.x, y).into()).unwrap(),
            Directions::Left => self.map.bounds.x().rev().find(|x| self.map.has(&(*x, from.y).into())).map(|x| (x, from.y).into()).unwrap(),
            _ => panic!("Invalid direction")
        }
    }
    
    fn get_next_on_cube(&self, direction: &Directions, from: &Point) -> (Point, Directions) {
        let next_point: Point = match direction {
            Directions::Top => *from + (0, -1),
            Directions::Right => *from + (1, 0),
            Directions::Bottom => *from + (0, 1),
            Directions::Left => *from + (-1, 0),
            _ => panic!("Invalid direction")
        };
        if self.map.has(&next_point) { return (next_point, *direction); }
        
        println!("Map has not {} (from {}, dir {:?})", next_point, from, direction);
        
        // Now, we could maybe write code that folds the cube; but I found it easier to prepare this for my puzzle input.
        // My map is as follows:
        //  21
        //  3
        // 54
        // 6
        match direction {
            Directions::Top => {
                if (1..=50).contains(&from.x) {
                    // Going up from 5 into the left of 3
                    let next = Point::from((51, 50 + from.x));
                    (next, Directions::Right)
                } else if (51..=100).contains(&from.x) {
                    // Going up from 2 into the left of 6
                    let next = Point::from((1, from.x + 100));
                    (next, Directions::Right)
                } else if (101..=150).contains(&from.x) {
                    // Going up from 1 into the bottom of 6
                    let next = Point::from((from.x - 100, 200));
                    (next, Directions::Top)
                } else {
                    panic!("Cannot walk off the cube at {}", from);
                }
            },
            Directions::Left => {
                if (1..=50).contains(&from.y) {
                    // Left from 2 into the left of 5
                    let next = Point::from((1, 100 + (51 - &from.y)));
                    (next, Directions::Right)
                } else if (51..=100).contains(&from.y) {
                    // Left from 3 into the top of 5
                    let next = Point::from((&from.y - 50, 101));
                    (next, Directions::Bottom)
                } else if (101..=150).contains(&from.y) {
                    // Left from 5 into the left of 2
                    let next = Point::from((51, 51 - (&from.y - 100)));
                    (next, Directions::Right)
                } else if (151..=200).contains(&from.y) {
                    // Left from 6 into the top of 2
                    let next = Point::from((&from.y - 100, 1));
                    (next, Directions::Bottom)
                } else {
                    panic!("Cannot walk off the cube at {}", from);
                }
            },
            Directions::Bottom => {
                if (1..=50).contains(&from.x) {
                    // Down from 6 into the top of 1
                    let next = Point::from((from.x + 100, 1));
                    (next, Directions::Bottom)
                } else if (51..=100).contains(&from.x) {
                    // Down from 4 into the right of 6
                    let next = Point::from((50, 100 + from.x));
                    (next, Directions::Left)
                } else if (101..=150).contains(&from.x) {
                    // Down from 1 into the right of 3
                    let next = Point::from((100, from.x - 50));
                    (next, Directions::Left)
                } else {
                    panic!("Cannot walk off the cube at {}", from);
                }
            },
            Directions::Right => {
                if (1..=50).contains(&from.y) {
                    // Right from 1 to the right of 4
                    let next = Point::from((100, (51 - from.y) + 100));
                    (next, Directions::Left)
                } else if (51..=100).contains(&from.y) {
                    // Right from 3 into the bottom of 1
                    let next = Point::from((from.y + 50, 50));
                    (next, Directions::Top)
                } else if (101..=150).contains(&from.y) {
                    // Right from 4 into the right of 1
                    let next = Point::from((150, 51 - (from.y - 100)));
                    (next, Directions::Left)
                } else if (151..=200).contains(&from.y) {
                    // Right from 6 into the bottom of 4
                    let next = Point::from((from.y - 100, 150));
                    (next, Directions::Top)
                } else {
                    panic!("Cannot walk off the cube at {}", from);
                }
            },
            _ => panic!("Wrong direction!")
        }
    }
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid<Tile> = Grid::empty();
        
        let mut lines: Vec<_> = s.lines().collect();
        let moves_str = lines.pop().unwrap();

        for y in 0..lines.len() {
            let chars: Vec<_> = lines[y].chars().collect();
            for x in 0..chars.len() {
                match chars[x] {
                    '.' => { grid.set((x+1,y+1).try_into()?, Tile::Empty); }
                    '#' => { grid.set((x+1,y+1).try_into()?, Tile::Wall); }
                    c if c.is_whitespace() => { /* ignore whitespace */ },
                    c => return Err(format!("Invalid map char '{}'", c))
                }
            }
        }
        
        let mut moves = vec![];
        // Parse moves.
        let mut parser = Parser::new(moves_str);
        while !parser.is_exhausted() {
            if let Ok(steps) = parser.usize() {
                moves.push(Move::Forward(steps));
            } else if let Ok(()) = parser.literal("R") {
                moves.push(Move::Right);
            } else if let Ok(()) = parser.literal("L") {
                moves.push(Move::Left);
            } else {
                return Err(format!("Could not match a number, L, or R."))
            }
        }
        
        Ok(Puzzle {
            map: grid, moves
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day22::Puzzle;
    use crate::util::geometry::{Directions, Point};

    #[test]
    fn test_parse() {
        let puzzle_result: Result<Puzzle, _> = TEST_INPUT.parse();
        assert!(puzzle_result.is_ok(), "Expected OK but got: {}", puzzle_result.err().unwrap_or_default());
        
        let puzzle = puzzle_result.unwrap();
        assert_eq!(12, puzzle.map.bounds.height);
        assert_eq!(16, puzzle.map.bounds.width);
    }
    
    #[test]
    fn test_get_next_in_direction() {
        let puzzle: Puzzle = TEST_INPUT.parse().unwrap();
        assert_eq!(Point::from((5, 8)), puzzle.get_next_in_direction(&Directions::Top, &(5, 5).into()));
        assert_eq!(Point::from((5, 7)), puzzle.get_next_in_direction(&Directions::Top, &(5, 8).into()));
        assert_eq!(Point::from((12, 7)), puzzle.get_next_in_direction(&Directions::Left, &(1, 7).into()));
        assert_eq!(Point::from((9, 4)), puzzle.get_next_in_direction(&Directions::Right, &(12, 4).into()));
    }
    
    #[test]
    fn test_get_password() {
        let puzzle: Puzzle = TEST_INPUT.parse().unwrap();
        assert_eq!(6032, puzzle.get_password(false));
    }
    
    const TEST_INPUT: &str = "\
        \x20       ...#\n\
        \x20       .#..\n\
        \x20       #...\n\
        \x20       ....\n\
        ...#.......#\n\
        ........#...\n\
        ..#....#....\n\
        ..........#.\n\
        \x20       ...#....\n\
        \x20       .....#..\n\
        \x20       .#......\n\
        \x20       ......#.\n\
        \n\
        10R5L5R10L4R5L5\n\
    ";
}