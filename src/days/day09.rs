use std::cmp;
use std::collections::HashMap;
use crate::days::Day;
use crate::util::geometry::{Bounds, Directions, Grid, Point};
use crate::util::number::parse_usize;

pub const DAY9: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let steps = parse_input(input).unwrap();
    let mut sim = Simulation::new(2);
    steps.iter().for_each(|s| sim.apply_step(s));
    let visited_spots = sim.get_tail_position_count();
    println!("Tail (2 knots) visited {} different spots in the simulation", visited_spots);
}

fn puzzle2(input: &String) {
    let steps = parse_input(input).unwrap();
    let mut sim = Simulation::new(10);
    steps.iter().for_each(|s| sim.apply_step(s));
    let visited_spots = sim.get_tail_position_count();
    println!("Tail (10 knots) visited {} different spots in the simulation", visited_spots);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up, Right, Down, Left
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Step {
    direction: Direction,
    amount: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Simulation {
    knots: Vec<Point>,
    tail_history: Grid<bool>
}

impl Simulation {
    fn new(knot_count: usize) -> Self {
        Simulation {
            knots: (0..knot_count).map(|_| (0, 0).into()).collect(),
            tail_history: Grid::new(HashMap::from([((0, 0).into(), true)]))
        }
    }

    fn apply_step(&mut self, step: &Step) {
        // We move the head first. If the head ends up no longer touching (in any direction) the tail;
        // we'll need to move the tail.
        for _ in 0..step.amount {
            for i in  0..self.knots.len() {
                if i == 0 {
                    let knot = self.knots.get_mut(i).unwrap();
                    // Head, we'll just be moving that:
                    match step.direction {
                        Direction::Up => { knot.y -= 1 },
                        Direction::Right => { knot.x += 1 },
                        Direction::Down => { knot.y += 1 },
                        Direction::Left => { knot.x -= 1 },
                    }
                } else {
                    let knot = self.knots[i];
                    // Next knot, follow previous if needed:
                    let previous = self.knots[i-1];
                    let points = knot.get_points_around(Directions::All);
                    if previous == knot || points.contains(&previous) { continue; } // still close enough.
                    // Move the tail towards the head. After moving the tail will always be directly next to the
                    // head (so not diagonal). We can easily find this point by finding the overlapping point
                    // of all points around the tail, and the non-diagonal ones around the head.
                    let possible_targets = previous.get_points_around(Directions::NonDiagonal);
                    if let Some(new_pos) = points.iter().find(|p| possible_targets.contains(p)) {
                        self.knots[i] = new_pos.clone();
                    } else {
                        // If the NonDiagonal are not matching, we should be able to find a diagonal one.
                        // This might be needed when using multiple knots, as knots can now move diagonally
                        // after a chain of moves.
                        let diagonal_targets = previous.get_points_around(Directions::Diagonal);
                        if let Some(new_pos) = points.iter().find(|p| diagonal_targets.contains(p)) {
                            self.knots[i] = new_pos.clone();
                        } else {
                            panic!("No overlap to move {:?} closer to {:?}. ({:?}) vs ({:?} and {:?})", knot, previous, points, possible_targets, diagonal_targets)
                        }
                    }
                }
            }

            // Tail, keep track of visited places:
            self.tail_history.set(self.knots.last().unwrap().clone(), true);
        }
    }

    fn get_tail_position_count(&self) -> usize {
        self.tail_history.values().into_iter().filter(|v| v.clone()).count()
    }

    fn _print_tail_history(&self) {
        print!("{}\n", "-".repeat(self.tail_history.bounds.width));
        for y in self.tail_history.bounds.y() {
            for x in self.tail_history.bounds.x() {
                print!("{}", if self.tail_history.get(&(x, y).into()).unwrap_or_default() { "#" } else { "." });
            }
            print!("\n");
        }
        print!("{}\n", "-".repeat(self.tail_history.bounds.width));
    }

    fn _print_state(&self) {
        let top = cmp::min(self.tail_history.bounds.top, self.knots.iter().map(|p| p.y).min().unwrap_or(0));
        let left = cmp::min(self.tail_history.bounds.left, self.knots.iter().map(|p| p.x).min().unwrap_or(0));
        let bottom = cmp::max(self.tail_history.bounds.bottom(), self.knots.iter().map(|p| p.y).max().unwrap_or(0));
        let right = cmp::max(self.tail_history.bounds.right(), self.knots.iter().map(|p| p.x).max().unwrap_or(0));
        let bounds = Bounds::from_tlbr(top, left, bottom, right);

        for y in bounds.y() {
            for x in bounds.x() {
                let hist_token = if self.tail_history.get(&(x, y).into()) == Some(true) { "#" } else { "." };
                let point = Point::from((x, y));

                if self.knots[0] == point {
                    print!("H");
                } else {
                    let mut result: Option<usize> = None;
                    for i in 1..self.knots.len() {
                        if self.knots[i] == point {
                            result = Some(i);
                            break;
                        }
                    }
                    if let Some(val) = result {
                        print!("{}", val)
                    } else {
                        print!("{}", hist_token)
                    }
                }
            }
            print!("\n");
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Step>, String> {
    input.lines().map(|l| if let [left, right] = l.split(" ").collect::<Vec<_>>()[..] {
        let amount = parse_usize(right)?;
        match left {
            "U" => Ok(Step{ direction: Direction::Up, amount }),
            "R" => Ok(Step{ direction: Direction::Right, amount }),
            "D" => Ok(Step{ direction: Direction::Down, amount }),
            "L" => Ok(Step{ direction: Direction::Left, amount }),
            _ => Err(format!("Invalid direction value: {}", left))
        }
    } else {
        Err(format!("Invalid direction line: '{}'", l))
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{Direction, parse_input, Simulation, Step};

    const TEST_INPUT: &str = "\
        R 4\n\
        U 4\n\
        L 3\n\
        D 1\n\
        R 4\n\
        D 1\n\
        L 5\n\
        R 2\n\
    ";

    const TEST_INPUT_2: &str = "\
        R 5\n\
        U 8\n\
        L 8\n\
        D 3\n\
        R 17\n\
        D 10\n\
        L 25\n\
        U 20\n\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());
        let steps = result.unwrap();
        assert_eq!(8, steps.len());
        assert_eq!(Step { direction: Direction::Up, amount: 4 }, steps[1]);
    }

    #[test]
    fn test_apply_step() {
        let mut sim = Simulation::new(2);
        sim.apply_step(&Step { direction: Direction::Right, amount: 4 });
        assert_eq!(sim.knots[0], (4, 0).into());
        assert_eq!(sim.knots[1], (3, 0).into());

        sim.apply_step(&Step { direction: Direction::Up, amount: 4 });
        assert_eq!(sim.knots[0], (4, -4).into());
        assert_eq!(sim.knots[1], (4, -3).into());
    }

    #[test]
    fn test_get_tail_position_count() {
        let mut sim = Simulation::new(2);
        let steps = parse_input(TEST_INPUT).unwrap();
        steps.iter().for_each(|s| sim.apply_step(s));
        sim._print_tail_history();
        assert_eq!(13, sim.get_tail_position_count());

        sim = Simulation::new(10);
        steps.iter().for_each(|s| sim.apply_step(s));
        sim._print_tail_history();
        assert_eq!(1, sim.get_tail_position_count());

        let steps2 = parse_input(TEST_INPUT_2).unwrap();
        sim = Simulation::new(10);
        steps2.iter().for_each(|s| sim.apply_step(s));
        sim._print_tail_history();
        assert_eq!(36, sim.get_tail_position_count());
    }
}