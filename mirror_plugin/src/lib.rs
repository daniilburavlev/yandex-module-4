#![deny(unreachable_pub)]
#![warn(missing_docs)]
//! The `blur_plugin` crate provides ability
//! to blur PNG image by radius and iterations
use std::ffi::{CStr, c_char};

use serde::{Deserialize, Serialize};

const PIXEL_SIZE: usize = 4;

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

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) {
    if rgba_data.is_null() || width == 0 || height == 0 {
        return;
    }
    let params_str = if !params.is_null() {
        unsafe { CStr::from_ptr(params).to_string_lossy().to_string() }
    } else {
        String::new()
    };
    let width = width as usize;
    let height = height as usize;
    let size = width * height * PIXEL_SIZE;
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, size) };

    let params = Params::parse_json(params_str);
    if params.vertical {
        flip_vertical(width, height, data);
    }
    if params.horizontal {
        flip_horizontal(width, height, data);
    }
}

fn flip_horizontal(width: usize, height: usize, rgba: &mut [u8]) {
    for y in 0..height {
        for x in 0..width / 2 {
            let left_idx = (y * width + x) * PIXEL_SIZE;
            let right_idx = (y * width + (width - 1 - x)) * PIXEL_SIZE;

            for i in 0..PIXEL_SIZE {
                rgba.swap(left_idx + i, right_idx + i);
            }
        }
    }
}

fn flip_vertical(width: usize, height: usize, rgba: &mut [u8]) {
    let row_size = width * PIXEL_SIZE;

    for y in 0..height / 2 {
        let top_start = y * row_size;
        let bottom_start = (height - 1 - y) * row_size;

        let (top_row, rest) = rgba.split_at_mut(bottom_start);
        let (bottom_row, _) = rest.split_at_mut(row_size);
        let top_row_slice = &mut top_row[top_start..top_start + row_size];

        top_row_slice.swap_with_slice(&mut bottom_row[..row_size]);
    }
}
