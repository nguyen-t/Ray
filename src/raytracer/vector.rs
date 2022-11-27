use std::simd::f32x4;
use std::ops;
use std::fmt;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct vec3 {
  e: f32x4
}

impl vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> vec3 {
    return vec3 {
      e: f32x4::from([x, y, z, 0.0])
    };
  }

  pub fn dot(self, v: vec3) -> f32 {
    return self[0] * v[0] + self[1] * v[1] + self[2] * v[2];
  }

  pub fn cross(self, v:vec3) -> vec3 {
    return vec3::new(
      self[1] * v[2] - self[2] * v[1],
      self[2] * v[0] - self[0] * v[2],
      self[0] * v[1] - self[1] * v[0]
    );
  }

  pub fn length(self) -> f32 {
    return self.length_squared().sqrt();
  }

  pub fn length_squared(self) -> f32 {
    let u = self * self;

    return u[0] + u[1] + u[2];
  }

  pub fn unit(self) -> vec3 {
    return self / self.length();
  }

  pub fn rgb(self) -> (u8, u8, u8) {
    let r = 255.999 * self[0];
    let g = 255.999 * self[1];
    let b = 255.999 * self[2];

    return (r as u8, g as u8, b as u8);
  }

  pub fn ycbcr(self) -> (u8, u8, u8) {
    let y  = 16.0 + ((65.738 * self[0]) / 256.0) + ((129.057 * self[1]) / 256.0) + ((25.064 * self[2]) / 256.0);
    let cb = 128.0 - ((37.945 * self[0]) / 256.0) + ((74.494 * self[1]) / 256.0) + ((112.439 * self[2]) / 256.0);
    let cr = 128.0 + ((112.439 * self[0]) / 256.0) + ((94.154 * self[1]) / 256.0) + ((18.285 * self[2]) / 256.0);

    return (y as u8, cb as u8, cr as u8);
  }
}

/* Vector-Vector Operations */
impl ops::Add<vec3> for vec3 {
  type Output = vec3;

  fn add(self, v: vec3) -> vec3 {
    return vec3 {
      e: self.e + v.e
    };
  }
}

impl ops::Sub<vec3> for vec3 {
  type Output = vec3;

  fn sub(self, v: vec3) -> vec3 {
    return vec3 {
      e: self.e - v.e
    };
  }
}

impl ops::Mul<vec3> for vec3 {
  type Output = vec3;

  fn mul(self, v: vec3) -> vec3 {
    return vec3 {
      e: self.e * v.e
    };
  }
}

impl ops::Div<vec3> for vec3 {
  type Output = vec3;

  fn div(self, v: vec3) -> vec3 {
    return vec3 {
      e: self.e / v.e
    };
  }
}

impl ops::AddAssign<vec3> for vec3 {
  fn add_assign(&mut self, v: vec3) {
    self.e += v.e;
  }
}

impl ops::SubAssign<vec3> for vec3 {
  fn sub_assign(&mut self, v: vec3) {
    self.e -= v.e;
  }
}

impl ops::MulAssign<vec3> for vec3 {
  fn mul_assign(&mut self, v: vec3) {
    self.e *= v.e;
  }
}

impl ops::DivAssign<vec3> for vec3 {
  fn div_assign(&mut self, v: vec3) {
    self.e /= v.e;
  }
}

/* Scalar-Vector Operations */
impl ops::Mul<f32> for vec3 {
  type Output = vec3;

  fn mul(self, a: f32) -> vec3 {
    return self * vec3::new(a, a, a);
  }
}

impl ops::Div<f32> for vec3 {
  type Output = vec3;

  fn div(self, a: f32) -> vec3 {
    return self / vec3::new(a, a, a);
  }
}

impl ops::MulAssign<f32> for vec3 {
  fn mul_assign(&mut self, a: f32) {
    self.e *= vec3::new(a, a, a).e;
  }
}

impl ops::DivAssign<f32> for vec3 {
  fn div_assign(&mut self, a: f32) {
    self.e /= vec3::new(a, a, a).e;
  }
}

/* Others */
impl ops::Index<usize> for vec3 {
  type Output = f32;

  fn index(&self, i: usize) -> &f32 {
    return &self.e[i];
  }
}

impl ops::IndexMut<usize> for vec3 {
  fn index_mut(&mut self, i: usize) -> &mut f32 {
    return &mut self.e[i];
  }
}

impl fmt::Display for vec3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "< {}, {}, {} >", self[0], self[1], self[2]);
  }
}