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
            x if x == first => self.r = second,
            x if x == second => self.r = first,
            _ => {}
        }
        match self.g {
            x if x == first => self.g = second,
            x if x == second => self.g = first,
            _ => {}
        }
        match self.b {
            x if x == first => self.b = second,
            x if x == second => self.b = first,
            _ => {}
        }
        match self.a {
            x if x == first => self.a = second,
            x if x == second => self.a = first,
            _ => {}
        }
        self
    }
}