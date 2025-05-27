#[derive(Debug, Copy, Clone)]
pub struct ValueBounds {
    pub min: u32,
    pub max: u32,
}

impl ValueBounds {
    pub fn new(min: u32, max: u32) -> ValueBounds {
        ValueBounds { min, max }
    }
}
