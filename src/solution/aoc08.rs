pub struct _Solution {}

impl _Solution {
    pub fn _aoc_08_part1(strs: &str) -> i32 {
        0
    }

    pub fn _aoc_08_part2(strs: &str) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_08_part1() {
        assert_eq!(0, _Solution::_aoc_08_part1("(())"));
    }

    #[test]
    fn test_08_part2() {
        assert_eq!(0, _Solution::_aoc_08_part2("(())"));
    }

    #[test]
    fn test_08_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input08.txt").expect("Unable to read file");
        assert_eq!(0, _Solution::_aoc_08_part1(&data));
    }

    #[test]
    fn test_08_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input08.txt").expect("Unable to read file");
        assert_eq!(0, _Solution::_aoc_08_part2(&data));
    }
}
