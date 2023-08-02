use std::collections::HashMap;

use regex::Regex;

pub struct Solution {}

impl Solution {
    pub fn aoc_07_part1(strs: &str, target: &str) -> u16 {
        let mut map: HashMap<String, u16> = HashMap::new();
        let mixed_pattern = Regex::new(r"(\w+) (AND|LSHIFT|RSHIFT|OR) (\w+) -> (\w+)").unwrap();
        let add_compliment_pattern = Regex::new(r"NOT (\w+) -> (\w+)").unwrap();
        let add_pattern = Regex::new(r"(\w+) -> (\w+)").unwrap();
        let mut lines: Vec<&str> = strs.lines().collect();

        while lines.len() > 0 {
            let line = lines.remove(0);
            match mixed_pattern.captures(line) {
                Some(matched) => {
                    let a = matched.get(1).unwrap().as_str();
                    let a = match a.parse::<u16>() {
                        Ok(v) => v,
                        Err(e) => match map.get(a) {
                            Some(v) => *v,
                            None => {
                                lines.push(line);
                                continue;
                            }
                        },
                    };
                    let b = matched.get(3).unwrap().as_str();
                    let b = match b.parse::<u16>() {
                        Ok(v) => v,
                        Err(e) => match map.get(b) {
                            Some(v) => *v,
                            None => {
                                lines.push(line);
                                continue;
                            }
                        },
                    };
                    let v1;
                    let action = matched.get(2).unwrap().as_str();
                    if action == "LSHIFT" {
                        v1 = a << b;
                    } else if action == "RSHIFT" {
                        v1 = a >> b;
                    } else if action == "OR" {
                        v1 = a | b;
                    } else {
                        v1 = a & b;
                    }

                    let c = matched.get(4).unwrap().as_str();
                    let v2 = match map.get(c) {
                        Some(v) => *v,
                        None => 0,
                    };
                    println!("line: {} v1: {}, v2: {}", line, v1, v2);
                    map.insert(c.to_string(), Self::add(v1, v2));
                    continue;
                }
                None => {}
            }

            match add_compliment_pattern.captures(line) {
                Some(matched) => {
                    let a = matched.get(1).unwrap().as_str();
                    let v1 = !match a.parse::<u16>() {
                        Ok(v) => v,
                        Err(e) => match map.get(a) {
                            Some(v) => *v,
                            None => {
                                lines.push(line);
                                continue;
                            }
                        },
                    };
                    let b = matched.get(2).unwrap().as_str();
                    let v2 = match map.get(b) {
                        Some(v) => *v,
                        None => 0,
                    };
                    println!("line: {} v1: {}, v2: {}", line, v1, v2);
                    map.insert(b.to_string(), v1);
                    continue;
                }
                None => {}
            }

            match add_pattern.captures(line) {
                Some(matched) => {
                    let a = matched.get(1).unwrap().as_str();
                    let v1 = match a.parse::<u16>() {
                        Ok(v) => v,
                        Err(e) => match map.get(a) {
                            Some(v) => *v,
                            None => {
                                lines.push(line);
                                continue;
                            }
                        },
                    };
                    let b = matched.get(2).unwrap().as_str();
                    let v2 = match map.get(b) {
                        Some(v) => *v,
                        None => 0,
                    };
                    println!("line: {} v1: {}, v2: {}", line, v1, v2);
                    map.insert(b.to_string(), Self::add(v1, v2));
                }
                None => {}
            }
        }
        println!("{:?}", map);
        *map.get(target).unwrap()
    }

    fn add(v1: u16, v2: u16) -> u16 {
        ((v1 as u32 + v2 as u32) % 65535).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_07_part1() {
        let instructions = "123 -> x";
        assert_eq!(123, Solution::aoc_07_part1(instructions, "x"));
        let instructions = "123 -> x\n123 -> x";
        assert_eq!(246, Solution::aoc_07_part1(instructions, "x"));
        let instructions = "123 -> x\n234 -> b\nb -> x";
        assert_eq!(357, Solution::aoc_07_part1(instructions, "x"));
        let instructions = "NOT 123 -> x";
        assert_eq!(65412, Solution::aoc_07_part1(instructions, "x"));
        let instructions = "128 -> x\n65412 -> x";
        assert_eq!(5, Solution::aoc_07_part1(instructions, "x"));

        let instructions = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        assert_eq!(72, Solution::aoc_07_part1(instructions, "d"));
        assert_eq!(507, Solution::aoc_07_part1(instructions, "e"));
        assert_eq!(492, Solution::aoc_07_part1(instructions, "f"));
        assert_eq!(114, Solution::aoc_07_part1(instructions, "g"));
        assert_eq!(65412, Solution::aoc_07_part1(instructions, "h"));
        assert_eq!(65079, Solution::aoc_07_part1(instructions, "i"));
        assert_eq!(123, Solution::aoc_07_part1(instructions, "x"));
        assert_eq!(456, Solution::aoc_07_part1(instructions, "y"));
        let instructions = "b -> x\n123 -> x\n234 -> b";
        assert_eq!(357, Solution::aoc_07_part1(instructions, "x"));
    }

    #[test]
    fn test_07_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input07.txt").expect("Unable to read file");
        assert_eq!(3176, Solution::aoc_07_part1(&data, "a"));
    }

    #[test]
    fn test_07_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input07-2.txt").expect("Unable to read file");
        assert_eq!(14710, Solution::aoc_07_part1(&data, "a"));
    }
}
