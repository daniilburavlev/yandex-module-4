//! Run external plugin
use std::{
    ffi::{CString, c_char},
    path::Path,
};

use image::RgbaImage;
use libloading::{Library, Symbol};

use crate::error::ProcessorError;

/// Main image processing function
/// # Safety
/// * `width` - Image width in pixels (can overflow buffer with values width * height * 4 > i32::MAX)
/// * `height` - Image height in pixels (can overflow buffer with values width * height * 4 > i32::MAX)
/// * `rgba_data` - pointing to valid, not null RGBA8 buffer(len = width * height * 4 pixels).
/// * `params` - Parameter string - valid, not null, not empty string, terminated with '\0'.
///
/// # Returns `i32`:
/// * `0` - successfull
/// * `-1` - error
pub type ProcessImagePlugin =
    unsafe extern "C" fn(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) -> i32;

pub(crate) fn apply_plugin(
    img: &mut RgbaImage,
    plugin: &Path,
    params: String,
) -> Result<(), ProcessorError> {
    let plugin = Plugin::new(plugin)?;
    let plugin = plugin.interface()?;
    let width = img.width();
    let height = img.height();
    let params_cstr = CString::new(params.as_str())
        .map_err(|_| ProcessorError::FFI("cannot convert params to c_str".to_string()))?;
    unsafe {
        let result = (plugin.process_image)(width, height, img.as_mut_ptr(), params_cstr.as_ptr());
        if result < 0 {
            return Err(ProcessorError::FFI("error processing image".to_string()));
        }
    }
    Ok(())
}

struct ImgProcessPlugin<'a> {
    pub process_image: Symbol<'a, ProcessImagePlugin>,
}

struct Plugin {
    plugin: Library,
}

impl Plugin {
    fn new(filepath: &Path) -> Result<Self, ProcessorError> {
        Ok(Plugin {
            plugin: unsafe { Library::new(filepath) }?,
        })
    }

    fn interface(&self) -> Result<ImgProcessPlugin<'_>, ProcessorError> {
        Ok(ImgProcessPlugin {
            process_image: unsafe { self.plugin.get("process_image") }?,
        })
    }
}
