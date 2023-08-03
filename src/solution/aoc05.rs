pub struct _Solution {}

impl _Solution {
    pub fn _aoc_05_part1(strs: &str) -> i32 {
        let mut count = 0;
        for line in strs.lines() {
            if !_Solution::_is_forbidden(line)
                && _Solution::_same_letter(line)
                && _Solution::_exceeds_vowels(line)
            {
                count += 1
            }
        }
        count
    }

    fn _same_letter(s: &str) -> bool {
        let ans = s
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|x| x[0] == x[1]);
        ans
    }

    fn _is_forbidden(s: &str) -> bool {
        let forbidden_list = ["ab", "cd", "pq", "xy"];
        let ans = s.chars().collect::<Vec<char>>().windows(2).any(|y| {
            forbidden_list
                .iter()
                .position(|&x| x == format!("{}{}", y[0], y[1]))
                .is_some()
        });
        ans
    }

    fn _exceeds_vowels(s: &str) -> bool {
        let vowels = "aeiou";
        let mut vowels_count = 0;
        for c in s.chars().collect::<Vec<char>>() {
            if vowels.chars().position(|x| x == c).is_some() {
                vowels_count += 1
            }
        }
        vowels_count >= 3
    }

    pub fn _aoc_05_part2(strs: &str) -> i32 {
        let mut count = 0;
        for line in strs.lines() {
            if _Solution::_is_sandwich(line) && _Solution::_a_pair_duplicate(line) {
                count += 1
            }
        }
        count
    }

    fn _is_sandwich(s: &str) -> bool {
        let ans = s
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .any(|x| x[0] == x[2]);
        ans
    }

    fn _a_pair_duplicate(s: &str) -> bool {
        for (i, w1) in s.chars().collect::<Vec<char>>().windows(2).enumerate() {
            for (j, w2) in s.chars().collect::<Vec<char>>().windows(2).enumerate() {
                if i == j || i == j + 1 || i + 1 == j {
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
        assert_eq!(true, _Solution::_is_forbidden("haegwjzuvuyypxy"));
        assert_eq!(true, _Solution::_is_forbidden("xyhaegwjzuvuyyp"));
        assert_eq!(false, _Solution::_is_forbidden("pa"));
    }

    #[test]
    fn test_same_letter() {
        assert_eq!(false, _Solution::_same_letter("abc"));
        assert_eq!(true, _Solution::_same_letter("aa"));
        assert_eq!(false, _Solution::_same_letter("pa"));
    }

    #[test]
    fn test_05_part1() {
        assert_eq!(1, _Solution::_aoc_05_part1("ugknbfddgicrmopn"));
        assert_eq!(4, _Solution::_aoc_05_part1("aaa\naaa\naaa\naaa"));
        assert_eq!(1, _Solution::_aoc_05_part1("aaccbbdde"));
        assert_eq!(0, _Solution::_aoc_05_part1("aaab"));
        assert_eq!(1, _Solution::_aoc_05_part1("iii"));
        assert_eq!(0, _Solution::_aoc_05_part1("xazegov"));
        assert_eq!(0, _Solution::_aoc_05_part1("jchzalrnumimnmhp"));
        assert_eq!(0, _Solution::_aoc_05_part1("haegwjzuvuyypxy"));
        assert_eq!(0, _Solution::_aoc_05_part1("dvszwmarrgswjxmb"));
        assert_eq!(
            1,
            _Solution::_aoc_05_part1("jchzalrnumimnmhp\naaa\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\n")
        );
    }

    #[test]
    fn test_05_part2() {
        assert_eq!(1, _Solution::_aoc_05_part2("qjhvhtzxzqqjkmpb"));
        assert_eq!(1, _Solution::_aoc_05_part2("xxyxx"));
        assert_eq!(0, _Solution::_aoc_05_part2("aaa"));
        assert_eq!(0, _Solution::_aoc_05_part2("uurcxstgmygtbstg"));
        assert_eq!(0, _Solution::_aoc_05_part2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_05_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input05.txt").expect("Unable to read file");
        assert_eq!(255, _Solution::_aoc_05_part1(&data));
    }

    #[test]
    fn test_05_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input05.txt").expect("Unable to read file");
        assert_eq!(55, _Solution::_aoc_05_part2(&data));
    }
}
