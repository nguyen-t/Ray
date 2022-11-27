#![feature(portable_simd)]

mod formats;
mod raytracer;
mod entities;

use formats::{PPM, Image};
use raytracer::*;
use entities::*;

const IMG_HEIGHT: usize = 720;
const IMG_WIDTH: usize = 1280;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ((IMG_WIDTH as f32) / (IMG_HEIGHT as f32)) * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn ray_color(r: &Ray) -> vec3 {
  let unit_direction = r.direction.unit();
  let t = 0.5 * (unit_direction[1] + 1.0);
  
  return vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
  let mut ppm: PPM<IMG_HEIGHT, IMG_WIDTH> = PPM::new("raytrace.ppm");
  let mut img: Image<IMG_HEIGHT, IMG_WIDTH> = Image::new();

  let origin = vec3::new(0.0, 0.0, 0.0);
  let horizontal = vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
  let vertical = vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
  let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - vec3::new(0.0, 0.0, FOCAL_LENGTH);

  for h in (0..IMG_HEIGHT).rev() {
    for w in (0..IMG_WIDTH).rev() {
      let u = (w as f32) / ((IMG_WIDTH - 1) as f32);
      let v = (h as f32) / ((IMG_HEIGHT - 1) as f32);
      let r = Ray::new(origin, lower_left + horizontal * u + vertical * v - origin);
      let pixel = ray_color(&r).rgb();

      img.draw(&pixel);
    }
  }

  ppm.draw(img.raw());

}
