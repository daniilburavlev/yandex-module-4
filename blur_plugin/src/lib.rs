#![deny(unreachable_pub)]
#![warn(missing_docs)]
//! The `mirror_plugin` crate provides ability
//! to flip PNG image vertically and horizontally
use std::{ffi::CStr, os::raw::c_char};

use serde::Deserialize;

const PIXEL_SIZE: i32 = 4;

#[derive(Deserialize)]
struct Params {
    pub radius: u32,
    pub iterations: usize,
}

impl Params {
    fn parse_json(json: String) -> Self {
        serde_json::from_str(&json).expect("cannot parse params")
    }
}

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) {
    if rgba_data.is_null() || params.is_null() {
        return;
    }
    let params_str = if !params.is_null() {
        unsafe { CStr::from_ptr(params).to_string_lossy().to_string() }
    } else {
        String::new()
    };
    let params = Params::parse_json(params_str);
    let radius = params.radius as i32;

    let width = width as i32;
    let height = height as i32;

    let size = (width * height * PIXEL_SIZE) as usize;

    for _ in 0..params.iterations {
        let mut temp_buffer = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(rgba_data, temp_buffer.as_mut_ptr(), size);
        }
        let dst = unsafe { std::slice::from_raw_parts_mut(rgba_data, size) };

        box_blur(width, height, &temp_buffer, dst, radius);
    }
}

fn box_blur(width: i32, height: i32, temp_buffer: &[u8], dst: &mut [u8], r: i32) {
    for y in 0..height {
        for x in 0..width {
            blur_pixel(x, y, width, height, temp_buffer, dst, r);
        }
    }
}

fn blur_pixel(x: i32, y: i32, width: i32, height: i32, src: &[u8], dst: &mut [u8], radius: i32) {
    let mut sum_r = 0u32;
    let mut sum_g = 0u32;
    let mut sum_b = 0u32;
    let mut sum_a = 0u32;
    let mut count = 0u32;

    for dy in -radius..=radius {
        let ny = y + dy;
        if ny < 0 || ny >= height {
            continue;
        }

        for dx in -radius..=radius {
            let nx = x + dx;
            if nx < 0 || nx >= width {
                continue;
            }

            let idx = ((ny * width + nx) * PIXEL_SIZE) as usize;

            sum_r += src[idx] as u32;
            sum_g += src[idx + 1] as u32;
            sum_b += src[idx + 2] as u32;
            sum_a += src[idx + 3] as u32;
            count += 1;
        }
    }

    if count > 0 {
        let idx = ((y * width + x) * PIXEL_SIZE) as usize;
        dst[idx] = (sum_r / count) as u8;
        dst[idx + 1] = (sum_g / count) as u8;
        dst[idx + 2] = (sum_b / count) as u8;
        dst[idx + 3] = (sum_a / count) as u8;
    }
}
