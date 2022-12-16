#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub sensor: Coordinates,
    pub beacon: Coordinates,
    pub distance: i32,
}

pub enum ScannedPosition {
    Clear,
    Occupied,
    Scanned,
}

impl Measurement {
    pub fn new(sensor: Coordinates, beacon: Coordinates) -> Self {
        let distance = Self::_distance(&sensor, &beacon);
        Self {
            sensor,
            beacon,
            distance,
        }
    }

    pub fn distance_to(&self, coordinates: &Coordinates) -> i32 {
        Self::_distance(&self.sensor, coordinates)
    }

    fn _distance(sensor: &Coordinates, beacon: &Coordinates) -> i32 {
        (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs()
    }
}
