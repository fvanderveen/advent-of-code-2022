use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Bounds, Directions, Grid, Point};

pub const DAY23: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let mut game: GameOfElves = input.parse().unwrap();
    
    for _ in 0..10 {
        game.play_round();
    }
    
    let empty_ground = game.get_empty_ground();
    println!("There are {} empty tiles after 10 rounds between the elves.", empty_ground);
}

fn puzzle2(input: &String) {
    let mut game: GameOfElves = input.parse().unwrap();
    
    let stabilize_round = game.get_stabilize_round();
    
    println!("Game stabilizes after {} rounds.", stabilize_round);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Tile {
    #[default]
    Nothing,
    Elf,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Nothing => write!(f, "."),
            Tile::Elf => write!(f, "#")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn apply(&self, point: &Point) -> Point {
        *point + match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0)
        }
    }
    
    fn can_move(&self, point: &Point, tiles: &Grid<Tile>) -> bool {
        let directions = match self {
            Direction::North => Directions::TopAll,
            Direction::East => Directions::RightAll,
            Direction::South => Directions::BottomAll,
            Direction::West => Directions::LeftAll
        };
        tiles.get_adjacent(point, directions).iter().all(|t| *t != Tile::Elf)
    }
}

struct GameOfElves {
    tiles: Grid<Tile>,
    directions: VecDeque<Direction>
}

impl GameOfElves {
    fn new() -> Self {
        let directions = VecDeque::from([Direction::North, Direction::South, Direction::West, Direction::East]);
        Self { tiles: Grid::empty(), directions }
    }
    
    fn play_round(&mut self) -> usize {
        // I am so sure this will not be good enough for part 2... but let's start simple anyway
        
        // Elves without any adjacent elves don't move, so we can skip them in the round
        let cells = self.tiles.entries();
        let elves_to_move: Vec<_> = cells.iter()
            .filter(|(p, t)| *t == Tile::Elf && self.tiles.get_adjacent(p, Directions::All).iter().any(|v| *v == Tile::Elf))
            .collect();
        // Map of destination => source(s)
        let mut move_map: HashMap<Point, Vec<Point>> = HashMap::new();
        'move_loop: for (elf, _) in elves_to_move {
            for direction in &self.directions {
                if direction.can_move(elf, &self.tiles) {
                    let move_to = direction.apply(elf);
                    if let Some(list) = move_map.get_mut(&move_to) {
                        list.push(*elf);
                    } else {
                        move_map.insert(move_to, vec![*elf]);
                    }
                    continue 'move_loop;
                }
            }
        }
        
        // Move the initial preferred direction
        self.directions.rotate_left(1);
        
        let mut moves = 0;
        
        // Move all elves that had a unique target point:
        for (dest, sources) in move_map {
            if sources.len() == 1 {
                self.tiles.set(sources[0], Tile::Nothing);
                self.tiles.set(dest, Tile::Elf);
                moves += 1;
            }
        }
        
        moves
    }

    fn get_elf_bounds(&self) -> Bounds {
        let entries = self.tiles.entries();
        let elves: Vec<_> = entries.iter().filter(|(_, v)| *v == Tile::Elf).map(|(p, _)| p).collect();
        
        let top = elves.iter().map(|p| p.y).min().unwrap();
        let left = elves.iter().map(|p| p.x).min().unwrap();
        let bottom = elves.iter().map(|p| p.y).max().unwrap();
        let right = elves.iter().map(|p| p.x).max().unwrap();
        
        Bounds::from_tlbr(top, left, bottom, right)
    }
    
    fn get_empty_ground(&self) -> usize {
        let bounds = self.get_elf_bounds();
        bounds.points().iter().filter(|p| self.tiles.get(p) != Some(Tile::Elf)).count()
    }
    
    fn get_stabilize_round(&mut self) -> usize {
        let mut rounds = 1; // assuming the first round is not actually stable already 
        while self.play_round() > 0 {
            rounds += 1;
        }
        rounds
    }
}

impl FromStr for GameOfElves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = GameOfElves::new();
        
        let lines: Vec<_> = s.lines().collect();
        for y in 0..lines.len() {
            let chars: Vec<_> = lines[y].chars().collect();
            for x in 0..chars.len() {
                match chars[x] {
                    '.' => game.tiles.set((x,y).try_into()?, Tile::Nothing),
                    '#' => game.tiles.set((x,y).try_into()?, Tile::Elf),
                    _ => return Err(format!("Invalid game char: '{}'", chars[x]))
                }
            }
        }
        
        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day23::GameOfElves;

    #[test]
    fn test_parse() {
        let parse_result: Result<GameOfElves, _> = TEST_INPUT.parse();
        assert!(parse_result.is_ok(), "Expected Ok but was '{}'", parse_result.err().unwrap_or_default());
        
        let game = parse_result.unwrap();
        assert_eq!(7, game.tiles.bounds.width);
        assert_eq!(7, game.tiles.bounds.height);
    }
    
    #[test]
    fn test_play_round() {
        let mut game: GameOfElves = TEST_INPUT.parse().unwrap();
        assert_eq!(TEST_INPUT, format!("{}\n", game.tiles));
        
        game.play_round();
        assert_eq!("\
            .....#...\n\
            ...#...#.\n\
            .#..#.#..\n\
            .....#..#\n\
            ..#.#.##.\n\
            #..#.#...\n\
            #.#.#.##.\n\
            .........\n\
            ..#..#...\
        ", format!("{}", game.tiles).replace(" ", "."));

        game.play_round();
        assert_eq!("\
            ......#....\n\
            ...#.....#.\n\
            ..#..#.#...\n\
            ......#...#\n\
            ..#..#.#...\n\
            #...#.#.#..\n\
            ...........\n\
            .#.#.#.##..\n\
            ...#..#....\
        ", format!("{}", game.tiles).replace(" ", "."));
    }
    
    #[test]
    fn test_get_empty_ground() {
        let mut game: GameOfElves = TEST_INPUT.parse().unwrap();
        assert_eq!(27, game.get_empty_ground());
        
        // Play 10 rounds:
        for _ in 0..10 {
            game.play_round();
        }
        
        assert_eq!(110, game.get_empty_ground());
    }
    
    #[test]
    fn test_get_stabilize_round() {
        let mut game: GameOfElves = TEST_INPUT.parse().unwrap();
        assert_eq!(20, game.get_stabilize_round());
    }
    
    const TEST_INPUT: &str = "\
        ....#..\n\
        ..###.#\n\
        #...#.#\n\
        .#...##\n\
        #.###..\n\
        ##.#.##\n\
        .#..#..\n\
    ";
}