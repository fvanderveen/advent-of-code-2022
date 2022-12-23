use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;
use crate::days::Day;
use crate::util::geometry::{Grid, Point};

pub const DAY17: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let tape = parse_input(input).unwrap();

    let height = Tetris::get_height_after(2022, tape);
    println!("The tetris tower reaches {} height after 2022 drops", height);
}

fn puzzle2(input: &String) {
    let tape = parse_input(input).unwrap();

    let height = Tetris::get_height_after(1_000_000_000_000, tape);
    println!("The tetris tower will be {} block high after 1.000.000.000.000 drops", height);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Movement {
    Left,
    Right,
    Down
}

impl Movement {
    fn translate(&self, p: Point) -> Point {
        match self {
            Movement::Left => p + (-1, 0),
            Movement::Right => p + (1, 0),
            Movement::Down => p + (0, -1)
        }
    }
}

impl TryFrom<char> for Movement {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Movement::Left),
            '>' => Ok(Movement::Right),
            _ => Err(format!("Cannot create Movement from char '{}'", value))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Hash)]
enum Shape {
    // ####
    #[default]
    HorBlock,
    //  #
    // ###
    //  #
    Plus,
    //   #
    //   #
    // ###
    WeirdL,
    // #
    // #
    // #
    // #
    VerBlock,
    // ##
    // ##
    Square
}

impl Shape {
    fn get_points(&self, bottom_left: Point) -> Vec<Point> {
        match self {
            Shape::HorBlock => vec![(0,0).into(), (1,0).into(), (2,0).into(), (3,0).into()] + bottom_left,
            Shape::Plus => vec![(1,0).into(), (0,1).into(), (1,1).into(), (2,1).into(), (1,2).into()] + bottom_left,
            Shape::WeirdL => vec![(0,0).into(), (1,0).into(), (2,0).into(), (2,1).into(), (2,2).into()] + bottom_left,
            Shape::VerBlock => vec![(0,0).into(), (0,1).into(), (0,2).into(), (0,3).into()] + bottom_left,
            Shape::Square => vec![(0,0).into(), (1,0).into(), (0,1).into(), (1,1).into()] + bottom_left
        }
    }
}

struct Tetris {
    blocks: usize,
    formation: Grid<String>,
    move_tape: Vec<Movement>,
    move_loc: usize,
    cave_width: RangeInclusive<isize> // coult be usize, but isize calculates nicer with Point
}

impl Tetris {
    fn create(tape: Vec<Movement>) -> Self {
        Tetris {
            blocks: 0,
            formation: Grid::default(),
            move_tape: tape,
            move_loc: 0,
            cave_width: 0..=6
        }
    }

    fn get_drop_shape(&self) -> Shape {
        match self.blocks % 5 {
            0 => Shape::HorBlock,
            1 => Shape::Plus,
            2 => Shape::WeirdL,
            3 => Shape::VerBlock,
            4 => Shape::Square,
            oops => panic!("{} should not be a result of {} % 5!", oops, self.blocks)
        }
    }

    fn get_drop_loc(&self) -> Point {
        // The drop location (bottom-left) of a new shape is always 2 from the left boundary, and three above the highest point.
        let x = 2;
        let y = (self.formation.bounds.height + 3) as isize;
        (x, y).into()
    }

    fn get_points_from_floor(&self) -> Vec<Point> {
        // To have a proper cache, and I hope this doesn't grow too big and too slow... we need to
        // know the shape of the rocks stacked from the assuming floor. (This is the lowest point
        // where all columns are filled, seen from above.)
        let deepest_point = self.get_top_locs().iter().max().unwrap_or(&0).clone();
        let height = self.formation.bounds.height as isize;
        let floor = height - deepest_point;
        let mut points = vec![];
        for y in 1..=deepest_point {
            for x in self.cave_width.clone() {
                let point: Point = (x,height-y).into();
                if self.formation.get(&point).is_some() {
                    points.push(point + (0, -1 * floor));
                }
            }
        }
        points
    }

    fn get_top_locs(&self) -> Vec<isize> {
        // Calculate for ever row, how far down the top block is:
        let height = self.formation.bounds.height as isize;
        self.cave_width.clone().map(|x| {
            let mut y = height - 1;
            while y > 0 {
                if self.formation.get(&(x,y).into()).is_some() {
                    break;
                }
                y -= 1;
            }
            height - y
        }).collect()
    }

