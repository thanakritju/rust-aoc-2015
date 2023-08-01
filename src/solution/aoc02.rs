pub struct Solution {}

impl Solution {
    pub fn aoc_02_part1(strs: &str) -> i32 {
        0
    }

    pub fn aoc_02_part2(strs: &str) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_02_part1() {
        assert_eq!(0, Solution::aoc_02_part1("(())"));
    }

    #[test]
    fn test_02_part2() {
        assert_eq!(0, Solution::aoc_02_part2("(())"));
    }

    #[test]
    fn test_02_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input02.txt").expect("Unable to read file");
        assert_eq!(280, Solution::aoc_02_part1(&data));
    }

    #[test]
    fn test_02_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input02.txt").expect("Unable to read file");
        assert_eq!(280, Solution::aoc_02_part2(&data));
    }
}
