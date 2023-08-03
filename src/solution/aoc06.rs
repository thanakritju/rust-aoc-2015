use regex::Regex;

pub struct _Solution {}

impl _Solution {
    pub fn _aoc_06_part1(strs: &str) -> i32 {
        let mut state = vec![vec![false; 1000]; 1000];
        for line in strs.lines() {
            let re =
                Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let action = caps.get(1).unwrap().as_str();
            let x1: usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let y1: usize = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let x2: usize = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let y2: usize = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
            if action == "turn on" {
                for i in x1..(x2 + 1) {
                    for j in y1..(y2 + 1) {
                        state[i][j] = true
                    }
                }
            } else if action == "turn off" {
                for i in x1..(x2 + 1) {
                    for j in y1..(y2 + 1) {
                        state[i][j] = false
                    }
                }
            } else {
                for i in x1..(x2 + 1) {
                    for j in y1..(y2 + 1) {
                        state[i][j] = !state[i][j]
                    }
                }
            }
        }
        let mut sum = 0;
        for row in state.iter() {
            for col in row.iter() {
                if *col {
                    sum += 1
                }
            }
        }
        sum
    }

    pub fn _aoc_06_part2(strs: &str) -> i64 {
        let mut state = vec![vec![0 as u32; 1000]; 1000];
        for line in strs.lines() {
            let re =
                Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let action = caps.get(1).unwrap().as_str();
            let x1: usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let y1: usize = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let x2: usize = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let y2: usize = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
            println!("{} {} {} {} {} {}", line, action, x1, y1, x2, y2);
            if action == "turn on" {
                for i in x1..(x2 + 1) {
                    for j in y1..(y2 + 1) {
                        state[i][j] += 1
                    }
                }
            } else if action == "turn off" {
                for i in x1..(x2 + 1) {
                    for j in y1..(y2 + 1) {
                        if state[i][j] >= 1 {
                            state[i][j] -= 1
                        }
                    }
                }
            } else {
                for i in x1..(x2 + 1) {
                    for j in y1..(y2 + 1) {
                        state[i][j] += 2
                    }
                }
            }
        }
        let mut sum = 0;
        for row in state.iter() {
            for col in row.iter() {
                sum += col
            }
        }
        sum.into()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_06_part1() {
        assert_eq!(
            1000 * 1000,
            _Solution::_aoc_06_part1("turn on 0,0 through 999,999")
        );
        assert_eq!(1000, _Solution::_aoc_06_part1("toggle 0,0 through 999,0"));
        assert_eq!(
            0,
            _Solution::_aoc_06_part1("toggle 0,0 through 999,0\ntoggle 0,0 through 999,0")
        );
        assert_eq!(
            0,
            _Solution::_aoc_06_part1("turn off 499,499 through 500,500")
        );
        assert_eq!(
            1000 * 1000 - 4,
            _Solution::_aoc_06_part1(
                "toggle 0,0 through 999,999\nturn off 499,499 through 500,500"
            )
        );
    }

    #[test]
    fn test_06_part2() {
        assert_eq!(
            2000000,
            _Solution::_aoc_06_part2("toggle 0,0 through 999,999")
        );
        assert_eq!(1, _Solution::_aoc_06_part2("turn on 0,0 through 0,0"));
        assert_eq!(
            2000000,
            _Solution::_aoc_06_part2("turn off 0,0 through 999,999\ntoggle 0,0 through 999,999")
        );
    }

    #[test]
    fn test_06_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input06.txt").expect("Unable to read file");
        assert_eq!(400410, _Solution::_aoc_06_part1(&data));
    }

    #[test]
    fn test_06_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input06.txt").expect("Unable to read file");
        assert_eq!(15343601, _Solution::_aoc_06_part2(&data));
    }
}
