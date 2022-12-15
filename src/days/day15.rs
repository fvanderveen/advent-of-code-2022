use std::ops::{RangeInclusive};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::Point;
use crate::util::parser::Parser;

pub const DAY15: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let sensors = parse_input(input).unwrap();

    let coverage = get_coverage_on_line(&sensors, 2_000_000);
    println!("There are {} spots on line 2.000.000 that cannot have a beacon", coverage);
}

fn puzzle2(input: &String) {
    let sensors = parse_input(input).unwrap();

    // Crossing fingers
    let point = find_sensor_location(&sensors, 0..=4_000_000).unwrap();
    println!("Found where the beacon has to be: {}, result = {}", point, point.x * 4_000_000 + point.y);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sensor {
    location: Point,
    beacon: Point,
    area: ManhattanArea
}

impl Sensor {

}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Sensor at x=#, y=##: closest beacon is at x=##, y=##
        let mut parser = Parser::new(s.to_string());
        parser.literal("Sensor at x=")?;
        let sx = parser.isize()?;
        parser.literal(",")?;
        parser.literal("y=")?;
        let sy = parser.isize()?;
        parser.literal(":")?;
        parser.literal("closest beacon is at x=")?;
        let bx = parser.isize()?;
        parser.literal(",")?;
        parser.literal("y=")?;
        let by = parser.isize()?;

        let location = (sx, sy).into();
        let beacon = (bx, by).into();
        Ok(Sensor { location, beacon, area: ManhattanArea { center: location, length: location.manhattan_distance(&beacon) } })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ManhattanArea {
    center: Point,
    length: isize
}

impl ManhattanArea {
    fn get_cols_for_line(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let dist_y = (self.center.y - y).abs();
        let dist_x = self.length - dist_y;
        if dist_x < 0 {
            None
        } else {
            Some((self.center.x - dist_x)..=(self.center.x + dist_x))
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self.center.manhattan_distance(point) <= self.length
    }

    fn get_points_around(&self) -> Vec<Point> {
        let mut points = vec![];
        // We can start above this area (y - length - 1) and walk around it?
        let distance = self.length + 1;
        for i in 0..distance {
            points.push(self.center + (i, -(distance - i)));  // top (x,y-distance) -> right (x+distance,y)
            points.push(self.center + ((distance - i), i));   // right (x+distance,y) -> bottom (x,y+distance)
            points.push(self.center + (-i, distance - i));    // bottom (x,y+distance) -> left (x-distance,y)
            points.push(self.center + (-(distance - i), -i)); // left (x-distance,y) -> top (x,y-distance)
        }

        points
    }
}

fn parse_input(input: &str) -> Result<Vec<Sensor>, String> {
    input.lines().map(|l| l.parse()).collect()
}

fn find_sensor_location(sensors: &Vec<Sensor>, cap: RangeInclusive<isize>) -> Option<Point> {
    // For ever line in cap
    // See if there are options
    // There should be, according to the puzzle, exactly one...
    // I hope looping 4.000.000 times (worst case) is fast enough :D
    // It is ... not!
    // Second idea: Since there can only be one spot, we can be sure it's a point that is
    // around one of the sensor areas. Probably walking those points would be a lot faster.
    for sensor in sensors {
        if let Some(p) = sensor.area.get_points_around().iter()
                .filter(|p| cap.contains(&p.x) && cap.contains(&p.y))
                .find(|p| !sensors.iter().any(|s| s.area.contains(p))) {
            return Some(p.clone());
        }
    }

    None
}

fn get_coverage_on_line(sensors: &Vec<Sensor>, line: isize) -> usize {
    #[derive(Clone)]
    struct Coverage {
        range: RangeInclusive<isize>,
        is_overlap: bool,
    }

    let mut coverages: Vec<Coverage> = vec![];

    fn overlap_range(range: &RangeInclusive<isize>, other: &RangeInclusive<isize>) -> RangeInclusive<isize> {
        let range_start = range.start().clone();
        let range_end = range.end().clone();
        let other_start = other.start().clone();
        let other_end = other.end().clone();

        other_start.max(range_start)..=other_end.min(range_end)
    }

    for sensor in sensors {
        if let Some(xs) = sensor.area.get_cols_for_line(line) {
            for coverage in coverages.clone() {
                let overlap = overlap_range(&xs, &coverage.range);
                if !overlap.is_empty() {
                    coverages.push(Coverage { range: overlap, is_overlap: !coverage.is_overlap });
                }
            }

            coverages.push(Coverage { range: xs.clone(), is_overlap: false });
        }
    }

    let mut result = 0;
    for coverage in coverages {
        if coverage.is_overlap {
            result -= coverage.range.count();
        } else {
            result += coverage.range.count();
        }
    }

    let mut beacons_on_line: Vec<_> = sensors.iter().map(|s| s.beacon).filter(|b| b.y == line).collect();
    beacons_on_line.dedup();

    result - beacons_on_line.len()
}

#[cfg(test)]
mod tests {
    use crate::days::day15::{find_sensor_location, get_coverage_on_line, ManhattanArea, parse_input, Sensor};
    use crate::util::geometry::Point;

    #[test]
    fn test_parse_sensor() {
        assert_eq!(Ok(Sensor { location: (2, 18).into(), beacon: (-2,15).into(), area: ManhattanArea { center: (2,18).into(), length: 7 }}), "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse());
        assert_eq!(Ok(Sensor { location: (9, 16).into(), beacon: (10,16).into(), area: ManhattanArea { center: (9,16).into(), length: 1 }}), "Sensor at x=9, y=16: closest beacon is at x=10, y=16".parse());
    }

    #[test]
    fn test_manhattan_area() {
        let area = ManhattanArea { center: (2,18).into(), length: 7 };
        assert_eq!(Some(-5..=9), area.get_cols_for_line(18));
        assert_eq!(Some(-4..=8), area.get_cols_for_line(17));
        assert_eq!(Some(-4..=8), area.get_cols_for_line(19));
        assert_eq!(Some(2..=2), area.get_cols_for_line(11));
        assert_eq!(None, area.get_cols_for_line(10));
    }

    #[test]
    fn test_area_get_points_around() {
        let area_1 = ManhattanArea { center: (2,18).into(), length: 1 };
        assert_eq!(vec![
            Point::from((2,16)), Point::from((4,18)), Point::from((2,20)), Point::from((0,18)),
            Point::from((3,17)), Point::from((3,19)), Point::from((1,19)), Point::from((1,17)),
        ], area_1.get_points_around());

        let area_2 = ManhattanArea { center: (5, 5).into(), length: 2 };
        assert_eq!(vec![
            Point::from((5,2)), Point::from((8,5)), Point::from((5,8)), Point::from((2,5)), // TRBL
            Point::from((6,3)), Point::from((7,6)), Point::from((4,7)), Point::from((3,4)),
            Point::from((7,4)), Point::from((6,7)), Point::from((3,6)), Point::from((4,3)),
        ], area_2.get_points_around())
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());

        let sensors = result.unwrap();
        assert_eq!(14, sensors.len());
    }

    #[test]
    fn test_get_coverage_on_line() {
        let sensors = parse_input(TEST_INPUT).unwrap();
        assert_eq!(26, get_coverage_on_line(&sensors, 10));
    }

    #[test]
    fn test_find_sensor_location() {
        let sensors = parse_input(TEST_INPUT).unwrap();

        assert_eq!(Some(Point { x: 14, y: 11 }), find_sensor_location(&sensors, 0..=20));
    }

    const TEST_INPUT: &str = "\
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
        Sensor at x=20, y=1: closest beacon is at x=15, y=3\n\
    ";
}