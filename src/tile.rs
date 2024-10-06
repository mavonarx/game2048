use std::fmt;

#[derive(Copy, Clone)]
pub struct Tile {
    exponent: u32,
    merged: bool,
}

impl Tile {
    // Constructor
    pub fn new(exponent: u32) -> Self {
        Self {
            exponent,
            merged: false,
        }
    }

    pub fn print(&self) {
        println!("{}", self.get_number());
    }

    pub fn get_number(&self) -> u32 {
        (2 as u32).pow(self.exponent)
    }

    pub fn double(&mut self) {
        self.exponent += 1;
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.get_number() == 1 {
            write!(f, " ")
        } else {
            write!(f, "{}", self.get_number())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        let tile: Tile = Tile::new(2);
        assert_eq!(tile.get_number(), 4);
    }
}
