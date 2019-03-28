use clip_sys::*;
use image::ColorType;
use std::io::Write;

pub struct ClipImage {
  ptr: CClipImage,
}

impl<'a> ClipImage {
  pub fn from_ptr(ptr: CClipImage) -> Self {
    Self { ptr }
  }

  pub fn get_spec(&self) -> CClipImageSpec {
    unsafe { clip_get_image_spec(self.ptr) }
  }

  pub fn get_data(&'a self) -> &'a [u8] {
    let spec = self.get_spec();
    let len: usize = spec.bytes_per_row as usize * spec.height as usize;
    unsafe { std::slice::from_raw_parts(clip_get_image_data(self.ptr) as *const u8, len) }
  }

  pub fn write_as_png<W: Write>(&self, writer: &mut W) {
    let encoder = image::png::PNGEncoder::new(writer);

    let clip_spec = self.get_spec();
    let clip_data = self.get_data();

    encoder
      .encode(
        clip_data,
        clip_spec.width as u32,
        clip_spec.height as u32,
        ColorType::RGBA(8), // TODO check spec.bits_per_pixel
      )
      .unwrap();
  }

  pub fn write_as_jpeg<W: Write>(&self, writer: &mut W) {
    let mut encoder = image::jpeg::JPEGEncoder::new(writer);

    let clip_spec = self.get_spec();
    let clip_data = self.get_data();

    encoder
      .encode(
        clip_data,
        clip_spec.width as u32,
        clip_spec.height as u32,
        ColorType::RGBA(8), // TODO check spec.bits_per_pixel
      )
      .unwrap();
  }
}

impl Drop for ClipImage {
  fn drop(&mut self) {
    unsafe {
      clip_delete_image(self.ptr);
    }
  }
}

#[test]
fn test_write_image_as_png() {
  use super::Clip;
  use std::fs::File;

  let image = Clip::get_image().unwrap();
  image.write_as_png(&mut File::create("test.png").unwrap());
}

#[test]
fn test_write_image_as_jpeg() {
  use super::Clip;
  use std::fs::File;

  let image = Clip::get_image().unwrap();
  image.write_as_jpeg(&mut File::create("test.jpeg").unwrap());
}
