pub struct _Solution {}

impl _Solution {
    pub fn _aoc_09_part1(strs: &str) -> i32 {
        0
    }

    pub fn _aoc_09_part2(strs: &str) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_09_part1() {
        let data =
            fs::read_to_string("src/solution/input09-example.txt").expect("Unable to read file");
        assert_eq!(605, _Solution::_aoc_09_part1(&data));
    }

    #[test]
    fn test_09_part2() {
        assert_eq!(0, _Solution::_aoc_09_part2("(())"));
    }

    #[test]
    fn test_09_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input09.txt").expect("Unable to read file");
        assert_eq!(0, _Solution::_aoc_09_part1(&data));
    }

    #[test]
    fn test_09_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input09.txt").expect("Unable to read file");
        assert_eq!(0, _Solution::_aoc_09_part2(&data));
    }
}
