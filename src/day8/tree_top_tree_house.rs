use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

enum Detected {
    HigherTree(i32),
    LowerTree,
    SameTree,
}

fn open_file(file_path: &str) -> File {
    let path = Path::new(file_path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    file
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
    map: &Vec<Vec<i32>>,
    get_coordinates_strategy: impl Fn((usize, usize), (usize, usize)) -> (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut found_trees_set = HashSet::new();
    let dim = (map.len(), map[0].len());
    for i in 0..dim.0 {
        let mut previous_height = -1;
        for j in 0..dim.1 {
            let coordinates = get_coordinates_strategy((i, j), dim);
            match get_new_tree_height(map, previous_height, coordinates) {
                Detected::HigherTree(height) => {
                    found_trees_set.insert(coordinates);
                    previous_height = height;
                }
                Detected::LowerTree => (),
                Detected::SameTree => (),
            }
        }
    }
    found_trees_set
}

mod count_strategy {
    pub fn from_left(coordinates: (usize, usize), dim: (usize, usize)) -> (usize, usize) {
        coordinates
    }
    pub fn from_right(coordinates: (usize, usize), dim: (usize, usize)) -> (usize, usize) {
        (coordinates.0, dim.1 - coordinates.1 - 1)
    }
    pub fn from_top(coordinates: (usize, usize), dim: (usize, usize)) -> (usize, usize) {
        (coordinates.1, coordinates.0)
    }
    pub fn from_bottom(coordinates: (usize, usize), dim: (usize, usize)) -> (usize, usize) {
        (dim.0 - coordinates.1 - 1, coordinates.0)
    }
}

pub mod debug {
    use std::collections::HashSet;

    pub fn print_visible_trees(map: &Vec<Vec<i32>>, found_trees_set: &HashSet<(usize, usize)>) {
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if found_trees_set.contains(&(i, j)) {
                    print!("X");
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }
}

fn get_new_tree_height(
    map: &Vec<Vec<i32>>,
    previous_height: i32,
    coordinates: (usize, usize),
) -> Detected {
    let height = map[coordinates.0][coordinates.1];
    if height > previous_height {
        return Detected::HigherTree(height);
    } else if height == previous_height {
        return Detected::SameTree;
    };
    Detected::LowerTree
}

pub fn solve(file_path: &str) {
    let file = open_file(file_path);
    let map = read_lines(file);

    let visible_trees: HashSet<(usize, usize)> = count_trees(&map, count_strategy::from_left)
        .into_iter()
        .chain(count_trees(&map, count_strategy::from_right))
        .chain(count_trees(&map, count_strategy::from_top))
        .chain(count_trees(&map, count_strategy::from_bottom))
        .collect();

    println!("Number of visible trees: {}", &visible_trees.len());
    debug::print_visible_trees(&map, &visible_trees);
}
