pub trait Material {
    fn new(x: u16, y: u16) -> Self
    where
        Self: Sized;

    fn glyph(&self) -> &str {
        return "■";
    }

    fn x(&self) -> u16 {
        return 1;
    }
    fn y(&self) -> u16 {
        return 1;
    }
    fn position(&self) {}

    fn drop(&mut self) {}

    fn settle(&mut self) {}
}

pub struct Sand {
    x: u16,
    y: u16,
}

impl Material for Sand {
    fn new(x: u16, y: u16) -> Sand {
        Sand { x, y }
    }

    fn glyph(&self) -> &str {
        return "■";
    }

    fn x(&self) -> u16 {
        return self.x;
    }

    fn y(&self) -> u16 {
        return self.y;
    }

    fn position(&self) {
        (self.x, self.y);
    }
}

pub struct Water {
    x: u16,
    y: u16,
}

impl Material for Water {
    fn new(x: u16, y: u16) -> Water {
        Water { x, y }
    }

    fn glyph(&self) -> &str {
        return "~";
    }

    fn x(&self) -> u16 {
        return self.x;
    }

    fn y(&self) -> u16 {
        return self.y;
    }

    fn position(&self) {
        (self.x, self.y);
    }
}
