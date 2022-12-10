use crate::day9::entities::{Coordinate, Direction, Move, State};
use std::collections::{HashSet, VecDeque};

fn move_head(rope_move: &Move, coordinates: &Coordinate) -> (Option<Move>, Coordinate) {
    let new_move = rope_move.consume_step();
    match rope_move.direction {
        Direction::Up => (new_move, Coordinate::new(coordinates.x, coordinates.y + 1)),
        Direction::Down => (new_move, Coordinate::new(coordinates.x, coordinates.y - 1)),
        Direction::Left => (new_move, Coordinate::new(coordinates.x - 1, coordinates.y)),
        Direction::Right => (new_move, Coordinate::new(coordinates.x + 1, coordinates.y)),
    }
}

fn move_tail(head_coordinates: &Coordinate, tail_coordinates: &Coordinate) -> Coordinate {
    let tail_coordinates_difference = tail_coordinates.subtract(head_coordinates);

    if tail_coordinates_difference.in_one_radius() {
        return tail_coordinates.clone();
    }
    tail_coordinates_difference.ones_vector() + tail_coordinates.clone()
}

fn move_tails(
    head_coordinates: &Coordinate,
    tail_coordinates: &Vec<Coordinate>,
) -> Vec<Coordinate> {
    let mut _head_coordinates = head_coordinates.clone();
    let mut _tail_coordinates = Vec::new();

    for tail_coordinate in tail_coordinates {
        let new_tail_coordinate = move_tail(&_head_coordinates, tail_coordinate);
        _head_coordinates = tail_coordinate.clone();
        _tail_coordinates.push(new_tail_coordinate)
    }
    _tail_coordinates
}

fn execute_turn(state: State) -> Option<State> {
    let mut new_moves = state.moves.clone();
    match new_moves.pop_front() {
        Some(rope_move) => {
            let (new_move, new_head) = move_head(&rope_move, &state.head);

            if new_move.is_some() {
                new_moves.push_front(new_move.unwrap());
            }
            let new_tails = move_tails(&new_head, &state.tails);

            Some(State {
                head: new_head,
                tails: new_tails,
                moves: new_moves,
            })
        }
        None => None,
    }
}

pub fn execute_moves(
    moves: VecDeque<Move>,
    head: Coordinate,
    tails: Vec<Coordinate>,
) -> Vec<State> {
    let mut state = State { head, tails, moves };
    let mut states = Vec::new();
    states.push(state.clone());
    loop {
        match execute_turn(state) {
            Some(new_state) => {
                states.push(new_state.clone());
                state = new_state;
            }
            None => break,
        }
    }
    states
}

pub fn extract_tail_coordinates(states: &Vec<State>) -> Vec<Vec<Coordinate>> {
    let dim = states[0].tails.len();
    let mut tail_coordinates = Vec::new();
    for i in 0..dim {
        tail_coordinates.push(states.iter().map(|state| state.tails[i].clone()).collect());
    }
    tail_coordinates
}

pub fn extract_head_coordinates(states: &Vec<State>) -> Vec<Coordinate> {
    states.iter().map(|state| state.head.clone()).collect()
}

pub fn unique_coordinates(coordinates: &Vec<Coordinate>) -> HashSet<Coordinate> {
    coordinates.iter().cloned().collect::<HashSet<Coordinate>>()
}
