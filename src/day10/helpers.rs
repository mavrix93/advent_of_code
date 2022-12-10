use crate::day10::entities::*;
use std::fs::File;
use std::io::BufRead;

pub fn load_cpu_instructions(file: File) -> Vec<CpuInstruction> {
    let mut instructions = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        let _line = line.unwrap();
        let mut parts = _line.trim().split(" ");
        match parts.next().unwrap().trim() {
            "noop" => instructions.push(CpuInstruction::Noop),
            "addx" => instructions.push(CpuInstruction::AddX(
                parts.next().unwrap().trim().parse::<i32>().unwrap(),
            )),
            _ => panic!("Unknown instruction"),
        }
    }
    instructions
}
