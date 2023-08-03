use std::collections::HashSet;

pub struct _Solution {}

impl _Solution {
    pub fn _aoc_03_part1(strs: &str) -> i32 {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((x, y));
        for c in strs.chars() {
            if c == '^' {
                y += 1
            } else if c == '>' {
                x += 1
            } else if c == '<' {
                x -= 1
            } else {
                y -= 1
            }
            visited.insert((x, y));
        }
        visited.len().try_into().unwrap()
    }

    pub fn _aoc_03_part2(strs: &str) -> i32 {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut x_robot: i32 = 0;
        let mut y_robot: i32 = 0;
        let mut is_robot_turn: bool = false;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((x, y));
        for c in strs.chars() {
            if is_robot_turn {
                if c == '^' {
                    y_robot += 1
                } else if c == '>' {
                    x_robot += 1
                } else if c == '<' {
                    x_robot -= 1
                } else {
                    y_robot -= 1
                }
                visited.insert((x_robot, y_robot));
            } else {
                if c == '^' {
                    y += 1
                } else if c == '>' {
                    x += 1
                } else if c == '<' {
                    x -= 1
                } else {
                    y -= 1
                }
                visited.insert((x, y));
            }
            is_robot_turn = !is_robot_turn
        }
        visited.len().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_03_part1() {
        assert_eq!(2, _Solution::_aoc_03_part1(">"));
        assert_eq!(2, _Solution::_aoc_03_part1("^v^v^v^v^v"));
        assert_eq!(4, _Solution::_aoc_03_part1("^>v<"));
    }

    #[test]
    fn test_03_part2() {
        assert_eq!(3, _Solution::_aoc_03_part2("^v"));
        assert_eq!(11, _Solution::_aoc_03_part2("^v^v^v^v^v"));
        assert_eq!(3, _Solution::_aoc_03_part2("^>v<"));
    }

    #[test]
    fn test_03_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input03.txt").expect("Unable to read file");
        assert_eq!(2572, _Solution::_aoc_03_part1(&data));
    }

    #[test]
    fn test_03_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input03.txt").expect("Unable to read file");
        assert_eq!(2631, _Solution::_aoc_03_part2(&data));
    }
}
