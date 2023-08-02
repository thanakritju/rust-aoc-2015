use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

pub fn generate_template(id: String) -> bool {
    println!("Generating aoc2015 template for id {id}");
    let filename = format!("aoc{:02}.rs", id);
    let data_filename = format!("input{:02}.txt", id);
    println!("Filename: {filename}");
    update_mod_file(&id);
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
    pub fn aoc_{id}_part1(strs: &str) -> i32 {{
        0
    }}

    pub fn aoc_{id}_part2(strs: &str) -> i32 {{
        0
    }}
}}

#[cfg(test)]
mod tests {{
    use std::fs;

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
        assert_eq!(0, Solution::aoc_{id}_part1(&data));
    }}

    #[test]
    fn test_{id}_part2_puzzle() {{
        let data = fs::read_to_string(\"src/solution/input{id}.txt\").expect(\"Unable to read file\");
        assert_eq!(0, Solution::aoc_{id}_part2(&data));
    }}
}}",
            id = id
        )
        .as_bytes(),
    )
    .expect("Error while writing to file");
}

fn create_input_file(filename: String) {
    let mut _file = File::create(format!("src/solution/{}", filename))
        .expect("Error encountered while creating file!");
}

fn update_mod_file(id: &String) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open("src/solution/mod.rs")
        .unwrap();

    if let Err(e) = writeln!(file, "mod aoc{};", id) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let reader = BufReader::new(&mut file);

    let mut lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line"))
        .collect();
    lines.dedup();

    let mut file = OpenOptions::new()
        .write(true)
        .open("src/solution/mod.rs")
        .unwrap();

    for line in lines {
        file.write_all(line.as_bytes())
            .expect("Couldn't write to file");
    }
}
