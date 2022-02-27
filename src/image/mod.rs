//! A module for reading/parsing PNG images
//!
//! Example usage:
//!    let image = PNGImage::from_file("example.png");
//!    let signature = image.signature();
//!    let size  : (u32, u32) = image.size();
//!
#[macro_use]
pub mod sprite;
pub mod png;
