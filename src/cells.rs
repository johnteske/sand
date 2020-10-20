pub enum Material {
    Bedrock,
    Sand,
    // Water,
}

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

pub struct Cells {
    pub cells: Vec<Cell>, // TODO make private
}

impl Cells {
    pub fn new(width: u16, height: u16) -> Cells {
        let size = width * height;
        let mut cells: Vec<Cell> = Vec::with_capacity(size as usize);

        // TODO how can this use its own method to add
        // add a single-cell border
        // to avoid the need to boundary-check
        for i in 0..height {
            cells.push(Cell::new(Material::Bedrock, 0, i));
            cells.push(Cell::new(Material::Bedrock, width + 1, i));
        }
        for i in 1..width {
            cells.push(Cell::new(Material::Bedrock, i, 0));
            cells.push(Cell::new(Material::Bedrock, i, height + 1));
        }

        Cells { cells }
    }

    // can be "set" if 2d array is used
    pub fn add(&mut self, x: u16, y: u16, material: Material) {
        self.cells.push(Cell::new(material, x, y));
    }

    pub fn get(&self, x: u16, y: u16) -> Option<usize> {
        self.cells.iter().position(|c| c.x == x && c.y == y)
    }

    // pub fn neighbors_of(&self, x:u16, y:u16) -> {}
}
