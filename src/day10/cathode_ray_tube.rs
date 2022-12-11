use crate::day10::cpu::{execute_cpu_instructions, signal_strength};
use crate::day10::entities::CrtDisplay;
use crate::day10::helpers::load_cpu_instructions;
use crate::day10::observer::Observer;
use crate::utils::open_file;

pub fn solve_first_part(file_path: &str) {
    let file = open_file(file_path);
    let instructions = load_cpu_instructions(file);

    let x: Vec<Box<dyn Observer>> = Vec::new();
    let values = execute_cpu_instructions(&instructions, &x);

    //println!("Instructions: {:?}", &instructions);

    println!(
        "Signal strength: {:?}",
        signal_strength(&values, &vec![20, 60, 100, 140, 180, 220])
    );
    println!("20: {:?}", &values[19]);
    println!("60: {:?}", &values[59]);
    println!("100: {:?}", &values[99]);
    println!("140: {:?}", &values[139]);
    println!("180: {:?}", &values[179]);
    println!("220: {:?}", &values[219]);
}

pub fn solve_second_part(file_path: &str) {
    let file = open_file(file_path);
    let instructions = load_cpu_instructions(file);

    let mut x = vec![CrtDisplay::new(40, 6)];
    execute_cpu_instructions(&instructions, &mut x);
}
