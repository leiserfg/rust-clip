use clip_sys::*;

pub struct ClipImage {
  ptr: CClipImage,
}

impl ClipImage {
  pub fn from_ptr(ptr: CClipImage) -> Self {
    Self { ptr }
  }

  pub fn get_spec(&self) -> CClipImageSpec {
    unsafe { clip_get_image_spec(self.ptr) }
  }

  pub fn get_data(&self) {
    let _data = unsafe { clip_get_image_data(self.ptr) };

    unimplemented!()
  }
}

impl Drop for ClipImage {
  fn drop(&mut self) {
    unsafe {
      clip_delete_image(self.ptr);
    }
  }
}
