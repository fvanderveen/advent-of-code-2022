use crate::days::Day;
use crate::util::collection::CollectionExtension;

pub const DAY6: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let marker = detect_start_of_packet(input).unwrap();

    println!("Start of packet at offset: {}", marker);
}
fn puzzle2(input: &String) {
    let marker = detect_start_of_message(input).unwrap();

    println!("Start of message at offset: {}", marker);
}

/// To fix the communication system, you need to add a subroutine to the device that detects a
/// start-of-packet marker in the datastream. In the protocol being used by the Elves, the start
/// of a packet is indicated by a sequence of four characters that are all different.
fn detect_start_of_packet(stream: &str) -> Option<usize> {
    for i in 4..stream.len() {
        let chars = stream[i-4..i].chars().collect::<Vec<_>>().deduplicate();
        if chars.len() == 4 {
            return Some(i)
        }
    }
    None
}

/// Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.
/// A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.
fn detect_start_of_message(stream: &str) -> Option<usize> {
    for i in 14..stream.len() {
        let chars = stream[i-14..i].chars().collect::<Vec<_>>().deduplicate();
        if chars.len() == 14 {
            return Some(i)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::days::day06::{detect_start_of_message, detect_start_of_packet};

    #[test]
    fn test_detect_start_of_packet() {
        assert_eq!(Some(7), detect_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), detect_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), detect_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), detect_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), detect_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_detect_start_of_message() {
        assert_eq!(Some(19), detect_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(23), detect_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(23), detect_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(29), detect_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(26), detect_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}