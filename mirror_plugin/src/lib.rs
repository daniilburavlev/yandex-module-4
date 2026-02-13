#![deny(unreachable_pub)]
#![warn(missing_docs)]
//! The `blur_plugin` crate provides ability
//! to blur PNG image by radius and iterations
use std::ffi::{CStr, c_char};

use serde::{Deserialize, Serialize};

const PIXEL_SIZE: u32 = 4;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Params {
    pub vertical: bool,
    pub horizontal: bool,
}

impl Params {
    fn parse_json(json: String) -> Self {
        serde_json::from_str(&json).expect("cannot read params")
    }
}

const OVERFLOW_ERROR: &str = "WIDTH x HEIGHT overflow";

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) {
    if rgba_data.is_null() || width == 0 || height == 0 {
        eprintln!("Empty image input");
        return;
    }
    let params_str = if !params.is_null() {
        unsafe { CStr::from_ptr(params).to_string_lossy().to_string() }
    } else {
        String::new()
    };
    let Some(size) = get_size(width, height) else {
        eprintln!("{}", OVERFLOW_ERROR);
        return;
    };
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, size as usize) };

    let params = Params::parse_json(params_str);
    if params.vertical {
        flip_vertical(width, height, data).expect(OVERFLOW_ERROR);
    }
    if params.horizontal {
        flip_horizontal(width, height, data).expect(OVERFLOW_ERROR);
    }
}

fn flip_horizontal(width: u32, height: u32, rgba: &mut [u8]) -> Option<()> {
    for y in 0..height {
        for x in 0..width / 2 {
            let left_idx = (y.checked_mul(width)?.checked_add(x)?).checked_mul(PIXEL_SIZE)?;
            let right_idx = (y * width + (width - 1 - x)) * PIXEL_SIZE;

            for i in 0..PIXEL_SIZE {
                rgba.swap(
                    left_idx.checked_add(i)? as usize,
                    right_idx.checked_add(i)? as usize,
                );
            }
        }
    }
    Some(())
}

fn flip_vertical(width: u32, height: u32, rgba: &mut [u8]) -> Option<()> {
    let row_size = width.checked_mul(PIXEL_SIZE)?;

    for y in 0..height / 2 {
        let top_start = y.checked_mul(row_size)?;
        let bottom_start = height
            .checked_sub(1)?
            .checked_sub(y)?
            .checked_mul(row_size)?;

        let (top_row, rest) = rgba.split_at_mut(bottom_start as usize);
        let (bottom_row, _) = rest.split_at_mut(row_size as usize);
        let top_row_slice =
            &mut top_row[top_start as usize..top_start as usize + row_size as usize];

        top_row_slice.swap_with_slice(&mut bottom_row[..row_size as usize]);
    }
    Some(())
}

fn get_size(width: u32, height: u32) -> Option<u32> {
    width.checked_mul(height)?.checked_mul(PIXEL_SIZE)
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn check_overflow() {
        let width = u32::MAX;
        let height = u32::MAX;

        let mut buffer: Vec<u8> = Vec::new();
        process_image(width, height, buffer.as_mut_ptr(), std::ptr::null());
    }

    #[test]
    fn horizontal() {
        let height = 5;
        let width = 1;
        let params = Params {
            vertical: true,
            horizontal: true,
        };
        let params = serde_json::to_string(&params).unwrap();
        let size = (width * height * PIXEL_SIZE) as usize;
        let mut input = vec![0u8; size];
        let mut output = vec![0u8; size];
        let mut pixel = 1;
        for i in 0..size {
            input[i] = pixel;
            output[size - i - 1] = pixel;
            if (i + 1) % 4 == 0 {
                pixel += 1;
            }
        }
        let params_cstr = CString::new(params.as_str()).unwrap();

        process_image(width, height, input.as_mut_ptr(), params_cstr.as_ptr());
        assert_eq!(input, output);
    }

    #[test]
    fn vertical() {
        let width = 5;
        let height = 1;
        let params = Params {
            vertical: false,
            horizontal: true,
        };
        let params = serde_json::to_string(&params).unwrap();
        let size = (width * height * PIXEL_SIZE) as usize;
        let mut input = vec![0u8; size];
        let mut output = vec![0u8; size];
        let mut pixel = 1;
        for i in 0..size {
            input[i] = pixel;
            output[size - i - 1] = pixel;
            if (i + 1) % 4 == 0 {
                pixel += 1;
            }
        }
        let params_cstr = CString::new(params.as_str()).unwrap();

        process_image(width, height, input.as_mut_ptr(), params_cstr.as_ptr());
        assert_eq!(input, output);
    }
}
