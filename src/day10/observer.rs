pub trait Observer {
    fn observe(&mut self, cycle: i32, value: i32);

    fn finished(&mut self) {}
}
