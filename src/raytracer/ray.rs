use super::vector::vec3;

pub struct Ray {
  pub origin: vec3,
  pub direction: vec3
}

impl Ray {
  pub fn new(origin: vec3, direction: vec3) -> Ray {
    return Ray {
      origin: origin,
      direction: direction
    };
  }

  pub fn at(&self, magnitude: f32) -> vec3  {
    return self.direction + self.origin * magnitude;
  }
}