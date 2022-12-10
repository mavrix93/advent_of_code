use crate::day9::entities::{Direction, Move};
use crate::utils::open_file;

use crate::day9::helpers;
use crate::day9::rope::{
    execute_moves, extract_head_coordinates, extract_tail_coordinates, unique_coordinates,
};

pub fn solve_first_part(file_path: &str) {
    let file = open_file(file_path);
    let moves = helpers::load_moves(file);

    let states = execute_moves(moves);

    let tail_coordinates = extract_tail_coordinates(&states);

    println!(
        "Unique coordinates: {:?}",
        unique_coordinates(&tail_coordinates).len()
    );
}
