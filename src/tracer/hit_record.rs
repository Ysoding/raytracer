use super::{ray::Ray, vec3::Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub normal: Vec3,
    pub p: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(normal: Vec3, p: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord {
            normal,
            p,
            t,
            front_face,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
