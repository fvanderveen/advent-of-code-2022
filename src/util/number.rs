// Allow dead_code since this is a util file copied across years. Later in the AoC we might use everything, or not.
#![allow(dead_code)]

macro_rules! parse_int_impl {
    ($($t:ty, $name: ident)*) => {$(
        #[allow(unused)]
        pub fn $name(input: &str) -> Result<$t, String> {
            input.to_string().parse().map_err(|e| format!("{}", e))
        }
    )*}
}

parse_int_impl! {
    u8, parse_u8
    u16, parse_u16
    u32, parse_u32
    u64, parse_u64
    u128, parse_u128
    usize, parse_usize
    i8, parse_i8
    i16, parse_i16
    i32, parse_i32
    i64, parse_i64
    i128, parse_i128
    isize, parse_isize
}

pub fn parse_binary(binary: &str) -> usize {
    let mut result = 0;

    for char in binary.chars() {
        result <<= 1;
        match char {
            '1' => result += 1,
            '0' => {},
            _ => panic!("Invalid binary character: {}", char)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::util::number::parse_binary;

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_binary("0101"), 5);
        assert_eq!(parse_binary("1111"), 15);
        assert_eq!(parse_binary("1000000"), 64);
    }
}