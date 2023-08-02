use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn aoc_05_part1(strs: &str) -> i32 {
        let mut count = 0;
        for line in strs.lines() {
            if !Solution::is_forbidden(line)
                && Solution::same_letter(line)
                && Solution::exceeds_vowels(line)
            {
                count += 1
            }
        }
        count
    }

    fn same_letter(s: &str) -> bool {
        let ans = s
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|x| x[0] == x[1]);
        ans
    }

    fn is_forbidden(s: &str) -> bool {
        let forbidden_list = ["ab", "cd", "pq", "xy"];
        let ans = s.chars().collect::<Vec<char>>().windows(2).any(|y| {
            forbidden_list
                .iter()
                .position(|&x| x == format!("{}{}", y[0], y[1]))
                .is_some()
        });
        ans
    }

    fn exceeds_vowels(s: &str) -> bool {
        let vowels = "aeiou";
        let mut vowels_count = 0;
        for c in s.chars().collect::<Vec<char>>() {
            if vowels.chars().position(|x| x == c).is_some() {
                vowels_count += 1
            }
        }
        vowels_count >= 3
    }

    pub fn aoc_05_part2(strs: &str) -> i32 {
        let mut count = 0;
        for line in strs.lines() {
            if Solution::is_sandwich(line) && Solution::a_pair_duplicate(line) {
                count += 1
            }
        }
        count
    }

    fn is_sandwich(s: &str) -> bool {
        let ans = s
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .any(|x| x[0] == x[2]);
        ans
    }

    fn a_pair_duplicate(s: &str) -> bool {
        for (i, w1) in s.chars().collect::<Vec<char>>().windows(2).enumerate() {
            for (j, w2) in s.chars().collect::<Vec<char>>().windows(2).enumerate() {
                if (i == j || i == j + 1 || i + 1 == j) {
                    continue;
                }
                if w1[0] == w2[0] && w1[1] == w2[1] {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_is_forbidden() {
        assert_eq!(true, Solution::is_forbidden("haegwjzuvuyypxy"));
        assert_eq!(true, Solution::is_forbidden("xyhaegwjzuvuyyp"));
        assert_eq!(false, Solution::is_forbidden("pa"));
    }

    #[test]
    fn test_same_letter() {
        assert_eq!(false, Solution::same_letter("abc"));
        assert_eq!(true, Solution::same_letter("aa"));
        assert_eq!(false, Solution::same_letter("pa"));
    }

    #[test]
    fn test_05_part1() {
        assert_eq!(1, Solution::aoc_05_part1("ugknbfddgicrmopn"));
        assert_eq!(4, Solution::aoc_05_part1("aaa\naaa\naaa\naaa"));
        assert_eq!(1, Solution::aoc_05_part1("aaccbbdde"));
        assert_eq!(0, Solution::aoc_05_part1("aaab"));
        assert_eq!(1, Solution::aoc_05_part1("iii"));
        assert_eq!(0, Solution::aoc_05_part1("xazegov"));
        assert_eq!(0, Solution::aoc_05_part1("jchzalrnumimnmhp"));
        assert_eq!(0, Solution::aoc_05_part1("haegwjzuvuyypxy"));
        assert_eq!(0, Solution::aoc_05_part1("dvszwmarrgswjxmb"));
        assert_eq!(
            1,
            Solution::aoc_05_part1("jchzalrnumimnmhp\naaa\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\n")
        );
    }

    #[test]
    fn test_05_part2() {
        assert_eq!(1, Solution::aoc_05_part2("qjhvhtzxzqqjkmpb"));
        assert_eq!(1, Solution::aoc_05_part2("xxyxx"));
        assert_eq!(0, Solution::aoc_05_part2("aaa"));
        assert_eq!(0, Solution::aoc_05_part2("uurcxstgmygtbstg"));
        assert_eq!(0, Solution::aoc_05_part2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_05_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input05.txt").expect("Unable to read file");
        assert_eq!(255, Solution::aoc_05_part1(&data));
    }

    #[test]
    fn test_05_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input05.txt").expect("Unable to read file");
        assert_eq!(55, Solution::aoc_05_part2(&data));
    }
}
