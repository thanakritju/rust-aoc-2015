pub struct Solution {}

impl Solution {
    pub fn aoc_01_part1(strs: &str) -> i32 {
        let mut sum: i32 = 0;
        for c in strs.chars() {
            if c == '(' {
                sum += 1
            } else {
                sum -= 1
            }
        }
        sum
    }

    pub fn aoc_01_part2(strs: &str) -> i32 {
        let mut sum: i32 = 0;
        for (i, c) in strs.chars().enumerate() {
            if c == '(' {
                sum += 1
            } else {
                sum -= 1
            }
            if sum == -1 {
                return (i + 1).try_into().unwrap();
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_aoc_01_part1() {
        assert_eq!(0, Solution::aoc_01_part1("(())"));
        assert_eq!(3, Solution::aoc_01_part1("((("));
        assert_eq!(-3, Solution::aoc_01_part1(")())())"));
    }

    #[test]
    fn test_aoc_01_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input01.txt").expect("Unable to read file");
        assert_eq!(280, Solution::aoc_01_part1(&data));
    }

    #[test]
    fn test_aoc_01_part2() {
        assert_eq!(1, Solution::aoc_01_part2(")"));
        assert_eq!(5, Solution::aoc_01_part2("()())"));
    }

    #[test]
    fn test_aoc_01_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input01.txt").expect("Unable to read file");
        assert_eq!(1797, Solution::aoc_01_part2(&data));
    }
}
