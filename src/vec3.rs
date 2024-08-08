use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.squard_length().sqrt()
    }

    pub fn squard_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();
        Self::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        assert_eq!(Vec3::new(-1.0, -2.0, 3.0), -Vec3::new(1.0, 2.0, -3.0));
    }

    #[test]
    fn test_index() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec[0] = -1.0;
        vec[1] = -2.0;
        vec[2] = -3.0;

        assert_eq!(vec, -Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_index_panic() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert!(std::panic::catch_unwind(|| vec[22]).is_err());
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 3.0);
        assert_eq!(v1 + v2, Vec3::new(3.0, 4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 3.0);
        assert_eq!(v1 - v2, Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_length() {
        assert_eq!(3.0, Vec3::new(2.0, -2.0, 1.0).length());
    }

    #[test]
    fn test_squared_length() {
        assert_eq!(9.0, Vec3::new(2.0, -2.0, 1.0).squard_length());
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.cross(v2), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            -2.0 + 6.0 + 12.0,
            Vec3::new(1.0, 2.0, -3.0).dot(Vec3::new(-2.0, 3.0, -4.0))
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(
            Vec3::new(1.0, 2.0, -2.0).unit(),
            Vec3::new(1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0)
        );
    }
}
