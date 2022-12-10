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

fn move_tail(head_coordinates: &Coordinate, tail_coordinates: &Coordinate) -> Option<Coordinate> {
    let tail_coordinates_difference = tail_coordinates.subtract(head_coordinates);

    if tail_coordinates_difference.in_one_radius() {
        return None;
    }
    Some(tail_coordinates_difference.ones_vector())
}

fn execute_turn(state: State) -> Option<State> {
    let mut new_moves = state.moves.clone();
    match new_moves.pop_front() {
        Some(rope_move) => {
            let (new_move, new_head) = move_head(&rope_move, &state.head);
            let tail_movement = move_tail(&new_head, &state.tail);

            if new_move.is_some() {
                new_moves.push_front(new_move.unwrap());
            }

            Some(State {
                head: new_head,
                tail: tail_movement.unwrap_or(Coordinate::zero()) + state.tail,
                moves: new_moves,
            })
        }
        None => None,
    }
}

pub fn execute_moves(moves: VecDeque<Move>) -> Vec<State> {
    let mut state = State {
        head: Coordinate::new(0, 0),
        tail: Coordinate::new(0, 0),
        moves,
    };
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

pub fn extract_tail_coordinates(states: &Vec<State>) -> Vec<Coordinate> {
    states
        .iter()
        .map(|state| state.tail.clone())
        .collect::<Vec<Coordinate>>()
}

pub fn extract_head_coordinates(states: &Vec<State>) -> Vec<Coordinate> {
    states
        .iter()
        .map(|state| state.head.clone())
        .collect::<Vec<Coordinate>>()
}

pub fn unique_coordinates(coordinates: &Vec<Coordinate>) -> HashSet<Coordinate> {
    coordinates.iter().cloned().collect::<HashSet<Coordinate>>()
}
