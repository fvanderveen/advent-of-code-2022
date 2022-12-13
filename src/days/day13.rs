use std::cmp::{max, Ordering};
use std::fmt;
use std::str::FromStr;
use crate::days::Day;

pub const DAY13: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let pairs = parse_input(input).unwrap();

    let correct_indices = get_right_ordered_indices(&pairs);
    let answer: usize = correct_indices.iter().sum();

    println!("Sum of correctly ordered packet indices: {}", answer);
}

fn puzzle2(input: &String) {
    let pairs = parse_input(input).unwrap();

    let answer: usize = get_distress_decoder_key(&pairs);

    println!("Distress decoder key: {}", answer);
}

fn get_right_ordered_indices(pairs: &Vec<(Packet, Packet)>) -> Vec<usize> {
    pairs.iter().enumerate().filter_map(|(i, (lhs, rhs))| match lhs.cmp(rhs) {
        Ordering::Less => Some(i + 1), // Puzzle expects first index to be 1
        _ => None
    }).collect()
}

fn get_distress_decoder_key(pairs: &Vec<(Packet, Packet)>) -> usize {
    let ordered = order_packets_for_distress_signal(pairs);
    ordered.iter().enumerate().filter_map(|(i, p)| if Packet::decoder_key_a().eq(p) || Packet::decoder_key_b().eq(p) { Some(i+1) } else { None }).fold(1, |acc, v| acc * v)
}

fn order_packets_for_distress_signal(pairs: &Vec<(Packet, Packet)>) -> Vec<Packet> {
    let mut packets: Vec<_> = pairs.iter().flat_map(|(l, r)| vec![l, r]).cloned().collect();
    // Add the two distress divider packets
    packets.push(Packet::decoder_key_a());
    packets.push(Packet::decoder_key_b());
    packets.sort();

    packets
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Value(usize)
}

impl Packet {
    fn decoder_key_a() -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])])
    }

    fn decoder_key_b() -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])])
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(lhs), Packet::Value(rhs)) => lhs.cmp(rhs),
            (Packet::List(lhs), Packet::List(rhs)) => {
                let max_len = max(lhs.len(), rhs.len());
                for i in 0..max_len {
                    match (lhs.get(i), rhs.get(i)) {
                        (None, None) => panic!("This is not supposed to happen!"),
                        (None, _) => return Ordering::Less,
                        (_, None) => return Ordering::Greater,
                        (Some(lhp), Some(rhp)) => match lhp.cmp(rhp) {
                            Ordering::Equal => (),
                            res @ _ => return res
                        }
                    }
                }
                Ordering::Equal
            },
            (lhs @ Packet::List(_), rhs @ Packet::Value(_)) => lhs.cmp(&Packet::List(vec![rhs.clone()])),
            (lhs @ Packet::Value(_), rhs @ Packet::List(_)) => Packet::List(vec![lhs.clone()]).cmp(rhs)
        }
    }
}
impl PartialOrd for Packet { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.trim().chars().collect();

        let mut packet = None;
        let mut stack = vec![];
        for i in 0..chars.len() {
            let char = chars[i];
            let parse_error = |details: String| -> Self::Err {
                format!("{} at '{}':{}", details, s, i)
            };

            match char {
                '[' => { // Begin a new list packet
                    stack.push(vec![]);
                },
                '0'..='9' => { // (Begin) parse a value
                    let mut current = match packet {
                        Some(Packet::Value(val)) => val * 10,
                        Some(Packet::List(_)) => return Err(parse_error(format!("Missing ',' after list"))),
                        None => 0
                    };
                    current += (char as usize) - ('0' as usize);
                    packet = Some(Packet::Value(current));
                },
                ',' => { // Create packet from current value
                    if let Some(current) = packet {
                        // Current value is a number:
                        stack.last_mut().ok_or(parse_error(format!("Unexpected ',', no current list")))?.push(current);
                        packet = None;
                    } else {
                        return Err(parse_error(format!("Unexpected ',', no packet value read yet")))
                    }
                },
                ']' => { // End of current list
                    if let Some(mut list) = stack.pop() {
                        if let Some(current) = packet {
                            list.push(current);
                        }

                        packet = Some(Packet::List(list));
                    } else {
                        return Err(parse_error(format!("Unexpected ']', no list on stack.")))
                    }
                },
                _ if char.is_whitespace() => (), // Ignore whitespace during parsing
                _ => return Err(parse_error(format!("Invalid char: '{}'", char)))
            }
        }

