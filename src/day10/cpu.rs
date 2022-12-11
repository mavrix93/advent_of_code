use crate::day10::entities;
use crate::day10::observer::Observer;

pub fn signal_strength(values: &Vec<i32>, cycles: &Vec<i32>) -> i32 {
    cycles
        .iter()
        .map(|cycle| values[(cycle - 1) as usize] * cycle)
        .sum::<i32>()
}
pub fn execute_cpu_instructions(
    instructions: &Vec<entities::CpuInstruction>,
    observers: &mut Vec<impl Observer>,
) -> Vec<i32> {
    let mut values = Vec::new();
    let mut cycle = 0;
    let mut value = 1;

    let mut instructions_counter = 0;

    let mut _capacity = 0;
    while instructions_counter < instructions.len() {
        for observer in observers.iter_mut() {
            observer.observe(cycle, value);
        }

        values.push(value);
        _capacity += 1;

        match &instructions[instructions_counter] {
            entities::CpuInstruction::Noop => {
                instructions_counter += 1;
                _capacity = 0;
            }
            entities::CpuInstruction::AddX(x) => {
                if _capacity >= 2 {
                    value += x;
                    instructions_counter += 1;
                    _capacity = 0;
                }
            }
        }
        cycle += 1;
    }

    for observer in observers.iter_mut() {
        observer.finished();
    }
    values
}
