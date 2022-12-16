use crate::day15::entities::{Coordinates, Measurement};

pub fn scan_row(measurements: &Vec<Measurement>, row: i32, iteration_length: u32) -> u32 {
    _scan_row_in_direction(measurements, row, 0, iteration_length, 1)
        + _scan_row_in_direction(measurements, row, -1, iteration_length, -1)
}

fn _scan_row_in_direction(
    measurements: &Vec<Measurement>,
    row: i32,
    starting_column: i32,
    iteration_length: u32,
    increment: i32,
) -> u32 {
    let mut column = starting_column;
    let mut counter = 0;

    for _ in 0..(iteration_length as i32) {
        if measurements
            .iter()
            .map(|m| (m, m.distance_to(&Coordinates::new(column, row))))
            .filter(|(m, dist)| {
                dist <= &m.distance && dist != &0 && &m.beacon != &Coordinates::new(column, row)
            })
            .count()
            > 0
        {
            counter += 1;
        }

        column += increment;
    }

    counter
}
