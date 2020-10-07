pub trait Material {
    fn new(x: u16, y: u16) -> Self;

    fn drop(&mut self) {}

    fn settle(&mut self) {}
}
