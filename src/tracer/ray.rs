use crate::Vec3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use super::Vec3;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.at(2.0), Vec3::new(3.0, 5.0, 7.0));
    }
}
