use crate::raytracer::vec3::vec3;
use crate::raytracer::ray::Ray;

pub enum Face {
  FRONT,
  BACK
}

pub struct Hit {
  pub face: Face,
  pub point: vec3,
  pub normal: vec3,
  pub t: f32
}

impl Hit {
  pub fn new(point: vec3, normal: vec3, t: f32, r: &Ray) -> Hit {
    let face = if r.direction.dot(normal) < 0.0 { Face::FRONT } else { Face::BACK };
    let normal_face = match face {
      Face::FRONT => normal,
      Face::BACK  => normal * -1.0
    };

    return Hit {
      face: face,
      point: point,
      normal: normal_face,
      t: t
    };
  }
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}