        match packet {
            Some(p @ Packet::List(_)) => Ok(p),
            Some(_) => Err(format!("Unexpected end of packet, missing ']'? '{}':EOL", s)),
            None => Err(format!("Unexpected end of packet, no packet parsed? '{}':EOL", s))
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Value(val) => write!(f, "{}", val),
            Packet::List(vals) => write!(f, "[{}]", vals.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(","))
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Packet, Packet)>, String> {
    let lines: Vec<_> = input.lines().collect();

    let mut left = None;
    let mut right = None;
    let mut result = vec![];

    for i in 0..=lines.len() {
        // Make sure we iterate one more time with an empty string to avoid duplicating the result handling.
        let line = lines.get(i).unwrap_or(&"");

        if line.trim().is_empty() {
            if let Some(lv) = left {
                if let Some(rv) = right {
                    result.push((lv, rv));
                    left = None;
                    right = None;
                } else {
                    return Err(format!("Missing second packet!"));
                }
            }
            continue;
        }

        if left.is_none() {
            left = Some(line.parse()?)
        } else if right.is_none() {
            right = Some(line.parse()?)
        } else {
            return Err(format!("Extraneous line '{}' after reading two packets!", line))
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::days::day13::{get_distress_decoder_key, get_right_ordered_indices, order_packets_for_distress_signal, Packet, parse_input};

    impl Packet {
        fn values(vals: Vec<usize>) -> Packet {
            Packet::List(vals.iter().map(|v| Packet::Value(v.clone())).collect())
        }
    }

    #[test]
    fn test_parse_packet() {
        assert_eq!(Ok(Packet::values(vec![])), "[]".parse());
        assert_eq!(Ok(Packet::values(vec![1, 3, 5, 1])), "[1, 3, 5, 1]".parse());
        assert_eq!(Ok(Packet::List(vec![
            Packet::Value(1),
            Packet::values(vec![3, 3]),
            Packet::Value(7)
        ])), "[1,[3,3],7]".parse());
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "{}", result.err().unwrap_or("unexpected".to_string()));

        let pairs = result.unwrap();
        assert_eq!(8, pairs.len());
    }

    #[test]
    fn test_cmp_packet() {
        assert_eq!(Ordering::Less, Packet::Value(3).cmp(&Packet::Value(5)));
        assert_eq!(Ordering::Equal, Packet::Value(5).cmp(&Packet::Value(5)));
        assert_eq!(Ordering::Greater, Packet::Value(9).cmp(&Packet::values(vec![8, 7, 9])));
        assert_eq!(Ordering::Less, Packet::values(vec![1,2,3]).cmp(&Packet::values(vec![1,2,3,4])));
        assert_eq!(Ordering::Greater, Packet::values(vec![1,2,3,4]).cmp(&Packet::values(vec![1,2,3])));
        assert_eq!(Ordering::Equal, Packet::values(vec![1,2,3,4]).cmp(&Packet::values(vec![1,2,3,4])));

        let pairs = parse_input(TEST_INPUT).unwrap();
        assert_eq!(Ordering::Less, pairs[0].0.cmp(&pairs[0].1));
        assert_eq!(Ordering::Less, pairs[1].0.cmp(&pairs[1].1));
        assert_eq!(Ordering::Greater, pairs[2].0.cmp(&pairs[2].1));
        assert_eq!(Ordering::Less, pairs[3].0.cmp(&pairs[3].1));
        assert_eq!(Ordering::Greater, pairs[4].0.cmp(&pairs[4].1));
        assert_eq!(Ordering::Less, pairs[5].0.cmp(&pairs[5].1));
        assert_eq!(Ordering::Greater, pairs[6].0.cmp(&pairs[6].1));
        assert_eq!(Ordering::Greater, pairs[7].0.cmp(&pairs[7].1));
    }

    #[test]
    fn test_get_right_ordered_indices() {
        let pairs = parse_input(TEST_INPUT).unwrap();
        let indices = get_right_ordered_indices(&pairs);

        assert_eq!(vec![1, 2, 4, 6], indices);
    }

    #[test]
    fn test_order_packets_for_distress_signal() {
        let pairs = parse_input(TEST_INPUT).unwrap();
        let result = order_packets_for_distress_signal(&pairs);

        assert_eq!("[]", format!("{}", result[0]));
        assert_eq!("[[]]", format!("{}", result[1]));
        assert_eq!("[[[]]]", format!("{}", result[2]));
        assert_eq!("[1,1,3,1,1]", format!("{}", result[3]));
        assert_eq!("[1,1,5,1,1]", format!("{}", result[4]));
        assert_eq!("[[1],[2,3,4]]", format!("{}", result[5]));
        assert_eq!("[1,[2,[3,[4,[5,6,0]]]],8,9]", format!("{}", result[6]));
        assert_eq!("[1,[2,[3,[4,[5,6,7]]]],8,9]", format!("{}", result[7]));
        assert_eq!("[[1],4]", format!("{}", result[8]));
        assert_eq!("[[2]]", format!("{}", result[9]));
        assert_eq!("[3]", format!("{}", result[10]));
        assert_eq!("[[4,4],4,4]", format!("{}", result[11]));
        assert_eq!("[[4,4],4,4,4]", format!("{}", result[12]));
        assert_eq!("[[6]]", format!("{}", result[13]));
        assert_eq!("[7,7,7]", format!("{}", result[14]));
        assert_eq!("[7,7,7,7]", format!("{}", result[15]));
        assert_eq!("[[8,7,6]]", format!("{}", result[16]));
        assert_eq!("[9]", format!("{}", result[17]));
    }

    #[test]
    fn test_get_distress_decoder_key() {
        let pairs = parse_input(TEST_INPUT).unwrap();
        assert_eq!(140, get_distress_decoder_key(&pairs));
    }

    const TEST_INPUT: &str = "\
        [1,1,3,1,1]\n\
        [1,1,5,1,1]\n\
        \n\
        [[1],[2,3,4]]\n\
        [[1],4]\n\
        \n\
        [9]\n\
        [[8,7,6]]\n\
        \n\
        [[4,4],4,4]\n\
        [[4,4],4,4,4]\n\
        \n\
        [7,7,7,7]\n\
        [7,7,7]\n\
        \n\
        []\n\
        [3]\n\
        \n\
        [[[]]]\n\
        [[]]\n\
        \n\
        [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
        [1,[2,[3,[4,[5,6,0]]]],8,9]\n\
    ";
}