use crate::Unit;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector(Unit, Unit, Unit);

impl Vector {
    pub fn build(a: Unit, b: Unit, c: Unit) -> Self {
        Vector(a, b, c)
    }
}

impl Vector {
    pub fn scale(&self, scale: Unit) -> Self {
        Vector(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector( 5.0, 12.0,  -4.0);
        let v2 = Vector(-3.0, 10.0, -40.0);
        let v3 = Vector( 2.0, 22.0, -44.0);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector(-24.0, 10.0, 90.0);
        let v2 = Vector( 50.0,-17.0, 36.0);
        let v3 = Vector(-74.0, 27.0, 54.0);
        assert_eq!(v1 - v2, v3);
    }
}
