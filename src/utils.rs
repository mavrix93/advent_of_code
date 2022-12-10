use std::fs::File;
use std::path::Path;

pub fn open_file(file_path: &str) -> File {
    let path = Path::new(file_path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    file
}
