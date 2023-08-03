pub struct _Solution {}

impl _Solution {
    pub fn _aoc_08_part1(strs: &str) -> i32 {
        let mut sum_len_decode: i32 = 0;
        let mut sum_len_full: i32 = 0;
        for line in strs.lines() {
            sum_len_full += line.len() as i32;
            let parsed_line = Self::_decode(line);
            sum_len_decode += parsed_line.len() as i32;
        }
        sum_len_full - sum_len_decode
    }

    pub fn _decode(strs: &str) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        let mut chars: Vec<char> = strs.chars().collect();

        // remove double quotes
        chars.remove(0);
        chars.remove(chars.len() - 1);

        while chars.len() > 0 {
            let c = chars.remove(0);
            if c == '\\' {
                let c2 = chars.remove(0);
                if c2 == 'x' {
                    let c3 = chars.remove(0);
                    let c4 = chars.remove(0);
                    let out = Self::_to_ascii(format!("{}{}", c3, c4));
                    bytes.push(out)
                } else {
                    bytes.push(c2 as u8)
                }
            } else {
                bytes.push(c as u8)
            }
        }
        bytes
    }

    pub fn _encode(strs: &str) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        let mut chars: Vec<char> = strs.chars().collect();
        bytes.push(b'\"');

        while chars.len() > 0 {
            let c = chars.remove(0);
            if c == '\\' {
                bytes.push(b'\\');
                bytes.push(b'\\');
            } else if c == '"' {
                bytes.push(b'\\');
                bytes.push(b'\"');
            } else {
                bytes.push(c as u8)
            }
        }
        bytes.push(b'\"');
        bytes
    }

    fn _to_ascii(input: String) -> u8 {
        *hex::decode(input)
            .expect("Failed to decode")
            .get(0)
            .unwrap()
    }

    pub fn _aoc_08_part2(strs: &str) -> i32 {
        let mut sum_len_encode: i32 = 0;
        let mut sum_len_full: i32 = 0;
        for line in strs.lines() {
            sum_len_full += line.len() as i32;
            let encoded_line = Self::_encode(line);
            sum_len_encode += encoded_line.len() as i32;
        }
        sum_len_encode - sum_len_full
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_08_part1() {
        let data =
            fs::read_to_string("src/solution/input08-example.txt").expect("Unable to read file");
        assert_eq!(12, _Solution::_aoc_08_part1(&data));
        assert_eq!(2, _Solution::_aoc_08_part1("\"\""));
    }

    #[test]
    fn test_decode() {
        assert_eq!(0, _Solution::_decode("\"\"").len());
        assert_eq!(9, _Solution::_decode("\"aaa\\\"aaa\\\\\\\\\"").len());
        assert_eq!(2, _Solution::_decode("\"\\\\\\\\\"").len());
        assert_eq!(1, _Solution::_decode("\"\\\\\"").len());
        assert_eq!(7, _Solution::_decode("\"aaa\\\"aaa\"").len());
        assert_eq!(1, _Solution::_decode("\"\\x27\"").len());
        assert_eq!(1, _Solution::_decode("\"\\xd4\"").len());
    }

    #[test]
    fn test_encode() {
        assert_eq!(6, _Solution::_encode("\"\"").len());
        assert_eq!(11, _Solution::_encode("\"\\x27\"").len());
        assert_eq!(16, _Solution::_encode("\"aaa\\\"aaa\"").len());
        assert_eq!(9, _Solution::_encode("\"abc\"").len());
    }

    #[test]
    fn test_08_part2() {
        let data =
            fs::read_to_string("src/solution/input08-example.txt").expect("Unable to read file");
        assert_eq!(19, _Solution::_aoc_08_part2(&data));
    }

    #[test]
    fn test_08_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input08.txt").expect("Unable to read file");
        assert_eq!(1371, _Solution::_aoc_08_part1(&data));
    }

    #[test]
    fn test_08_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input08.txt").expect("Unable to read file");
        assert_eq!(2117, _Solution::_aoc_08_part2(&data));
    }
}
