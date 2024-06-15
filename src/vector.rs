use crate::Unit;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Vector(Unit, Unit, Unit);

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector( 5, 12,  -4);
        let v2 = Vector(-3, 10, -40);
        let v3 = Vector( 2, 22, -44);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vector(-24, 10, 90);
        let v2 = Vector( 50,-17, 36);
        let v3 = Vector(-74, 27, 54);
        assert_eq!(v1 - v2, v3);
    }
}
