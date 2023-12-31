use md5::Digest;

pub struct _Solution {}

impl _Solution {
    pub fn _aoc_04_part1(strs: &str) -> i32 {
        let mut i = 0;
        loop {
            let hash: Digest = md5::compute(&format!("{}{}", strs, i));
            if format!("{:x}", hash).starts_with("00000") {
                break;
            }
            i += 1
        }
        i
    }

    pub fn _aoc_04_part2(strs: &str) -> i32 {
        let mut i = 0;
        loop {
            let hash: Digest = md5::compute(&format!("{}{}", strs, i));
            if format!("{:x}", hash).starts_with("000000") {
                break;
            }
            i += 1
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[ignore]
    #[test]
    fn test_04_part1() {
        assert_eq!(609043, _Solution::_aoc_04_part1("abcdef"));
        assert_eq!(1048970, _Solution::_aoc_04_part1("pqrstuv"));
    }

    #[ignore]
    #[test]
    fn test_04_part2() {
        assert_eq!(6742839, _Solution::_aoc_04_part2("abcdef"));
    }

    #[ignore]
    #[test]
    fn test_04_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input04.txt").expect("Unable to read file");
        assert_eq!(346386, _Solution::_aoc_04_part1(&data));
    }

    #[ignore]
    #[test]
    fn test_04_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input04.txt").expect("Unable to read file");
        assert_eq!(9958218, _Solution::_aoc_04_part2(&data));
    }
}
