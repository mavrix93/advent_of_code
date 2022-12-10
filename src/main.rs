mod day10;
mod day8;
mod day9;
mod utils;

use crate::day8::tree_top_tree_house;

fn main() {
    // tree_top_tree_house::solve_first_part("src/day8/resources/example_forrest.txt");
    // tree_top_tree_house::solve_first_part("src/day8/resources/forrest.txt");

    // 2016 too long
    // tree_top_tree_house::solve_second_part("src/day8/resources/example_forrest_tree2.txt");
    // tree_top_tree_house::solve_second_part("src/day8/resources/forrest.txt");

    // day9::rope_bridge::solve_first_part("src/day9/resources/example_moves.txt");
    // day9::rope_bridge::solve_first_part("src/day9/resources/moves.txt");

    // 2629 too high
    // day9::rope_bridge::solve_second_part("src/day9/resources/moves.txt");

    // day10::cathode_ray_tube::solve_first_part("src/day10/resources/example_instructions.txt");
    day10::cathode_ray_tube::solve_first_part("src/day10/resources/instructions.txt");
}
