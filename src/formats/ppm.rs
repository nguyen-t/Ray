use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct PPM<const H: usize, const W: usize> {
  file: File
}

impl<const H: usize, const W: usize> PPM<H, W> {
  pub fn new(filename: &str) -> PPM<H, W> {
    let path = Path::new(filename);
    let mut file = match File::create(path) {
      Err(why) => panic!("CREATE failure: {}", why),
      Ok(f)    => f
    };
    let header = format!("P3\n{} {}\n255\n", W, H);

    match file.write_all(header.as_bytes()) {
      Err(why) => panic!("WRITE failure: {}", why),
      Ok(_)    => ()
    };

    return PPM {
      file: file
    };
  }

  pub fn draw(&mut self, rgb: &[[[u8; 3]; W]; H]) {
    let mut buffer = String::new();

    for h in 0..H {
      for w in 0..W {
        buffer += &rgb[h][w][0].to_string();
        buffer += " ";
        buffer += &rgb[h][w][1].to_string();
        buffer += " ";
        buffer += &rgb[h][w][2].to_string();
        buffer += "\n";
      }
    }

    match self.file.write_all(buffer.as_bytes()) {
      Err(why) => panic!("WRITE failure: {}", why),
      Ok(_)    => ()
    };
  }
}