use crate::{
    tracer::{hit_record::HitRecord, hittable::Hittable, ray::Ray, vec3::Vec3},
    Interval, Material,
};

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Self {
            center,
            material,
            radius: radius.max(0.0),
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squard();
        let h = ray.direction.dot(oc);
        let c = oc.length_squard() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let t = root;

        let mut hr = HitRecord {
            normal: outward_normal,
            p,
            t,
            front_face: false,
            material: &self.material,
        };

        hr.set_face_normal(ray, outward_normal);

        Some(hr)
    }
}
