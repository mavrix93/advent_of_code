use crate::day11::actions::{execute_n_turns, execute_turns};
use crate::day11::helpers::load_monkeys;
use crate::utils::open_file;

pub fn solve_first_part(file_path: &str) {
    let file = open_file(file_path);
    let mut monkeys = load_monkeys(file);
    execute_n_turns(&mut monkeys, 1);
    println!("Monkeys: {:?}", monkeys);
}
