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

    pub fn get_coordinates_map(
        head_coordinates: &Vec<Coordinate>,
        tail_coordinates: &Vec<Coordinate>,
    ) -> Vec<Vec<String>> {
        let max_x = tail_coordinates.iter().map(|c| c.x.abs()).max().unwrap();
        let max_y = tail_coordinates.iter().map(|c| c.y.abs()).max().unwrap();

        println!("Max x: {}, max y: {}", max_x, max_y);

        let mut map = _initialize_map(head_coordinates);
        for (i, coordinate) in head_coordinates.iter().enumerate() {
            map[coordinate.y as usize][coordinate.x as usize] = format!(
                "{}H{}-",
                map[coordinate.y as usize][coordinate.x as usize], i
            );
        }

        // for (i, coordinate) in tail_coordinates.iter().enumerate() {
        //     map[coordinate.y as usize][coordinate.x as usize] = format!(
        //         "{}T{}-",
        //         map[coordinate.y as usize][coordinate.x as usize], i
        //     );
        // }
        map
    }

    pub fn render_map(map: &Vec<Vec<String>>) {
        for row in map.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }

    fn _initialize_map(coordinates: &Vec<Coordinate>) -> Vec<Vec<String>> {
        let mut map: Vec<Vec<String>> = Vec::new();
        let max_x = coordinates.iter().map(|c| c.x).max().unwrap();
        let max_y = coordinates.iter().map(|c| c.y).max().unwrap();

        for _ in 0..(max_y + 1) {
            let mut row: Vec<String> = Vec::new();
            for _ in 0..(max_x + 1) {
                row.push(".".to_string());
            }
            map.push(row);
        }
        map
    }
}
