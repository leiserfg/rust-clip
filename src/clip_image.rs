use clip_sys::*;
use image::bmp::BMPEncoder;
use image::jpeg::JPEGEncoder;
use image::png::PNGEncoder;
use image::ColorType;
use std::io;
use std::io::Write;

pub struct ClipImage {
  ptr: CClipImage,
}

pub trait Encoder {
  fn encode(self, data: &[u8], width: u32, height: u32, color: ColorType) -> io::Result<()>;
}

impl<W: Write> Encoder for PNGEncoder<W> {
  fn encode(self, data: &[u8], width: u32, height: u32, color: ColorType) -> io::Result<()> {
    PNGEncoder::encode(self, data, width, height, color)
  }
}

impl<'a, W: 'a> Encoder for JPEGEncoder<'a, W>
where
  W: Write,
{
  fn encode(mut self, data: &[u8], width: u32, height: u32, color: ColorType) -> io::Result<()> {
    JPEGEncoder::encode(&mut self, data, width, height, color)
  }
}

impl<'a, W: 'a> Encoder for BMPEncoder<'a, W>
where
  W: Write,
{
  fn encode(mut self, data: &[u8], width: u32, height: u32, color: ColorType) -> io::Result<()> {
    BMPEncoder::encode(&mut self, data, width, height, color)
  }
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
    self.write_from_encoder(PNGEncoder::new(writer));
  }

  pub fn write_as_jpeg<W: Write>(&self, writer: &mut W) {
    self.write_from_encoder(JPEGEncoder::new(writer));
  }

  pub fn write_as_bmp<W: Write>(&self, writer: &mut W) {
    self.write_from_encoder(BMPEncoder::new(writer));
  }

  pub fn write_from_encoder<E: Encoder>(&self, encoder: E) {
    let clip_spec = self.get_spec();
    let clip_data = self.get_data();

    println!("{:#?}", clip_spec);

    assert!(clip_spec.bits_per_pixel == 32);

    assert!(clip_spec.red_shift == 0);
    assert!(clip_spec.green_shift == 8);
    assert!(clip_spec.blue_shift == 16);
    assert!(clip_spec.alpha_shift == 24);

    assert!(clip_spec.red_mask == 0b1111_1111);
    assert!(clip_spec.green_mask == 0b1111_1111 << 8);
    assert!(clip_spec.blue_mask == 0b1111_1111 << 16);
    assert!(clip_spec.alpha_mask == 0b1111_1111 << 24);

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
