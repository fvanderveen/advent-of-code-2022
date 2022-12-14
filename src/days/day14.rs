use std::fmt;
use crate::days::Day;
use crate::util::geometry::{Grid, Line, Point};
use crate::util::number::parse_isize;

pub const DAY14: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let cave = create_cave(input).unwrap();

    let held_sand = cave.get_max_held_sand(None);
    println!("The cave holds at most {} sand blocks", held_sand);
}
fn puzzle2(input: &String) {
    let cave = create_cave(input).unwrap();
    let flooring = cave.determine_flooring();

    let held_sand = cave.get_max_held_sand(flooring);
    println!("With a floor, the cave holds at most {} sand blocks", held_sand);
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
    Extruder
}

type Cave = Grid<Tile>;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Air => write!(f, "."),
            Tile::Rock => write!(f, "#"),
            Tile::Sand => write!(f, "o"),
            Tile::Extruder => write!(f, "+")
        }
    }
}

fn create_cave(input: &str) -> Result<Cave, String> {
    let mut cave = Grid::default();

    for r_line in input.lines() {
        let points = parse_rock_line(r_line)?;

        for i in 0..points.len()-1 {
            let line = Line { start: points[i], end: points[i+1] };
            line.get_points().iter().for_each(|p| cave.set(p.clone(), Tile::Rock));
        }
    }

    // The cave always has a source of falling sand at 500,0:
    cave.set((500, 0).into(), Tile::Extruder);

    Ok(cave)
}

fn parse_rock_line(line: &str) -> Result<Vec<Point>, String> {
    let mut points = vec![];

    for part in line.split(" -> ") {
        let coords: Vec<_> = part.split(",").map(|s| s.trim()).collect();
        if coords.len() != 2 { return Err(format!("Invalid coordinate '{}'", part)) }
        let x = parse_isize(coords[0])?;
        let y = parse_isize(coords[1])?;
        points.push((x,y).into());
    }

    Ok(points)
}

impl Cave {
    fn determine_flooring(&self) -> Option<isize> {
        let max_y = self.entries().iter().filter(|(_, t)| Tile::Rock.eq(t)).map(|(p, _)| p.y).max();
        return max_y.map(|v| v + 2);
    }

    fn get_max_held_sand(&self, flooring: Option<isize>) -> usize {
        // We could just make this param mut, but that's less fun.
        let mut sim = self.clone();
        let mut sands = 0;
        while sim.drop_sand(flooring) {
            sands += 1;
        }
        sands
    }

