use std::ops::{Add, Index, IndexMut, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        assert_eq!(Vec3::new(-1.0, -2.0, 3.0), -Vec3::new(1.0, 2.0, -3.0));
    }
}
