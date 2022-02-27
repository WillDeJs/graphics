use crate::image::png::PNGError;
use miniz_oxide::deflate;
use miniz_oxide::inflate;

pub fn inflate_idat(idat: &Vec<u8>) -> Result<Vec<u8>, PNGError> {
    let decompressed = inflate::decompress_to_vec_zlib(idat).or(Err(PNGError::ParssingError(
        "Error decompressing image data".into(),
    )))?;

    Ok(decompressed)
}

pub fn deflate_idat(idat: &Vec<u8>) -> Vec<u8> {
    deflate::compress_to_vec_zlib(idat, 0)
}