    // Of course step 2 wants to drop 1_000_000_000_000 blocks..
    // Most likely, we will reach some repetition, meaning we'll only need to calculate up to that
    // point; and add the remaining drops' height to that. (Of course just "playing" the game was
    // too easy...)
    // We need to somehow see when we're in a state that we recognize.
    // A state would need to entail (I think): the dropped shape (= dropped block % 5), the tape loc, and the drop position (relative to entry)
    fn get_height_after(drops: usize, tape: Vec<Movement>) -> usize {
        #[derive(Eq, PartialEq, Hash)]
        struct CacheKey {
            drop_shape: Shape,
            tape_pos: usize,
            points: Vec<Point>
        }

        let mut tetris = Self::create(tape.clone());
        let mut cache: HashMap<CacheKey, (usize, usize)> = HashMap::new();

        let repetition_start;
        let repetition_amount;
        let repetition_height;

        // We need to loop until we reach a state that we recognize.
        loop {
            let drop_shape = tetris.get_drop_shape();
            let tape_pos = tetris.move_loc;
            let blocks = tetris.blocks;
            let height = tetris.formation.bounds.height;
            if blocks == drops {
                // We're done before repetition.
                return height;
            }

            let key = CacheKey { drop_shape, tape_pos, points: tetris.get_points_from_floor() };
            if let Some((old_blocks, old_height)) = cache.get(&key) {
                repetition_start = old_blocks;
                repetition_amount = blocks - old_blocks;
                repetition_height = height - old_height;

                println!("Found a repetition {} -> {}, with {} blocks and {} height, next shape = {:?}", repetition_start, blocks, repetition_amount, repetition_height, key.drop_shape);
                break;
            } else {
                cache.insert(key, (blocks, height));
                tetris.drop_block();
            }
        }

        let repetitions = (drops - repetition_start) / repetition_amount;
        let rep_end = repetition_start + (repetitions * repetition_amount);
        let rest = drops - rep_end;

        let rep_height = repetitions * repetition_height;

        // Rest should be relatively small so that we can actually just drop those blocks for simplicity
        // (We will calculate the initial repeat height again with this, but that is fine.)
        let rest_height = Tetris::get_height_after(repetition_start + rest, tape.clone());

        rep_height + rest_height
    }


    fn drop_block(&mut self) -> Point {
        // Dropping a block starts at `get_drop_loc`, and will:
        // Move left/right according to the tape & location if possible.
        // Move the shape down if possible, otherwise it's placed and we're done dropping this block.
        let shape = self.get_drop_shape();
        let mut drop_loc = self.get_drop_loc();

        loop {
            // Get movement from tape:
            let movement = &self.move_tape[self.move_loc];
            self.move_loc = (self.move_loc + 1) % self.move_tape.len();

            let new_bl = movement.translate(drop_loc);
            // We can move if all new points are within bounds:
            if shape.get_points(new_bl).iter().all(|p| self.cave_width.contains(&p.x) && self.formation.get(p).is_none()) {
                drop_loc = new_bl;
            }

            // The next step is to check if we can move a location down:
            let down_loc = Movement::Down.translate(drop_loc);
            if shape.get_points(down_loc).iter().all(|p| p.y >= 0 && self.formation.get(p).is_none()) {
                // All spots are free, continue
                drop_loc = down_loc
            } else {
                // We hit something, mark all current points in the cave and return:
                self.blocks += 1;
                shape.get_points(drop_loc).iter().for_each(|p| self.formation.set(p.clone(), match shape {
                    Shape::HorBlock => "1",
                    Shape::Plus => "2",
                    Shape::WeirdL => "3",
                    Shape::VerBlock => "4",
                    Shape::Square => "5"
                }.to_string()));
                return drop_loc;
            }
        }
    }
}

impl fmt::Display for Tetris {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.formation.bounds.y().rev() {
            write!(f, "|")?;
            for x in self.cave_width.clone() {
                if let Some(_) = self.formation.get(&(x,y).into()) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "|\n")?;
        }
        write!(f, "+")?;
        for _ in self.cave_width.clone() {
            write!(f, "-")?;
        }
        write!(f, "+\n")
    }
}

fn parse_input(input: &str) -> Result<Vec<Movement>, String> {
    input.chars().map(|c| c.try_into()).collect()
}

#[cfg(test)]
mod tests {
    use crate::days::day17::{Movement, parse_input, Tetris};

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected OK result, but got {}", result.err().unwrap_or("unexpected".to_string()));

        let tape = result.unwrap();
        assert_eq!(40, tape.len());
        assert_eq!(Movement::Right, tape[0]);
        assert_eq!(Movement::Right, tape[1]);
        assert_eq!(Movement::Right, tape[2]);
        assert_eq!(Movement::Left, tape[3]);
        assert_eq!(Movement::Left, tape[19]);
    }

    #[test]
    fn test_tetris_drop_block() {
        let tape = parse_input(TEST_INPUT).unwrap();
        let mut tetris = Tetris::create(tape);

        tetris.drop_block();

        assert_eq!(4, tetris.move_loc);
        assert_eq!("\
        |..####.|\n\
        +-------+\n\
        ", format!("{}", tetris));
        assert_eq!(4, tetris.get_points_from_floor().len());

        tetris.drop_block();

        assert_eq!("\
        |...#...|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n\
        ", format!("{}", tetris));
        assert_eq!(9, tetris.get_points_from_floor().len());

        tetris.drop_block();

        assert_eq!("\
        |..#....|\n\
        |..#....|\n\
        |####...|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n\
        ", format!("{}", tetris));

        for _ in 0..7 {
            tetris.drop_block();
        }

        assert_eq!("\
        |....#..|\n\
        |....#..|\n\
        |....##.|\n\
        |##..##.|\n\
        |######.|\n\
        |.###...|\n\
        |..#....|\n\
        |.####..|\n\
        |....##.|\n\
        |....##.|\n\
        |....#..|\n\
        |..#.#..|\n\
        |..#.#..|\n\
        |#####..|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n\
        ", format!("{}", tetris));
    }

    #[test]
    fn test_get_height_after() {
        let tape = parse_input(TEST_INPUT).unwrap();
        assert_eq!(3068, Tetris::get_height_after(2022, tape.clone()));
        assert_eq!(1_514_285_714_288, Tetris::get_height_after(1_000_000_000_000, tape.clone()));
    }

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}