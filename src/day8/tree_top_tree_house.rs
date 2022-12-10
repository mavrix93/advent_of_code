use crate::utils;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

const TALLEST_TREE: i32 = 9;

enum Detected {
    HigherTree(i32),
    LowerTree,
    SameTree,
}

enum DetectedHouse {
    AboveHouse,
    HigherTreeAboveHouse,
    LowerTreeBelowHouse,
    HigherTreeBelowHouse(i32),
}

#[derive(Eq, PartialEq, Hash)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Self {
        Coordinates { x, y }
    }
}

pub struct Map {
    heights: Vec<Vec<i32>>,
    dimension: Coordinates,
}

impl Map {
    fn new(heights: Vec<Vec<i32>>) -> Map {
        let dimension = (heights.len(), heights[0].len());
        Map {
            heights,
            dimension: Coordinates {
                x: dimension.0 as i32,
                y: dimension.1 as i32,
            },
        }
    }

    fn get_height(&self, x: i32, y: i32) -> i32 {
        self.heights[x as usize][y as usize]
    }
}

fn read_lines(file: File) -> Vec<Vec<i32>> {
    let reader = BufReader::new(file);
    let mut map = Vec::new();
    for line_result in reader.lines() {
        let line: Vec<i32> = line_result
            .expect("Line error")
            .split("")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().expect("Values of the file should be integers"))
            .collect();

        map.push(line);
    }
    map
}

fn count_trees(
    map: &Map,
    get_coordinates_strategy: impl Fn(&Coordinates, &Coordinates) -> Coordinates,
) -> HashSet<Coordinates> {
    let mut found_trees_set = HashSet::new();
    for i in 0..map.dimension.x {
        let mut previous_height = -1;
        for j in 0..map.dimension.y {
            let coordinates = get_coordinates_strategy(&Coordinates::new(i, j), &map.dimension);
            match get_new_tree_height(&map, previous_height, &coordinates) {
                Detected::HigherTree(height) => {
                    found_trees_set.insert(coordinates);
                    previous_height = height;

                    if height == TALLEST_TREE {
                        break;
                    }
                }
                Detected::LowerTree => (),
                Detected::SameTree => (),
            }
        }
    }
    found_trees_set
}

mod count_strategy {
    use crate::day8::tree_top_tree_house::Coordinates;

    pub fn from_left(coordinates: &Coordinates, dim: &Coordinates) -> Coordinates {
        Coordinates::new(coordinates.x, coordinates.y)
    }
    pub fn from_right(coordinates: &Coordinates, dim: &Coordinates) -> Coordinates {
        Coordinates::new(coordinates.x, dim.y - coordinates.y - 1)
    }
    pub fn from_top(coordinates: &Coordinates, dim: &Coordinates) -> Coordinates {
        Coordinates::new(coordinates.y, coordinates.x)
    }
    pub fn from_bottom(coordinates: &Coordinates, dim: &Coordinates) -> Coordinates {
        Coordinates::new(dim.x - coordinates.y - 1, coordinates.x)
    }
}

