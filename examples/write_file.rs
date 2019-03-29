use clip::{Clip, ClipFormat};
use std::fs::File;
use std::io::Write;

fn main() {
  match Clip::get_format().expect("other unimplemented format") {
    ClipFormat::Empty => {
      println!("clipboard is empty!");
    }

    ClipFormat::Text => {
      {
        let mut file = File::create("test.txt").unwrap();
        file
          .write_all(Clip::get_text().unwrap().as_bytes())
          .unwrap();
      }
      println!("wrote to test.txt");
    }

    ClipFormat::Image => {
      {
        let image = Clip::get_image().unwrap();
        image
          .write_png(&mut File::create("test.png").unwrap())
          .unwrap();
      }

      println!("wrote to test.png");
    }
  }
}
