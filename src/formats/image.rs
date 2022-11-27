use std::ops::{Index, IndexMut};

pub struct Image<const H: usize, const W: usize> {
  data: [[[u8; 3]; W]; H],
  offset: usize
}

impl<const H: usize, const W: usize> Image<H, W> {
  pub fn new() -> Image<H, W> {
    return Image {
      data: [[[0; 3]; W]; H],
      offset: 0
    };
  }

  pub fn draw(&mut self, pixel: &(u8, u8, u8)) {
    let offset_h = self.offset / W;
    let offset_w = self.offset % W;

    self.data[offset_h][offset_w][0] = pixel.0;
    self.data[offset_h][offset_w][1] = pixel.1;
    self.data[offset_h][offset_w][2] = pixel.2;
    self.offset += 1;

    if self.offset == W * H {
      self.offset = 0;
    }
  }

  pub fn raw(&self) -> &[[[u8; 3]; W]; H] {
    return &self.data;
  }
}

impl<const H: usize, const W: usize> Index<(usize, usize, usize)> for Image<H, W> {
  type Output = u8;

  fn index(&self, index: (usize, usize, usize)) -> &u8 {
    return &self.data[index.0][index.1][index.2];
  }
}

impl<const H: usize, const W: usize> IndexMut<(usize, usize, usize)> for Image<H, W> {
  fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut u8 {
    return &mut self.data[index.0][index.1][index.2];
  }
}