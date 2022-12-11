use crate::day10::observer::Observer;

#[derive(Debug)]
pub enum CpuInstruction {
    Noop,
    AddX(i32),
}

pub struct CrtDisplay {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<PixelValue>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PixelValue {
    Black,
    White,
    NotSet,
}

impl CrtDisplay {
    pub fn new(width: usize, height: usize) -> CrtDisplay {
        CrtDisplay {
            width,
            height,
            pixels: vec![vec![PixelValue::NotSet; width]; height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[y][x] = if value {
            PixelValue::White
        } else {
            PixelValue::Black
        };
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &PixelValue {
        &self.pixels[y][x]
    }

    pub fn print(&self) {
        for row in &self.pixels {
            for pixel in row {
                match pixel {
                    PixelValue::Black => print!("."),
                    PixelValue::White => print!("█"),
                    PixelValue::NotSet => print!("?"),
                }
            }
            println!();
        }
    }
}

impl Observer for CrtDisplay {
    fn observe(&mut self, cycle: i32, value: i32) {
        // println!(
        //     "x: {} y: {} cycle: {} value: {} draw: {}",
        //     cycle % self.width as i32,
        //     cycle / self.width as i32,
        //     cycle,
        //     value,
        //     ((cycle - 1)..(cycle + 2)).contains(&value)
        // );
        let x = (cycle % self.width as i32) as usize;
        let y = (cycle / self.width as i32) as usize;

        let _cycle = cycle % self.width as i32;
        self.set_pixel(x, y, ((_cycle - 1)..(_cycle + 2)).contains(&value));
    }

    fn finished(&mut self) {
        self.print()
    }
}
