use std::{cmp, str::FromStr};

pub struct Solution {}

impl Solution {
    pub fn aoc_02_part1(strs: &str) -> i32 {
        let mut sum = 0;
        for line in strs.lines() {
            let dimensions = line.split('x').collect::<Vec<&str>>();
            let l: i32 = FromStr::from_str(dimensions[0]).unwrap();
            let w: i32 = FromStr::from_str(dimensions[1]).unwrap();
            let h: i32 = FromStr::from_str(dimensions[2]).unwrap();
            let side1 = l * w;
            let side2 = h * w;
            let side3 = h * l;
            sum += 2 * side1 + 2 * side2 + 2 * side3 + cmp::min(cmp::min(side1, side2), side3)
        }
        sum
    }

    pub fn aoc_02_part2(strs: &str) -> i32 {
        let mut sum = 0;
        for line in strs.lines() {
            let dimensions = line.split('x').collect::<Vec<&str>>();
            let l: i32 = FromStr::from_str(dimensions[0]).unwrap();
            let w: i32 = FromStr::from_str(dimensions[1]).unwrap();
            let h: i32 = FromStr::from_str(dimensions[2]).unwrap();
            if l < w {
                sum += 2 * l;
                if w < h {
                    sum += 2 * w
                } else {
                    sum += 2 * h
                }
            } else {
                sum += 2 * w;
                if l < h {
                    sum += 2 * l
                } else {
                    sum += 2 * h
                }
            }
            sum += l * w * h
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_02_part1() {
        assert_eq!(58, Solution::aoc_02_part1("2x3x4"));
        assert_eq!(43, Solution::aoc_02_part1("1x1x10"));
        assert_eq!(43 + 58, Solution::aoc_02_part1("2x3x4\n1x1x10\n"));
    }

    #[test]
    fn test_02_part2() {
        assert_eq!(34, Solution::aoc_02_part2("2x3x4"));
        assert_eq!(14, Solution::aoc_02_part2("1x1x10"));
        assert_eq!(48 + (21 * 12 * 12), Solution::aoc_02_part2("21x12x12"));
        assert_eq!(14 + 34, Solution::aoc_02_part2("2x3x4\n1x1x10\n"));
    }

    #[test]
    fn test_02_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input02.txt").expect("Unable to read file");
        assert_eq!(1588178, Solution::aoc_02_part1(&data));
    }

    #[test]
    fn test_02_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input02.txt").expect("Unable to read file");
        assert_eq!(3783758, Solution::aoc_02_part2(&data));
    }
}
