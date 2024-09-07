use crate::{Ray, Vec3};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scatter_ray: Ray,
}
