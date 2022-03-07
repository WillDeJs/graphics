use crate::image::png::PNGError;
use miniz_oxide::deflate;
use miniz_oxide::inflate;

pub fn decompress_zlib(idat: &[u8]) -> Result<Vec<u8>, PNGError> {
    let decompressed = inflate::decompress_to_vec_zlib(idat)
        .map_err(|_| PNGError::ParssingError("Error decompressing image data".into()))?;

    Ok(decompressed)
}

pub fn compress_zlib(idat: &[u8]) -> Vec<u8> {
    deflate::compress_to_vec_zlib(idat, 0)
}
