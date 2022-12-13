use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY12: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let map: HeightMap = input.parse().unwrap();
    let steps = map.find_shortest_route().unwrap();

    println!("It takes {} steps to the top!", steps);
}

fn puzzle2(input: &String) {
    let map: HeightMap = input.parse().unwrap();
    let steps = map.find_scenic_route().unwrap();

    println!("Shortest scenic route to the top is {} steps!", steps);
}

struct HeightMap {
    area: Grid<usize>,
    start: Point,
    end: Point
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut area = Grid::default();
        let mut start = None;
        let mut end = None;

        let lines: Vec<_> = input.lines().collect();
        for y in 0..lines.len() {
            let line: Vec<_> = lines[y].chars().collect();
            for x in 0..line.len() {
                let point = (x, y).try_into()?;
                match line[x] {
                    'S' => {
                        start = Some(point);
                        area.set(point, 0);
                    },
                    'E' => {
                        end = Some(point);
                        area.set(point, 25);
                    },
                    'a'..='z' => {
                        area.set(point, (line[x] as usize) - ('a' as usize))
                    },
                    _ => return Err(format!("Invalid height entry '{}'", line[x]))
                }
            }
        }

        match (start, end) {
            (Some(start), Some(end)) => Ok(HeightMap { area, start, end }),
            _ => Err(format!("Could not find start or end point inside input"))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PrioPoint { point: Point, distance: usize, height: usize }
impl Ord for PrioPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| self.point.cmp(&other.point))
    }
}
impl PartialOrd for PrioPoint {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}

impl HeightMap {
    fn find_shortest_route(&self) -> Option<usize> {
        // Hey look. Time for Dijkstra again!
        // We need:
        // - A priority queue to keep tracking the current shortest option
        let mut queue: BinaryHeap<PrioPoint> = BinaryHeap::new();
        // - A map of shortest-path values to a given point
        let mut values: Grid<usize> = Grid::default();
        // - The start added to both
        values.set(self.start, 0);
        queue.push(PrioPoint { point: self.start, distance: 0, height: 0 });

        // Now we just keep handling the point with the shortest current distance
        while let Some(current) = queue.pop() {
            // Have we reached the destination?
            if current.point == self.end {
                return Some(current.distance);
            }

            // Has someone else already reached our point with a shorter distance?
            if let Some(dist) = values.get(&current.point) {
                if current.distance > dist {
                    continue;
                }
            }

            // Otherwise, look for options and push them with new values onto the queue
            for neighbor in self.area.get_adjacent_points(&current.point, Directions::NonDiagonal) {
                // We can step to neighbors that are at most one higher than our current point
                if let Some(val) = self.area.get(&neighbor) {
                    if val > current.height + 1 {
                        continue;
                    }

                    // Check if we haven't already visited said point:
                    if let Some(dist) = values.get(&neighbor) {
                        if dist <= current.distance + 1 {
                            continue;
                        }
                    }

                    // We can add this one to the queue!
                    values.set(neighbor, current.distance + 1);
                    queue.push(PrioPoint { point: neighbor, distance: current.distance + 1, height: val });
                }
            }
        }

        None
    }

    fn find_scenic_route(&self) -> Option<usize> {
        // Hey look. Time for Dijkstra again!
        // We need:
        // - A priority queue to keep tracking the current shortest option
        let mut queue: BinaryHeap<PrioPoint> = BinaryHeap::new();
        // - A map of shortest-path values to a given point
        let mut values: Grid<usize> = Grid::default();
        // - The end added to both
        values.set(self.end, 0);
        queue.push(PrioPoint { point: self.end, distance: 0, height: 25 });

        // Now we just keep handling the point with the shortest current distance
        while let Some(current) = queue.pop() {
            // Have we reached a square of height 0?
            if current.height == 0 {
                return Some(current.distance);
            }

            // Has someone else already reached our point with a shorter distance?
            if let Some(dist) = values.get(&current.point) {
                if current.distance > dist {
                    continue;
                }
            }

            // Otherwise, look for options and push them with new values onto the queue
            for neighbor in self.area.get_adjacent_points(&current.point, Directions::NonDiagonal) {
                // We should only consider neighbors from which we could've reached this point. That is, 1 below or anything above.
                if let Some(val) = self.area.get(&neighbor) {
                    if current.height > val + 1 {
                        continue;
                    }

                    // Check if we haven't already visited said point:
                    if let Some(dist) = values.get(&neighbor) {
                        if dist <= current.distance + 1 {
                            continue;
                        }
                    }

                    // We can add this one to the queue!
                    values.set(neighbor, current.distance + 1);
                    queue.push(PrioPoint { point: neighbor, distance: current.distance + 1, height: val });
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12::{HeightMap};
    use crate::util::geometry::Point;

    #[test]
    fn test_parse_input() {
        let result = TEST_INPUT.parse::<HeightMap>();
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(Point { x: 0, y: 0 }, map.start);
        assert_eq!(Point { x: 5, y: 2 }, map.end);

        assert_eq!(Some(0), map.area.get(&map.start));
        assert_eq!(Some(25), map.area.get(&map.end));
        assert_eq!(Some(0), map.area.get(&(0,1).into()));
        assert_eq!(Some(1), map.area.get(&(1,1).into()));
        assert_eq!(Some(25), map.area.get(&(4,2).into()));
        assert_eq!(Some(24), map.area.get(&(4,1).into()));
        assert_eq!(Some(23), map.area.get(&(5,1).into()));
    }

    #[test]
    fn test_get_shortest_route() {
        let map: HeightMap = TEST_INPUT.parse().unwrap();
        let steps = map.find_shortest_route();

        assert_eq!(Some(31), steps);
    }

    #[test]
    fn test_get_scenic_route() {
        let map: HeightMap = TEST_INPUT.parse().unwrap();
        let steps = map.find_scenic_route();

        assert_eq!(Some(29), steps);
    }

    const TEST_INPUT: &str = "\
        Sabqponm\n\
        abcryxxl\n\
        accszExk\n\
        acctuvwj\n\
        abdefghi\n\
    ";
}