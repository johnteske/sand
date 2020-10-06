pub trait Material {
    fn new(x: u16, y: u16) -> Self;

    fn drop(&mut self) {
        // if available space
        //self.y += 1;
    }

    fn settle(&mut self) {}
}
