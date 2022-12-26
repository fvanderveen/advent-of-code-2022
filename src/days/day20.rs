use std::collections::VecDeque;
use crate::days::Day;
use crate::util::number::parse_isize;

pub const DAY20: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let numbers: Vec<isize> = input.lines().map(|l| parse_isize(l).unwrap()).collect();

    let coords = get_coordinates(&numbers, 1, 1);
    let result = coords[0] + coords[1] + coords[2];
    
    println!("Sum of coordinates ({}, {}, {}): {}", coords[0], coords[1], coords[2], result);
}
fn puzzle2(input: &String) {
    let numbers: Vec<isize> = input.lines().map(|l| parse_isize(l).unwrap()).collect();
    
    let coords = get_coordinates(&numbers, 811589153, 10);
    let result = coords[0] + coords[1] + coords[2];

    println!("Sum of coordinates ({}, {}, {}): {}", coords[0], coords[1], coords[2], result);
}

fn get_coordinates(input: &Vec<isize>, key: isize, rounds: usize) -> [isize;3] {
    // Handle numbers from input left -> right.
    // Each number moves as much as their value (e.g. 1 moves 1 to the right, -2 moves 2 to the left)
    // Index wraps around the list
    let mut values: VecDeque<(usize, isize)> = input.iter()
        .map(|v| *v * key)
        .enumerate()
        .collect();

    for _ in 0..rounds {
        for move_idx in 0..input.len() {
            let index = values.iter().position(|(i, _)| move_idx == *i).unwrap();
            // Move what we need to move to the front of this list
            values.rotate_left(index);
            let (og_idx, val) = values.pop_front().unwrap();
            let dest_index = val.rem_euclid(values.len() as isize) as usize;
            // Move the list again to where we need to insert the value
            values.rotate_left(dest_index);
            values.push_front((og_idx, val));
        }
    }

    let result: Vec<_> = values.iter().map(|(_, v)| *v).collect();
    // The first coordinate is the 1000th number (with wrapping) from 0. The second is at 2000, and the third at 3000.
    let start_idx = result.iter().position(|v| 0.eq(v)).unwrap();
    
    let first_idx = (start_idx + 1000) % result.len();
    let second_idx = (start_idx + 2000) % result.len();
    let third_idx = (start_idx + 3000) % result.len();
    
    [result[first_idx], result[second_idx], result[third_idx]]
}

#[cfg(test)]
mod tests {
    use crate::days::day20::{get_coordinates};

    #[test]
    fn test_get_coordinates() {
        let result = TEST_INPUT.into();
        assert_eq!([4, -3, 2], get_coordinates(&result, 1, 1));
        assert_eq!(3, get_coordinates(&result, 1, 1).iter().sum::<isize>());

        assert_eq!([811589153, 2434767459, -1623178306], get_coordinates(&result, 811589153, 10));
    }
    
    static TEST_INPUT: [isize;7] = [1,2,-3,3,-2,0,4];
}