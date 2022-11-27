use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct YUV4MPEG2<const H: usize, const W: usize> {
  file: File
}

impl<const H: usize, const W: usize> YUV4MPEG2<H, W> {
  pub fn new(filename: &String, fps: u32) -> YUV4MPEG2<H, W> {
    let path = Path::new(filename);
    let mut file = match File::create(path) {
      Err(why) => panic!("CREATE failure: {}", why),
      Ok(f)    => f
    };
    let header = format!("YUV4MPEG2 W{} H{} F{}:1 Ip A1:1 C444\n", W, H, fps);

    match file.write_all(header.as_bytes()) {
      Err(why) => panic!("WRITE failure: {}", why),
      Ok(_)    => ()
    };

    return YUV4MPEG2 {
      file: file
    };
  }

  pub fn draw(&mut self, ycbcr: &[[[u8; 3]; W]; H]) {
    let mut buffer  = String::new();
    let mut plane_y = String::new();
    let mut plane_cb = String::new();
    let mut plane_cr = String::new();

    for h in 0..H { 
      for w in 0..W {
        plane_y += &ycbcr[h][w][0].to_string();
        plane_cb += &ycbcr[h][w][1].to_string();
        plane_cr += &ycbcr[h][w][2].to_string();
      }
    }

    buffer += "FRAME\n";
    buffer += &plane_y;
    buffer += &plane_cb;
    buffer += &plane_cr;

    match self.file.write_all(buffer.as_bytes()) {
      Err(why) => panic!("WRITE failure: {}", why),
      Ok(_)    => ()
    };
  }
}