use std::{fs::File, io::Write};

pub fn generate_template(id: String) -> bool {
    println!("Generating aoc2015 template for id {id}");
    let filename = format!("aoc{:02}.rs", id);
    let data_filename = format!("input{:02}.txt", id);
    println!("Filename: {filename}");
    create_solution_file(filename, id);
    create_input_file(data_filename);
    true
}

fn create_solution_file(filename: String, id: String) {
    let mut file = File::create(format!("src/solution/{}", filename))
        .expect("Error encountered while creating file!");
    file.write_all(
        format!(
            "pub struct Solution {{}}

impl Solution {{
    pub fn aoc_{id}_part1(nums: strs: &str) -> i32{{
        vec![]
    }}
    pub fn aoc_{id}_part2(nums: strs: &str) -> i32 {{
        vec![]
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_{id}_part1() {{
        assert_eq!(0, Solution::aoc_{id}_part1(\"(())\"));
    }}

    #[test]
    fn test_{id}_part2() {{
        assert_eq!(0, Solution::aoc_{id}_part2(\"(())\"));
    }}

    #[test]
    fn test_{id}_part1_puzzle() {{
        let data = fs::read_to_string(\"src/solution/input{id}.txt\").expect(\"Unable to read file\");
        assert_eq!(280, Solution::aoc_{id}_part1(&data));
    }}

    #[test]
    fn test_{id}_part2_puzzle() {{
        let data = fs::read_to_string(\"src/solution/input{id}.txt\").expect(\"Unable to read file\");
        assert_eq!(280, Solution::aoc_{id}_part2(&data));
    }}
}}",
            id = id
        )
        .as_bytes(),
    )
    .expect("Error while writing to file");
}

fn create_input_file(filename: String) {
    let mut file = File::create(format!("src/solution/{}", filename))
        .expect("Error encountered while creating file!");
}
