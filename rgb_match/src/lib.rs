#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match self.r {
            first => self.r = second,
            second => self.r = first,
        }
        match self.g {
            first => self.g = second,
            second => self.g = first,
        }
        match self.b {
            first => self.b = second,
            second => self.b = first,
        }
        match self.a {
            first => self.a = second,
            second => self.a = first,
        }
        self
    }
}