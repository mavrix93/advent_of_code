use crate::day9::entities::{Coordinate, Direction, Move};
use crate::utils::open_file;

use crate::day9::helpers;
use crate::day9::helpers::debug::{get_coordinates_map, render_map};
use crate::day9::rope::{
    execute_moves, extract_head_coordinates, extract_tail_coordinates, unique_coordinates,
};

pub fn solve_first_part(file_path: &str) {
    let file = open_file(file_path);
    let moves = helpers::load_moves(file);

    let states = execute_moves(moves, Coordinate::new(0, 0), vec![Coordinate::new(0, 0)]);

    let tail_coordinates = extract_tail_coordinates(&states);
    assert!(tail_coordinates.len() == 1);

    let tail_coordinate = tail_coordinates.last().unwrap();

    println!(
        "Unique tail coordinates: {:?}",
        unique_coordinates(tail_coordinate).len()
    );
}

pub fn solve_second_part(file_path: &str) {
    let file = open_file(file_path);
    let moves = helpers::load_moves(file);

    let states = execute_moves(moves, Coordinate::new(0, 0), vec![Coordinate::new(0, 0); 9]);

    let tail_coordinates = extract_tail_coordinates(&states);

    let tail_coordinate = tail_coordinates.last().unwrap();

    println!(
        "Unique tail coordinates: {:?}",
        unique_coordinates(tail_coordinate).len()
    );

    render_map(&get_coordinates_map(tail_coordinate));

    // render_map(&get_coordinates_map(&extract_head_coordinates(&states)));
}
