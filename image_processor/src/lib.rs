#![deny(unreachable_pub)]
#![warn(missing_docs)]
//! The `image_processor` crate
use std::path::Path;

use image::{ImageReader, RgbaImage};

use crate::{error::ProcessorError, plugin_loader::apply_plugin};

pub mod error;
pub mod plugin_loader;

fn read_img(path: &Path) -> Result<RgbaImage, ProcessorError> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img.into_rgba8())
}

fn write_img(img: RgbaImage, path: &Path) -> Result<(), ProcessorError> {
    img.save(path)?;
    Ok(())
}

/// Process PNG image
/// # Arguments
///
/// * `input` - Input image path.
/// * `output` - Output path.
/// * `plugin` - Plugin path.
/// * `params` - Parameters.
///
pub fn process_image(
    input: &Path,
    output: &Path,
    plugin: &Path,
    params: String,
) -> Result<(), ProcessorError> {
    let mut img = read_img(input)?;
    apply_plugin(&mut img, plugin, params)?;
    write_img(img, output)?;
    Ok(())
}
