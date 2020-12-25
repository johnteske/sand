pub struct Cell {
    pub x: u16,
    pub y: u16,
    pub material: Material,
}

impl Cell {
    pub fn new(material: Material, x: u16, y: u16) -> Self {
        Cell { x, y, material }
    }
}
