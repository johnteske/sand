trait Entity {
    fn glyph(&self) -> &str {
        return "■";
    }
}

pub struct Sand {
    pub x: u16,
    pub y: u16,
}
impl Entity for Sand {}

pub enum Material {
    Sand(Sand),
}