    fn drop_sand(&mut self, flooring: Option<isize>) -> bool {
        let known_cells = self.entries();
        let extruder = match known_cells.iter().find(|(_, t)| Tile::Extruder.eq(t)).map(|(p, _)| p) {
            Some(point) => point,
            None => return false
        };

        // Sand falls straight down (y+1) until it hits rock or sand.
        // Once it finds rock or sand:
        // 1. if the spot to the left (x-1) is free, continue there
        // 2. if the spot to the right is free, continue there
        // 3. if either spot is taken, this sand stacks on top of the found tile.
        let mut current_point = extruder.clone();

        'main: loop {
            let next_point: Point = current_point + (0, 1);

            match self.get_tile(&next_point, flooring) {
                None => return false, // We're out-of-bounds
                Some(Tile::Air) | Some(Tile::Extruder) => {
                    // Next point is (considered) free, so we move on downwards.
                    current_point = next_point;
                    continue;
                },
                Some(Tile::Rock) | Some(Tile::Sand) => {
                    // Next point is taken. Check left & right:
                    let to_check = vec![next_point + (-1, 0), next_point + (1, 0)];
                    for point in to_check {
                        match self.get_tile(&point, flooring) {
                            None | Some(Tile::Air) | Some(Tile::Extruder) => {
                                // Point is free, continue there.
                                current_point = point;
                                continue 'main;
                            },
                            Some(Tile::Rock) | Some(Tile::Sand) => {
                                // Already taken, continue to the next point (if any)
                            }
                        }
                    }

                    // If the current point is the extruder, we're full as well:
                    // if current_point.eq(extruder) {
                    //     return false;
                    // }

                    // Both left & right are taken, so drop the sand here:
                    self.set(current_point, Tile::Sand);
                    return true;
                }
            }
        }
    }

    fn get_tile(&self, point: &Point, flooring: Option<isize>) -> Option<Tile> {
        match flooring {
            None if self.bounds.contains(point) => Some(self.get(point).unwrap_or_default()),
            None => None, // No flooring, and next point is outside the known area
            Some(val) if val > point.y => Some(self.get(point).unwrap_or_default()), // Flooring, but we've not yet reached it
            Some(_) => Some(Tile::Rock) // Flooring, and we've reached it.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day14::{create_cave, Tile};

    #[test]
    fn test_create_cave() {
        let result = create_cave(TEST_INPUT);
        assert!(result.is_ok());

        let cave = result.unwrap();
        assert_eq!("\
        ......+...\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ....#...##\n\
        ....#...#.\n\
        ..###...#.\n\
        ........#.\n\
        ........#.\n\
        #########.\
        ", format!("{}", cave));
    }

    #[test]
    fn test_drop_sand() {
        let mut cave = create_cave(TEST_INPUT).unwrap();

        assert_eq!(true, cave.drop_sand(None));
        assert_eq!("\
        ......+...\n\
        ..........\n\
        ..........\n\
        ..........\n\
        ....#...##\n\
        ....#...#.\n\
        ..###...#.\n\
        ........#.\n\
        ......o.#.\n\
        #########.\
        ", format!("{}", cave));

        assert_eq!(true, cave.drop_sand(None));
        assert_eq!(Some(Tile::Sand), cave.get(&(499, 8).into()));
        assert_eq!(Some(Tile::Sand), cave.get(&(500, 8).into()));
        assert_eq!(None, cave.get(&(501, 8).into()));

        assert_eq!(true, cave.drop_sand(None));
        assert_eq!(Some(Tile::Sand), cave.get(&(499, 8).into()));
        assert_eq!(Some(Tile::Sand), cave.get(&(500, 8).into()));
        assert_eq!(Some(Tile::Sand), cave.get(&(501, 8).into()));
        assert_eq!(None, cave.get(&(500, 7).into()));

        assert_eq!(true, cave.drop_sand(None));
        assert_eq!(Some(Tile::Sand), cave.get(&(500, 7).into()));

        assert_eq!(true, cave.drop_sand(None));
        assert_eq!(Some(Tile::Sand), cave.get(&(498, 8).into()));

        for _ in 0..17 {
            assert_eq!(true, cave.drop_sand(None));
        }
        assert_eq!("\
        ......+...\n\
        ..........\n\
        ......o...\n\
        .....ooo..\n\
        ....#ooo##\n\
        ....#ooo#.\n\
        ..###ooo#.\n\
        ....oooo#.\n\
        ...ooooo#.\n\
        #########.\
        ", format!("{}", cave));

        assert_eq!(true, cave.drop_sand(None));
        assert_eq!(true, cave.drop_sand(None));
        assert_eq!("\
        ......+...\n\
        ..........\n\
        ......o...\n\
        .....ooo..\n\
        ....#ooo##\n\
        ...o#ooo#.\n\
        ..###ooo#.\n\
        ....oooo#.\n\
        .o.ooooo#.\n\
        #########.\
        ", format!("{}", cave));
        assert_eq!(false, cave.drop_sand(None)); // Any other sand will just fall into the abyss
    }

    #[test]
    fn test_get_max_held_sand() {
        let cave = create_cave(TEST_INPUT).unwrap();
        assert_eq!(24, cave.get_max_held_sand(None));
    }

    #[test]
    fn test_determine_flooring() {
        let cave = create_cave(TEST_INPUT).unwrap();
        assert_eq!(Some(11), cave.determine_flooring());
    }

    #[test]
    fn test_drop_sand_with_flooring() {
        let mut cave = create_cave(TEST_INPUT).unwrap();
        let flooring = Some(11);

        for _ in 0..24 {
            assert_eq!(true, cave.drop_sand(flooring));
        }

        assert_eq!("\
        ......+...\n\
        ..........\n\
        ......o...\n\
        .....ooo..\n\
        ....#ooo##\n\
        ...o#ooo#.\n\
        ..###ooo#.\n\
        ....oooo#.\n\
        .o.ooooo#.\n\
        #########.\
        ", format!("{}", cave));

        // From here on, sand will fall on the ground (not rendered):
        assert_eq!(true, cave.drop_sand(flooring));
        assert_eq!("\
        .......+...\n\
        ...........\n\
        .......o...\n\
        ......ooo..\n\
        .....#ooo##\n\
        ....o#ooo#.\n\
        ...###ooo#.\n\
        .....oooo#.\n\
        ..o.ooooo#.\n\
        .#########.\n\
        o..........\
        ", format!("{}", cave));

        for _ in 0..68 {
            assert_eq!(true, cave.drop_sand(flooring));
        }

        assert_eq!("\
        ..........o..........\n\
        .........ooo.........\n\
        ........ooooo........\n\
        .......ooooooo.......\n\
        ......oo#ooo##o......\n\
        .....ooo#ooo#ooo.....\n\
        ....oo###ooo#oooo....\n\
        ...oooo.oooo#ooooo...\n\
        ..oooooooooo#oooooo..\n\
        .ooo#########ooooooo.\n\
        ooooo.......ooooooooo\
        ", format!("{}", cave));
        assert_eq!(false, cave.drop_sand(flooring)); // Next sand would be _on_ the extruder, so we ignore it.
    }

    #[test]
    fn test_get_max_held_sand_with_flooring() {
        let cave = create_cave(TEST_INPUT).unwrap();

        assert_eq!(93, cave.get_max_held_sand(Some(11)));
    }

    const TEST_INPUT: &str = "\
        498,4 -> 498,6 -> 496,6\n\
        503,4 -> 502,4 -> 502,9 -> 494,9\n\
    ";
}