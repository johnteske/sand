pub struct Point(pub u16, pub u16);

impl Point {
    pub fn x(&self) -> u16 {
        return self.0;
    }
    pub fn y(&self) -> u16 {
        return self.1;
    }
}
