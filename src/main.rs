mod solution;
mod util;

fn main() {
    println!("Welcome to aoc 2015");
    let id = std::env::args().nth(1).expect("no id given");
    util::generate::generate_template(id);
}
