use crate::day9::entities::{Direction, Move};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_moves(file: File) -> VecDeque<Move> {
    let reader = BufReader::new(file);

    let mut moves = VecDeque::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");
        let direction = Direction::from_string(parts.next().unwrap().trim());
        let count = parts.next().unwrap().trim().parse::<i32>().unwrap();

        moves.push_back(Move { direction, count });
    }
    moves
}

pub mod debug {
    use crate::day9::entities::{Coordinate, State};

    pub fn get_coordinates_map(coordinates: &Vec<Coordinate>) -> Vec<Vec<String>> {
        let (mut map, offset_coordinates) = _initialize_map(coordinates);
        for (i, coordinate) in coordinates.iter().enumerate() {
            // map[(coordinate.y - offset_coordinates.y) as usize]
            //     [(coordinate.x - offset_coordinates.x) as usize] = format!(
            //     "{}-{}",
            //     i,
            //     map[(coordinate.y - offset_coordinates.y) as usize]
            //         [(coordinate.x - offset_coordinates.x) as usize]
            // );
            map[(coordinate.y - offset_coordinates.y) as usize]
                [(coordinate.x - offset_coordinates.x) as usize] = "#".to_string();
        }

        map
    }

    pub fn render_map(map: &Vec<Vec<String>>) {
        for row in map.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn _initialize_map(coordinates: &Vec<Coordinate>) -> (Vec<Vec<String>>, Coordinate) {
        let mut map: Vec<Vec<String>> = Vec::new();
        let offset = Coordinate::new(
            coordinates.iter().map(|c| c.x).min().unwrap(),
            coordinates.iter().map(|c| c.y).min().unwrap(),
        );
        let max_x = coordinates.iter().map(|c| c.x).max().unwrap() - offset.x;
        let max_y = coordinates.iter().map(|c| c.y).max().unwrap() - offset.y;

        for _ in 0..(max_y + 1) {
            let mut row: Vec<String> = Vec::new();
            for _ in 0..(max_x + 1) {
                row.push(".".to_string());
            }
            map.push(row);
        }
        (map, offset)
    }
}
