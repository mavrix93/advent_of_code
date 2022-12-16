use crate::day15::entities::{Coordinates, Measurement};
use crate::day15::scan::scan_row;
use crate::day15::utils::extract_data;
use regex::Regex;

pub fn solve_first_part(file_name: &str) {
    let measurements = std::fs::read_to_string(file_name)
        .unwrap()
        .split("\n")
        .map(|line| extract_data(line.trim()))
        .map(|(sensor, beacon)| Measurement::new(sensor, beacon))
        .collect::<Vec<_>>();

    let max_distance = measurements
        .iter()
        .map(|m| m.distance + m.beacon.x.abs())
        .max()
        .unwrap();
    println!("max_distance: {:?}", max_distance);
    let cleared_positions_in_row = scan_row(&measurements, 2000000, max_distance as u32);
    println!("cleared_positions_in_row: {}", cleared_positions_in_row);
}
