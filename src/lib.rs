use clip_sys::*;
use failure::{err_msg, Error};

pub struct Clip;

impl Clip {
  pub fn set_text(text: String) -> Result<(), Error> {
    use std::ffi::CString;

    let c_string = CString::new(text)?;

    let did_set = unsafe {
      let raw = c_string.into_raw();

      let did_set = clip_set_text(raw);

      CString::from_raw(raw);

      did_set
    };

    if !did_set {
      Err(err_msg("couldn't set clipboard text"))
    } else {
      Ok(())
    }
  }

  pub fn get_text() -> Result<String, Error> {
    use std::ffi::CStr;

    unsafe {
      let c_str = clip_get_text();
      if c_str.is_null() {
        Err(err_msg("couldn't get clipboard text"))
      } else {
        let string = CStr::from_ptr(c_str)
          .to_str()
          .map(std::string::ToString::to_string)
          .map_err(std::convert::Into::into);

        clip_delete_text(c_str);

        string
      }
    }
  }
}

#[test]
fn test_text() {
  let s = "helloh".to_string();
  Clip::set_text(s.clone()).unwrap();

  assert_eq!(Clip::get_text().unwrap(), s);
}
