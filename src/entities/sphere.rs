use crate::raytracer::vec3::vec3;
use crate::raytracer::ray::Ray;
use crate::entities::hittable::Hittable;

pub struct Sphere {
  center: vec3,
  radius: f32
}

impl Sphere {
  pub fn new(center: vec3, radius: f32) -> Sphere {
    return Sphere {
      center: center,
      radius: radius
    };
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
    let oc = r.origin - self.center;
    let a = r.direction.length_squared();
    let b_half = oc.dot(r.direction);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = b_half * b_half - a * c;
    let offset = discriminant.sqrt();
    let mut root = (-b_half - offset) / a;

    if root >= t_min && root <= t_max {
      return Some(Hit::new(r.at(root), (r.at(root) - self.center) / self.radius, root, r));
    }

    root = (-b_half + offset) / a;

    if root >= t_min && root <= t_max {
      return Some(Hit::new(r.at(root), (r.at(root) - self.center) / self.radius, root, r));
    }

    return None;
  }
}