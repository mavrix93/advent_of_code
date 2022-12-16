use super::entities::Coordinates;
use regex::Regex;

pub fn extract_data(line: &str) -> (Coordinates, Coordinates) {
    let pattern = r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)";
    let re = Regex::new(pattern).unwrap();

    if let Some(captures) = re.captures(line) {
        let sensor_x: i32 = captures["sensor_x"].parse().unwrap();
        let sensor_y: i32 = captures["sensor_y"].parse().unwrap();
        let beacon_x: i32 = captures["beacon_x"].parse().unwrap();
        let beacon_y: i32 = captures["beacon_y"].parse().unwrap();

        return (
            Coordinates::new(sensor_x, sensor_y),
            Coordinates::new(beacon_x, beacon_y),
        );
    }
    panic!("No match found for line: {}", line);
}
