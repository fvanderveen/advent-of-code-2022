use crate::days::Day;

pub const DAY25: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let result: isize = input.lines().map(|l| decode_snafu_number(l).unwrap()).sum();
    let encoded = encode_snafu_number(result);
    println!("Sum of fuel is {}, as SNAFU: {}", result, encoded);
}

fn puzzle2(_: &String) {
    println!("Puzzle 2 is a freebie!");
}

fn decode_snafu_number(input: &str) -> Result<isize, String> {
    // SNAFU numbers are interesting. Powers of 5, and with weird options.
    // 2, 1, 0, - (-1), = (-2)
    // Right-most position is 'ones' (5^0), next is 'fives' (5^1), 25 (5^2), 125 (5^3) etc.
    // The number is multiplied by the position, and then everything is added.

    Ok(input.chars().rev().map(|c| match c {
        '2' => Ok(2),
        '1' => Ok(1),
        '0' => Ok(0),
        '-' => Ok(-1),
        '=' => Ok(-2),
        _ => Err(format!("Invalid SNAFU number: '{}'", c))
    }).collect::<Result<Vec<_>, _>>()?.iter().enumerate()
        .map(|(idx, val)| (5_isize.pow(idx as u32)) as isize * val).sum())
}
 
fn encode_snafu_number(input: isize) -> String {
    // What makes sense to do?
    // Ranges:
    // 0 (1) -> -2 => 2
    // 1 (5) -> -10 => 10 (total = -12 => 12) (= ±24 / 2)
    // 2 (25) -> -50 => 50 (total = -62 => 62) (= ±125 / 2)
    // 3 (125) -> -250 => 250 (total = -312 => 312)
    // (e.g. 201)
    // Find the power of 5 that is larger than the number (625 ('v))
    // Check if ('v - 1)/2 (312) is larger than the number
    // If so, we can fit it in the lower powers, if not, we need to subtract from this power.
    let mut values = vec![];
    
    // Note: could be done pulling the 5-root of input, but that is more difficult with floating points and their sizes.
    fn get_max_n(input: isize) -> u32 {
        let mut n = 1_u32;
        loop {
            if 5_isize.pow(n) > input {
                return n;
            }
            n += 1;
        }
    }
    
    let max_n = get_max_n(input);
    let mut rest = input;
    
    for n in (0..=max_n).rev() {
        let n_size = 5_isize.pow(n as u32);
        let max_lower_size = (n_size - 1) / 2;

        // This is not true; we might need a higher number here than rest, as the lower bits
        // can only go to half our n_size.
        if (rest + (n_size * 2)).abs() <= max_lower_size {
            // We need a -2
            values.push(-2);
            rest += n_size * 2;
        } else if (rest + n_size).abs() <= max_lower_size {
            // We need a -1
            values.push(-1);
            rest += n_size;
        } else if rest.abs() <= max_lower_size {
            // We need a 0
            values.push(0);
        } else if (rest - n_size).abs() <= max_lower_size {
            // We need a 1
            values.push(1);
            rest -= n_size;
        } else if (rest - (n_size * 2)).abs() <= max_lower_size {
            // We need a 2
            values.push(2);
            rest -= n_size * 2;
        } else {
            panic!("Couldn't determine number for n={}, rest={}, values={:?}, input={}, max_n={}", n, rest, values, input, max_n);
        }
    }
    
    values.iter().skip_while(|v| 0.eq(*v)).map(|v| match v {
        -2 => "=",
        -1 => "-",
        0 => "0",
        1 => "1",
        2 => "2",
        _ => panic!("No! {}", v)
    }).map(|s| s.to_string()).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
    use crate::days::day25::{decode_snafu_number, encode_snafu_number};

    #[test]
    fn test_decode_snafu_number() {
        for (expected, input) in SNAFU_TESTS {
            assert_eq!(Ok(expected), decode_snafu_number(input));
        }
    }
    
    #[test]
    fn test_encode_snafu_number() {
        for (input, expected) in SNAFU_TESTS {
            assert_eq!(expected, encode_snafu_number(input));
        }
    }
    
    #[test]
    fn test_example() {
        let value: isize = TEST_INPUT.lines().map(|l| decode_snafu_number(l).unwrap()).sum();
        assert_eq!(4890, value);
        assert_eq!("2=-1=0", encode_snafu_number(value));
    }
    
    const TEST_INPUT: &str = "\
        1=-0-2\n\
        12111\n\
        2=0=\n\
        21\n\
        2=01\n\
        111\n\
        20012\n\
        112\n\
        1=-1=\n\
        1-12\n\
        12\n\
        1=\n\
        122\n\
    ";
    
    const SNAFU_TESTS: [(isize, &str);15] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0")
    ];
}