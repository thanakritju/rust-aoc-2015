pub struct Solution {}

impl Solution {
    pub fn aoc_04_part1(strs: &str) -> i32 {
        0
    }

    pub fn aoc_04_part2(strs: &str) -> i32 {
        0
    }

    fn md5(input: &str) -> String {
        let message = input.clone();
        let mut s: [i32; 64] = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17,
            22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12,
            17, 22, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
        ];
        let mut k: [i32; 64] = [0; 64];

        for i in 0..63 {
            k[i] = (2_i64.pow(32) as f32 + (((i + 1) as f32).sin()).abs()).floor() as i32
        }

        let a0: u32 = 0x67452301;
        let b0: u32 = 0xefcdab89;
        let c0: u32 = 0x98badcfe;
        let d0: u32 = 0x10325476;

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(
            "e4d909c290d0fb1ca068ffaddf22cbd0",
            Solution::md5("The quick brown fox jumps over the lazy dog")
        );
        assert_eq!("d41d8cd98f00b204e9800998ecf8427e", Solution::md5(""));
    }

    #[test]
    fn test_04_part1() {
        assert_eq!(609043, Solution::aoc_04_part1("abcdef"));
        assert_eq!(1048970, Solution::aoc_04_part1("pqrstuv"));
    }

    #[test]
    fn test_04_part2() {
        assert_eq!(0, Solution::aoc_04_part2("(())"));
    }

    #[test]
    fn test_04_part1_puzzle() {
        let data = fs::read_to_string("src/solution/input04.txt").expect("Unable to read file");
        assert_eq!(0, Solution::aoc_04_part1(&data));
    }

    #[test]
    fn test_04_part2_puzzle() {
        let data = fs::read_to_string("src/solution/input04.txt").expect("Unable to read file");
        assert_eq!(0, Solution::aoc_04_part2(&data));
    }
}
