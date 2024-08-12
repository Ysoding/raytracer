use super::vec3::Vec3;

pub struct HitRecord {
    pub normal: Vec3,
    pub p: Vec3,
    pub t: f64,
}
