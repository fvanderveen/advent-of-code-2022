use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Bounds, Point};
use crate::util::number::lcm;

pub const DAY24: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let valley: Valley = input.parse().unwrap();
    
    let minutes = valley.shortest_steps_to_exit(0).unwrap();
    println!("The fastest route to the exit takes {} minutes", minutes);
}
fn puzzle2(input: &String) {
    let valley: Valley = input.parse().unwrap();

    let first = valley.shortest_steps_to_exit(0).unwrap();
    let back = valley.shortest_steps_to_entrance(first).unwrap();
    let again = valley.shortest_steps_to_exit(back).unwrap();
    println!("The fastest route to the exit, back, and again takes {} minutes", again);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Blizzard {
    start_location: Point,
    direction: Direction
}

impl Blizzard {
    fn location_at(&self, t: usize, valley: Bounds) -> Point {
        let t = t as isize;
        let width = valley.width as isize;
        let height = valley.height as isize;
        
        let x;
        let y;
        
        match self.direction {
            Direction::Up => {
                x = self.start_location.x;
                y = (self.start_location.y - t).rem_euclid(height);
            }
            Direction::Down => {
                x = self.start_location.x;
                y = (self.start_location.y + t).rem_euclid(height);
            }
            Direction::Left => {
                x = (self.start_location.x - t).rem_euclid(width);
                y = self.start_location.y;
            }
            Direction::Right => {
                x = (self.start_location.x + t).rem_euclid(width);
                y = self.start_location.y;
            }
        }

        (x,y).into()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Valley {
    bounds: Bounds,
    blizzards: Vec<Blizzard>,
    entrance: Point,
    exit: Point
}

impl Valley {
    fn shortest_steps_to_exit(&self, start_time: usize) -> Option<usize> {
        self.shortest_steps_between(start_time, self.entrance, self.exit)
    }

    fn shortest_steps_to_entrance(&self, start_time: usize) -> Option<usize> {
        self.shortest_steps_between(start_time, self.exit, self.entrance)
    }
    
    fn shortest_steps_between(&self, start_time: usize, start: Point, end: Point) -> Option<usize> {
        // Every turn, move the blizzards first. This should give a set of options:
        // - Wait, if our current tile is still empty.
        // - Move (non-diagonally) to an empty tile next to us.
        // Technically might work with shortest-path, though I don't know if there is a way to discard states (or if we need to)
        // For now:
        // - Pick the queue entry with the shortest time spend
        // - Queue whatever moves are possible
        // - Repeat.
        // We might be able to discard states based on the remainder of time spend from the lcm of the width/height of the valley.
        // That lcm gives the point when the blizzards are in the same state again, and still being in a spot you also were 'lcm
        // time ago is useless.
        
        #[derive(Eq, PartialEq)]
        struct State {
            pos: Point,
            time_spent: usize
        }
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.time_spent.cmp(&self.time_spent)
                    .then_with(|| self.pos.cmp(&other.pos))
            }
        }
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let blizzard_time = lcm(self.bounds.width, self.bounds.height);
        
        let mut dists: HashMap<(usize, Point), usize> = HashMap::new();
        let mut queue = BinaryHeap::new();
        
        queue.push(State { pos: start, time_spent: start_time });
        
        while let Some(state) = queue.pop() {
            if state.pos == end {
                // We're done!
                return Some(state.time_spent)
            }
            
            // Check if we're not stuck in a loop:
            if let Some(entry) = dists.get(&(state.time_spent % blizzard_time, state.pos)) {
                if *entry <= state.time_spent {
                    continue;
                }
            }
            dists.insert((state.time_spent % blizzard_time, state.pos), state.time_spent);
            
            // Check what we can actually do:
            let blizzards_at: Vec<_> = self.blizzards.iter().map(|b| b.location_at(state.time_spent + 1, self.bounds)).collect();
            // Can we wait?
            if !blizzards_at.contains(&state.pos) {
                queue.push(State { pos: state.pos, time_spent: state.time_spent + 1 });
            }
            
            let up = state.pos + (0, -1);
            let down = state.pos + (0, 1);
            let left = state.pos + (-1, 0);
            let right = state.pos + (1, 0);
            
            // Can we go up/down/left/right?
            for next in [up, down, left, right] {
                if (end == next || self.bounds.contains(&next)) && !blizzards_at.contains(&next) {
                    queue.push(State { pos: next, time_spent: state.time_spent + 1 });
                }
            }
        }
        
        None
    }
}

impl FromStr for Valley {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The top, left, right, and bottom of the input should be walls ('#'). The top should have one
        // opening ('.') for the entrance, and similarly, the bottom should have one for the exit.
        // All content within the walls is either empty ('.') or a blizzard's starting point with
        // direction (Up '^', left '<', down 'v', or right '>').
        let lines: Vec<_> = s.lines().collect();
        if lines.len() < 3 {
            return Err(format!("Too few lines in input."));
        }
        
        let bounds = Bounds::from_size(lines[0].len() - 2, lines.len() - 2);

        // Handle first and last lines first to find entrance/exit
        let entrance = lines.first()
            .and_then(|l| l.chars().skip(1).take(bounds.width).position(|c| c == '.'))
            .map(|x| Point::from((x as isize, -1)));
        let exit = lines.last()
            .and_then(|l| l.chars().skip(1).take(bounds.width).position(|c| c == '.'))
            .map(|x| Point::from((x as isize, bounds.height as isize)));

        // Handle rest of the lines, ignoring the first and last '#"
        let mut blizzards = vec![];
        for y in 0..bounds.height {
            let line = lines[y+1];
            if !(line.starts_with("#") && line.ends_with("#")) {
                return Err(format!("Invalid line '{}', must start and end with a wall.", line));
            }
            if line.len() != bounds.width + 2 {
                return Err(format!("Line is not the right length, expected {} but was {}: '{}'", bounds.width + 2, line.len(), line));
            }
            
            let chars: Vec<_> = line.chars().skip(1).take(bounds.width).collect();
            for x in 0..chars.len() {
                let loc: Point = (x,y).try_into()?;
                match chars[x] {
                    '.' => { /* nothing needed, empty space */ },
                    '>' => { blizzards.push(Blizzard { start_location: loc, direction: Direction::Right }); }
                    '<' => { blizzards.push(Blizzard { start_location: loc, direction: Direction::Left }); }
                    '^' => { blizzards.push(Blizzard { start_location: loc, direction: Direction::Up }); }
                    'v' => { blizzards.push(Blizzard { start_location: loc, direction: Direction::Down }); }
                    _ => return Err(format!("Invalid valley character: '{}'", chars[x]))
                }
            }
        }
        
        Ok(Self {
            bounds, blizzards, 
            entrance: entrance.ok_or("Could not find an entrance".to_string())?,
            exit: exit.ok_or("Could not find an exit".to_string())?
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day24::{Blizzard, Direction, Valley};
    use crate::util::geometry::{Bounds, Point};

    #[test]
    fn test_parse() {
        let small_res: Result<Valley, _> = SMALL_TEST_INPUT.parse();
        let res:Result<Valley, _> = TEST_INPUT.parse();
        
        assert!(small_res.is_ok(), "Expected OK but was '{}'", small_res.err().unwrap_or_default());
        assert!(res.is_ok(), "Expected OK but was '{}'", res.err().unwrap_or_default());
        
        let small = small_res.unwrap();
        assert_eq!(2, small.blizzards.len());
        assert_eq!(Bounds::from_size(5, 5), small.bounds);
        assert_eq!(Point::from((0, -1)), small.entrance);
        assert_eq!(Point::from((4, 5)), small.exit);
        
        let normal = res.unwrap();
        assert_eq!(19, normal.blizzards.len());
        assert_eq!(Bounds::from_size(6, 4), normal.bounds);
        assert_eq!(Point::from((0, -1)), normal.entrance);
        assert_eq!(Point::from((5, 4)), normal.exit);
    }
    
    #[test]
    fn test_position_at() {
        let bounds = Bounds::from_size(5, 4);
        let mut blizzard = Blizzard { direction: Direction::Right, start_location: (0, 1).into() };
        assert_eq!(Point::from((1, 1)), blizzard.location_at(1, bounds));
        assert_eq!(Point::from((2, 1)), blizzard.location_at(2, bounds));
        assert_eq!(Point::from((0, 1)), blizzard.location_at(5, bounds));
        
        blizzard.direction = Direction::Left;
        assert_eq!(Point::from((4, 1)), blizzard.location_at(1, bounds));
        assert_eq!(Point::from((3, 1)), blizzard.location_at(2, bounds));
        assert_eq!(Point::from((4, 1)), blizzard.location_at(6, bounds));
        
        blizzard.direction = Direction::Up;
        assert_eq!(Point::from((0, 0)), blizzard.location_at(1, bounds));
        assert_eq!(Point::from((0, 3)), blizzard.location_at(2, bounds));
        
        blizzard.direction = Direction::Down;
        assert_eq!(Point::from((0, 2)), blizzard.location_at(1, bounds));
        assert_eq!(Point::from((0, 3)), blizzard.location_at(2, bounds));
        assert_eq!(Point::from((0, 0)), blizzard.location_at(3, bounds));
    }
    
    #[test]
    fn test_shortest_steps() {
        let valley: Valley = TEST_INPUT.parse().unwrap();
        
        assert_eq!(Some(18), valley.shortest_steps_to_exit(0));
        assert_eq!(Some(41), valley.shortest_steps_to_entrance(18));
        assert_eq!(Some(54), valley.shortest_steps_to_exit(41));
    }
    
    const SMALL_TEST_INPUT: &str = "\
        #.#####\n\
        #.....#\n\
        #>....#\n\
        #.....#\n\
        #...v.#\n\
        #.....#\n\
        #####.#\n\
    ";
    
    const TEST_INPUT: &str = "\
        #.######\n\
        #>>.<^<#\n\
        #.<..<<#\n\
        #>v.><>#\n\
        #<^v^^>#\n\
        ######.#\n\
    ";
}