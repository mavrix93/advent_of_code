#[derive(Debug)]
pub enum CpuInstruction {
    Noop,
    AddX(i32),
}

#[derive(Debug)]
pub struct CpuState {
    pub value: i32,
    pub cycle: i32,
}

impl CpuState {
    pub fn new() -> Self {
        CpuState { value: 1, cycle: 0 }
    }
}
