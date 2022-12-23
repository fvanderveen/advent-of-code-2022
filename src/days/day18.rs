use std::collections::HashSet;
use crate::days::Day;
use crate::util::geometry::{Point3D};

pub const DAY18: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let drops = parse_input(input).unwrap();

    let area = get_surface_area(&drops);
    println!("Total surface area of droplets: {}", area);
}

fn puzzle2(input: &String) {
    let drops = parse_input(input).unwrap();

    let area = get_outer_surface_area(&drops);
    println!("Total outer surface area of droplets: {}", area);
}

fn get_surface_area(drops: &Vec<Point3D>) -> usize {
    // Ideally, we find – in a somewhat performant way – a number of overlapping sides.
    // A side overlaps if a cube is right next to another cube
    // We can add the sides per cube; and per added cube check if there is overlap (and compensate)

    let mut area = 0;
    let mut handled_cubes: Vec<Point3D> = vec![];

    for point in drops {
        area += 6; // 6 sides of this cube

        // Check against already handled cubes for touching sides:
        // A cube is touching this one if 2 coordinates are the same, and the other is ±1
        // IOW, it has a manhattan distance of 1.
        let touching_cubes = handled_cubes.iter().filter(|c| c.manhattan(point) == 1).count();
        // Every touching cube subtracts 2 units of surface area (since we added both with all 6 sides)
        area -= touching_cubes * 2;

        // Store a copy for the next cubes to check against.
        handled_cubes.push(point.clone());
    }

    area
}

fn get_outer_surface_area(drops: &Vec<Point3D>) -> usize {
    // For part 2, we need to ignore any holes inside the shape (completely enclosed).
    // The above could work, if we could detect that a new cube closes off an area inside...
    // Similarly, we could start outside the cube (this should be deducable from min x,y,z) and see
    // which cubes we can actually reach by just traversing all points.

    // In the (unlikely) case a pixel is in the MinX,MinY,MinZ corner, we - 1 all these values:
    let min_x = drops.iter().map(|p| p.x).min().unwrap() - 1;
    let min_y = drops.iter().map(|p| p.y).min().unwrap() - 1;
    let min_z = drops.iter().map(|p| p.z).min().unwrap() - 1;
    // Same here for max:
    let max_x = drops.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = drops.iter().map(|p| p.y).max().unwrap() + 1;
    let max_z = drops.iter().map(|p| p.z).max().unwrap() + 1;

    let mut done: HashSet<Point3D> = HashSet::new();
    let mut queue: Vec<Point3D> = vec![Point3D::from((min_x, min_y, min_z))];

    let mut sides = 0;

    while let Some(point) = queue.pop() {
        // For every neighbouring point that is not already done and in bounds:
        // - If it is a cube, add a side
        // - If it is not a cube, add to queue
        // - cross fingers
        if !done.insert(point) { continue; }

        point.get_points_around().iter()
            .filter(|p| min_x <= p.x && p.x <= max_x && min_y <= p.y && p.y <= max_y && min_z <= p.z && p.z <= max_z)
            .filter(|p| p.manhattan(&point) == 1) // Filter out diagonal points
            .for_each(|p| {
                if drops.contains(p) {
                    sides += 1;
                } else if !done.contains(p) {
                    queue.push(*p);
                }
            });
    }

    sides
}

fn parse_input(input: &str) -> Result<Vec<Point3D>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{get_outer_surface_area, get_surface_area, parse_input};

    #[test]
    fn test_parse_input() {
        assert!(parse_input(TEST_INPUT).is_ok());
    }

    #[test]
    fn test_get_surface_area() {
        let drops = parse_input(TEST_INPUT).unwrap();

        assert_eq!(64, get_surface_area(&drops));
    }

    #[test]
    fn test_get_outer_surface_area() {
        let drops = parse_input(TEST_INPUT).unwrap();

        assert_eq!(58, get_outer_surface_area(&drops));
    }

    const TEST_INPUT: &str = "\
        2,2,2\n\
        1,2,2\n\
        3,2,2\n\
        2,1,2\n\
        2,3,2\n\
        2,2,1\n\
        2,2,3\n\
        2,2,4\n\
        2,2,6\n\
        1,2,5\n\
        3,2,5\n\
        2,1,5\n\
        2,3,5\n\
    ";
}