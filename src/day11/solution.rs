use crate::day11::helpers::load_monkeys;
use crate::utils::open_file;

pub fn solve_first_part(file_path: &str) {
    let file = open_file(file_path);
    let monkeys = load_monkeys(file);
    println!("Monkeys: {:?}", monkeys);
}
