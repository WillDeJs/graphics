use crate::grfx::image::png::PNGError;
pub fn inflate_idat(idat: &Vec<u8>) -> Result<Vec<u8>, PNGError> {
    let decoded = inflate::inflate_bytes_zlib(&idat[..]);

    if decoded.is_err() {
        return Err(PNGError::ParssingError(decoded.unwrap_err()));
    }
    return Ok(decoded.unwrap().iter().map(|e| *e).collect::<Vec<u8>>());
}
