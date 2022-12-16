use crate::day15::entities::{Coordinates, Measurement, ScannedPosition};
use progressing::{
    // Just handy names for the examples below
    bernoulli::Bar as BernoulliBar,
    clamping::Bar as ClampingBar,
    mapping::Bar as MappingBar,
    // The underlying Trait
    Baring,
};

pub fn scan_row(
    measurements: &Vec<Measurement>,
    row: i32,
    iteration_boundary: (i32, i32),
) -> (u32, Vec<Coordinates>) {
    let mut possible_positions = vec![];
    let mut scanned_counter = 0;

    for column in iteration_boundary.0..iteration_boundary.1 {
        let scanned_position = measurements
            .iter()
            .map(|m| (m, m.distance_to(&Coordinates::new(column, row))))
            .map(|(m, d)| {
                if d == 0 || &m.beacon == &Coordinates::new(column, row) {
                    ScannedPosition::Occupied
                } else if &d <= &m.distance {
                    ScannedPosition::Scanned
                } else {
                    ScannedPosition::Clear
                }
            })
            .reduce(|value, previous_value| match (value, previous_value) {
                (ScannedPosition::Clear, ScannedPosition::Clear) => ScannedPosition::Clear,
                (ScannedPosition::Clear, ScannedPosition::Scanned) => ScannedPosition::Scanned,
                (ScannedPosition::Scanned, ScannedPosition::Clear) => ScannedPosition::Scanned,
                (ScannedPosition::Scanned, ScannedPosition::Scanned) => ScannedPosition::Scanned,
                (ScannedPosition::Clear, ScannedPosition::Occupied) => ScannedPosition::Occupied,
                (ScannedPosition::Occupied, ScannedPosition::Clear) => ScannedPosition::Occupied,
                (ScannedPosition::Occupied, ScannedPosition::Scanned) => ScannedPosition::Occupied,
                (ScannedPosition::Scanned, ScannedPosition::Occupied) => ScannedPosition::Occupied,
                (ScannedPosition::Occupied, ScannedPosition::Occupied) => ScannedPosition::Occupied,
            })
            .unwrap_or(ScannedPosition::Clear);
        match scanned_position {
            ScannedPosition::Clear => possible_positions.push(Coordinates::new(column, row)),
            ScannedPosition::Scanned => scanned_counter += 1,
            ScannedPosition::Occupied => (),
        }
    }

    (scanned_counter, possible_positions)
}

pub fn scan_rows(
    measurements: &Vec<Measurement>,
    row_iteration_boundary: (i32, i32),
    column_iteration_boundary: (i32, i32),
) -> Option<Coordinates> {
    let mut progress_bar =
        MappingBar::with_range(row_iteration_boundary.0, row_iteration_boundary.1).timed();
    progress_bar.set_len(100);
    for row in (row_iteration_boundary.0..row_iteration_boundary.1) {
        let possible_positions = scan_row(measurements, row, column_iteration_boundary).1;
        if possible_positions.len() > 0 {
            if possible_positions.len() > 1 {
                panic!("More than one possible position found")
            }
            let possible_position = possible_positions.first().copied();
            println!("Found possible position: {:?}", possible_position);
            return possible_position;
        }
        progress_bar.set(row);
        println!("[{}]: {}", row_iteration_boundary.0, progress_bar);
    }
    None
}

pub fn scan_rows_multy_thread(
    measurements: &Vec<Measurement>,
    row_iteration_boundary: (i32, i32),
    column_iteration_boundary: (i32, i32),
    n_threads: usize,
) {
    let length = row_iteration_boundary.1 - row_iteration_boundary.0;
    let chunk_size = length / n_threads as i32;
    let mut handles = vec![];
    let lengths = (0..n_threads)
        .map(|i| {
            let start = row_iteration_boundary.0 + ((i as i32) * chunk_size);
            let end = if i == n_threads - 1 {
                row_iteration_boundary.1
            } else {
                start + chunk_size
            };
            (start, end)
        })
        .collect::<Vec<_>>();
    println!("Lengths: {:?}", lengths);
    for i in 0..n_threads {
        let lengths = lengths.clone();
        let measurements = measurements.clone();
        handles.push(std::thread::spawn(move || {
            scan_rows(&measurements, lengths[i], column_iteration_boundary)
        }));
    }
    println!("Waiting for threads to finish");
    for handle in handles {
        match handle.join() {
            Ok(possible_position) => {
                if let Some(possible_position) = possible_position {
                    println!("Found possible position: {:?}", possible_position);
                    return;
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