mod debug {
    use crate::day8::tree_top_tree_house::{Coordinates, Map};
    use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn print_visible_trees(map: &Map, found_trees_set: &HashSet<Coordinates>) {
        for x in 0..map.dimension.x {
            for y in 0..map.dimension.y {
                if found_trees_set.contains(&Coordinates::new(x, y)) {
                    print!("X");
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }
}

fn get_new_tree_height(map: &Map, previous_height: i32, coordinates: &Coordinates) -> Detected {
    let height = map.get_height(coordinates.x, coordinates.y);
    if height > previous_height {
        return Detected::HigherTree(height);
    } else if height == previous_height {
        return Detected::SameTree;
    };
    Detected::LowerTree
}

fn get_new_tree_height_with_house(
    map: &Map,
    previous_height: i32,
    coordinates: &Coordinates,
    house_height: i32,
) -> DetectedHouse {
    let height = map.get_height(coordinates.x, coordinates.y);

    if height >= house_height && height > previous_height {
        return DetectedHouse::HigherTreeAboveHouse;
    } else if height > house_height {
        return DetectedHouse::AboveHouse;
    } else if height > previous_height {
        return DetectedHouse::HigherTreeBelowHouse(height);
    };
    DetectedHouse::LowerTreeBelowHouse
}

mod tree_count_score {
    use super::*;
    use std::cmp::max;

    fn down(map: &Map, coordinates: &Coordinates, house_height: i32) -> i32 {
        let mut count = 0;
        let mut previous_height = -1;
        for y in (coordinates.y + 1)..map.dimension.y {
            let new_coordinates = Coordinates::new(coordinates.x, y);

            match get_new_tree_height_with_house(
                &map,
                previous_height,
                &new_coordinates,
                house_height,
            ) {
                DetectedHouse::AboveHouse => break,
                DetectedHouse::HigherTreeBelowHouse(height) => {
                    previous_height = height;
                    count += 1;
                }
                DetectedHouse::LowerTreeBelowHouse => (),
                DetectedHouse::HigherTreeAboveHouse => {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
    fn up(map: &Map, coordinates: &Coordinates, house_height: i32) -> i32 {
        let mut count = 0;
        let mut previous_height = -1;
        for y in (0..coordinates.y).rev() {
            let new_coordinates = Coordinates::new(coordinates.x, y);

            match get_new_tree_height_with_house(
                &map,
                previous_height,
                &new_coordinates,
                house_height,
            ) {
                DetectedHouse::AboveHouse => break,
                DetectedHouse::HigherTreeBelowHouse(height) => {
                    previous_height = height;
                    count += 1;
                }
                DetectedHouse::LowerTreeBelowHouse => (),
                DetectedHouse::HigherTreeAboveHouse => {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    fn right(map: &Map, coordinates: &Coordinates, house_height: i32) -> i32 {
        let mut count = 0;
        let mut previous_height = -1;
        for x in (coordinates.x + 1)..map.dimension.x {
            let new_coordinates = Coordinates::new(x, coordinates.y);

            match get_new_tree_height_with_house(
                &map,
                previous_height,
                &new_coordinates,
                house_height,
            ) {
                DetectedHouse::AboveHouse => break,
                DetectedHouse::HigherTreeBelowHouse(height) => {
                    previous_height = height;
                    count += 1;
                }
                DetectedHouse::LowerTreeBelowHouse => (),
                DetectedHouse::HigherTreeAboveHouse => {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    fn left(map: &Map, coordinates: &Coordinates, house_height: i32) -> i32 {
        let mut count = 0;
        let mut previous_height = -1;
        for x in (0..coordinates.x).rev() {
            let new_coordinates = Coordinates::new(x, coordinates.y);

            match get_new_tree_height_with_house(
                &map,
                previous_height,
                &new_coordinates,
                house_height,
            ) {
                DetectedHouse::AboveHouse => break,
                DetectedHouse::HigherTreeBelowHouse(height) => {
                    previous_height = height;
                    count += 1;
                }
                DetectedHouse::LowerTreeBelowHouse => (),
                DetectedHouse::HigherTreeAboveHouse => {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    pub fn get_best_score(map: &Map, show_score_map: bool) -> i32 {
        let mut best_score = 0;
        for x in 0..map.dimension.x {
            for y in 0..map.dimension.y {
                let coordinates = Coordinates { x, y };
                let house_height = map.get_height(x, y);
                let score = right(&map, &coordinates, house_height)
                    * left(&map, &coordinates, house_height)
                    * up(&map, &coordinates, house_height)
                    * down(&map, &coordinates, house_height);

                if show_score_map {
                    // print!("{} ", score);
                    // print!("{}", map.get_height(x, y))
                    print!(
                        "-- ({} {} {} {})[{}] --",
                        up(&map, &coordinates, house_height), // left
                        left(&map, &coordinates, house_height), // up
                        down(&map, &coordinates, house_height), // right
                        right(&map, &coordinates, house_height), // down
                        map.get_height(x, y)
                    );
                }

                best_score = max(best_score, score)
            }
            if show_score_map {
                println!("")
            }
        }
        best_score
    }
}
pub fn solve_first_part(file_path: &str) {
    let file = utils::open_file(file_path);
    let map = Map::new(read_lines(file));

    let visible_trees: HashSet<Coordinates> = count_trees(&map, count_strategy::from_left)
        .into_iter()
        .chain(count_trees(&map, count_strategy::from_right))
        .chain(count_trees(&map, count_strategy::from_top))
        .chain(count_trees(&map, count_strategy::from_bottom))
        .collect();

    println!("Number of visible trees: {}", &visible_trees.len());
    debug::print_visible_trees(&map, &visible_trees);
}

pub fn solve_second_part(file_path: &str) {
    let file = utils::open_file(file_path);
    let map = Map::new(read_lines(file));

    let best_score = tree_count_score::get_best_score(&map, false);
    println!("Score: {}", best_score);
}
