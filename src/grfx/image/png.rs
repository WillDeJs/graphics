use crate::grfx::color::{self, Color};
use crate::utils::gz;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;

///
///  PNG signature contains the signature of the file
/// total 5 bytes.
///
/// Source: https://www.w3.org/TR/REC-png-961001
///
#[derive(Default, Debug, Copy, Clone)]
pub struct Signature {
    signature: [u8; 8],
}

/// size of signature
const SZ_SIGNATURE: usize = std::mem::size_of::<Signature>();

/// Size of length field on each chunk
const SZ_CHUNK_LENGTH: usize = 4;
const SZ_CHUNK_CRC: usize = 4;
const SZ_CHUNK_COLOR_TYPE: usize = 4;

/// Each PNG file is expected to start with this valid signature
const VALID_SIGNATURE: [u8; SZ_SIGNATURE] = [137, 80, 78, 71, 13, 10, 26, 10];

#[doc(hide)]
// Adam 7 interlacing table
#[doc(hide)]
#[allow(dead_code)]
const INTERLACING_SCAN_TABLE: [[u8; 7]; 6] = [
    [0, 0, 4, 0, 2, 0, 1], /* STARTING ROW*/
    [0, 4, 0, 2, 0, 1, 0], /* STARTING COLUM */
    [8, 8, 8, 4, 4, 2, 2], /* ROW INCREMENT*/
    [8, 8, 4, 4, 2, 2, 1], /* COLUMN INCREMENT */
    [8, 8, 4, 4, 2, 2, 1], /* BLOCK HEIGHT */
    [8, 4, 4, 2, 2, 1, 1], /*LOCK WIDTH */
];

#[doc(hide)]
///  Interlacing table indexes
/// INTERLACING NOT IMPLMENTED NOT YET IMPLEMENTED
#[allow(dead_code)]
const START_ROW: usize = 0;
#[allow(dead_code)]
const START_COLUMN: usize = 1;
#[allow(dead_code)]
const ROW_INCREMENT: usize = 2;
#[allow(dead_code)]
const COLUMN_INCREMENT: usize = 3;
#[allow(dead_code)]
const BLOCK_HEIGHT: usize = 4;
#[allow(dead_code)]
const WIDTH_HEIGHT: usize = 5;

/// CRC table initalization
static mut CRC_TABLE: [u32; 256] = [0; 256];
static mut CRC_INITIALIZED: bool = false;

const GRAY_SCALE_CTYPE: u8 = 0;
const RGB_CTYPE: u8 = 2;
const PALETTE_INDEX_CTYPE: u8 = 3;
const GREY_SCALE_ALPHA_CTYPE: u8 = 4;
const RGB_ALPHA_CTYPE: u8 = 6;

///
/// A PNG image contains a signature followed by
/// a set of chunks, each chunk contains:
///     a) Length integer
///     b) chunk type identifier integer
///     b) data bytes of length (length)
///     c) CRC a 4-byte cyclic redundancy check
#[derive(Default, Debug, Clone)]
pub struct Chunk {
    length: u32,
    c_type: [u8; 4],
    data: Vec<u8>,
    crc: [u8; 4],
}

impl Chunk {
    fn crc_okay(&self) -> bool {
        // data + 4 bytes for CHUNK TYPE
        let mut chunk_data = Vec::<u8>::with_capacity(self.data.len() + SZ_CHUNK_LENGTH);
        chunk_data.extend_from_slice(&self.c_type);
        chunk_data.extend_from_slice(&self.data[..]);
        crc(&chunk_data) == self.crc
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type: {}, Size {}, data: ... CRC: {:?}",
            std::str::from_utf8(&self.c_type).unwrap(),
            self.length,
            self.crc
        )
    }
}

/// IHDR chunk is always the first chunk in the image
///  It contains information such as width and height and compression
#[derive(Default, Debug, Copy, Clone)]
pub struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression: u8,
    filter: u8,
    interlace: u8,
}
/// Size of a IHDR struct
const SZ_IHDR: usize = 13;

/// Default PNG header types
pub const IHDR_TYPE: &[u8; 4] = b"IHDR";
pub const PLTE_TYPE: &[u8; 4] = b"PLTE";
pub const IDAT_TYPE: &[u8; 4] = b"IDAT";
pub const IEND_TYPE: &[u8; 4] = b"IEND";

#[allow(non_upper_case_globals)]
pub const gAMA_TYPE: &[u8; 4] = b"gAMA";

///
/// The PLTE chunk contains from 1 to 256 palette entries, each a three-byte series of the form:
///   Red:   1 byte (0 = black, 255 = red)
///   Green: 1 byte (0 = black, 255 = green)
///  Blue:  1 byte (0 = black, 255 = blue)
/// The number of entries is determined from the chunk length. A chunk length not divisible by 3 is an error.
#[derive(Default, Debug, Clone)]
pub struct PLTE {
    colors: Vec<Color>,
}

impl TryFrom<Chunk> for PLTE {
    type Error = PNGError;
    fn try_from(chunk: Chunk) -> Result<PLTE, Self::Error> {
        let colors = chunk.data;
        if colors.len() % 3 != 0 {
            // legnth must be divisible by 3
            Err(PNGError::ParssingError(
                "Invalid length for PLTE chunk".into(),
            ))
        } else {
            let mut pixels = Vec::<Color>::with_capacity(colors.len() / 3);
            for color_chunk in colors[..].chunks_exact(3) {
                let color = Color::from_slice(color_chunk);
                pixels.push(color);
            }
            // Just in case some images have invalid indexes
            // lets avoid crashes by making sure all allowed indexes are covered.
            for _ in 0..256 - pixels.len() {
                pixels.push(color::BLACK);
            }
            Ok(Self { colors: pixels })
        }
    }
}

/// The IDAT (IMAGE DATA) chunck contains the actual image data.
#[derive(Default, Debug, Clone)]
struct IDAT {
    data: Vec<u8>,
}
#[derive(Default, Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct gAMA {
    gamma: u32,
}
/// Tje last cunk that must appaer LAST and does not contain any data.
#[derive(Default, Debug, Clone)]
pub struct IEND {}

/// A PNG Image struct that contains a signature and a list of chunks
#[derive(Default, Debug, Clone)]
pub struct PNGImage {
    signature: Signature,
    header: IHDR,
    plte: Option<PLTE>,
    chunks: Vec<Chunk>,
}

/// Different allowed filter type
#[derive(Debug, Copy, Clone)]
pub enum FilterType {
    None = 0,
    Sub = 1,
    Up = 2,
    Average = 3,
    Paeth = 4,
    Unsupported,
}

impl From<u8> for FilterType {
    fn from(val: u8) -> FilterType {
        match val {
            0 => FilterType::None,
            1 => FilterType::Sub,
            2 => FilterType::Up,
            3 => FilterType::Average,
            4 => FilterType::Paeth,
            _ => FilterType::Unsupported,
        }
    }
}
/// Generic Error type for errors related parsing images
#[derive(Debug)]
pub enum PNGError {
    FileError(String),
    DataError(String),
    ParssingError(String),
}

impl Error for PNGError {}
impl fmt::Display for PNGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PNGError::ParssingError(_) => {
                write!(f, "PNG Error: Could not parse image.")
            }
            PNGError::DataError(_) => {
                write!(f, "Invalid data length extraction.")
            }
            _ => write!(f, "PNG Error: Error reading file."),
        }
    }
}

/******************************************************
 *          Decoding Chunk
 ******************************************************/

/// Chunk decoder to ease decoding a single chunk at the time using iterators
#[derive(Debug, Clone)]
struct ChunkDecoder {
    start: usize,
    data: Vec<u8>,
}

impl ChunkDecoder {
    /// Initialize the chunk decoder with a vector of bytes (usually from a file)
    pub fn from_data(data: Vec<u8>) -> Self {
        Self { start: 0, data }
    }

    /// extracts a given number of bytes from the decoder
    /// Returns them as an optional
    pub fn extract_bytes(&mut self, length: usize) -> Option<&[u8]> {
        let mut value = None;
        if length + self.start <= self.data.len() {
            value = Some(&self.data[self.start..self.start + length]);
            self.start += length;
        }
        return value;
    }
}

/// Allow calling an interator on the ChunkDecodor to get each chunk
impl Iterator for ChunkDecoder {
    type Item = Chunk;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.data.len() {
            let length_bytes = self.extract_bytes(SZ_CHUNK_LENGTH)?;
            let length = be_bytes_to_u32(length_bytes.try_into().ok()?);
            let c_type: [u8; SZ_CHUNK_COLOR_TYPE] =
                self.extract_bytes(SZ_CHUNK_COLOR_TYPE)?.try_into().ok()?;
            let data: Vec<u8> = self.extract_bytes(length as usize)?.try_into().ok()?;
            let crc: [u8; SZ_CHUNK_CRC] = self.extract_bytes(SZ_CHUNK_CRC)?.try_into().ok()?;
            Some(Chunk {
                length,
                c_type,
                data,
                crc,
            })
        } else {
            None
        }
    }
}

/******************************************************
 *          Decoding rows
 ******************************************************/

/// Chunk decoder to ease decoding a single chunk at the time using iterators
#[derive(Debug, Clone)]
#[allow(unused_variables, dead_code)]
struct RowDecoder {
    row_len: usize,
    start: usize,
    data: Vec<u8>,
    bpp: usize,
    previous_row: Vec<u8>,
}

impl RowDecoder {
    /// Initialize the filter  decoder with a vector of bytes (usually from a file)
    pub fn new(data: Vec<u8>, row_len: usize, bpp: usize) -> Self {
        Self {
            row_len,
            start: 0,
            data,
            bpp, // bytes per pixel
            previous_row: vec![0; row_len],
        }
    }

    /// extracts a given number of bytes from the decoder
    /// Returns them as an optional
    pub fn extract_bytes(&mut self, length: usize) -> Option<&[u8]> {
        let mut value = None;
        if length + self.start <= self.data.len() {
            value = Some(&self.data[self.start..self.start + length]);
            self.start += length;
        }
        return value;
    }
}
/// Allow calling an interator on the ChunkDecodor to get each chunk
impl Iterator for RowDecoder {
    type Item = Result<Vec<u8>, PNGError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.data.len() {
            let row_length = self.row_len;
            let filter_type = self.extract_bytes(1)?[0];
            let mut current_row = self.extract_bytes(row_length)?.iter().map(|e| *e).collect();
            let previous_row = self.previous_row.clone();
            let error = remove_filter(
                &mut current_row,
                &previous_row,
                FilterType::from(filter_type),
                self.bpp,
            );

            if error.is_err() {
                return Some(Err(error.unwrap_err()));
            }
            self.previous_row = current_row.clone();
            Some(Ok(current_row))
        } else {
            None
        }
    }
}

/******************************************************
 *          Decoding PNG Image itself
 ******************************************************/

/// Implementation of PNGImage functionalities
/// Contains the methods for interfacing with a PNG image.
///
/// Example usage:
///    let image = PNGImage::from_file("example.png");
///    let signature = image.signature();
///    let size  : (u32, u32) = image.size();
///
///  To display an image take a look at renders, View2D struct;
///
impl PNGImage {
    ///
    /// Read a PNG file into a PNGImage structure
    /// A Valid PNG file is expected
    /// If the IEND chunk is not at the end of this image an error is thrown
    ///
    pub fn from_file(file: &str) -> Result<Self, Box<dyn Error>> {
        let mut image_file = File::open(file)?;
        let mut data = Vec::<u8>::new();
        let mut signature = [0 as u8; SZ_SIGNATURE];
        let critical_chunks = vec![IHDR_TYPE, IDAT_TYPE, IEND_TYPE];
        let mut message = String::from("Unknown error encountered");
        let mut error_found = false;
        let mut header_found = false;
        let mut idat_found = false;
        let mut iend_found = false;
        let mut has_plte = false;
        let mut plte: Option<PLTE> = None;
        let mut header = IHDR::default();

        image_file.read_exact(&mut signature)?;

        if signature != VALID_SIGNATURE {
            return Err(Box::new(PNGError::ParssingError(
                "Invalid PNG signature encountered".into(),
            )));
        }
        let signature = Signature { signature };
        let _ = image_file.read_to_end(&mut data)?;

        let decoder = ChunkDecoder::from_data(data);

        // Ignore chunks that don't have valid CRC
        // If critical chunk CRC fails, error
        let chunks: Vec<Chunk> = decoder
            .filter(|chunk| {
                if chunk.c_type == *IDAT_TYPE {
                    idat_found = true;
                }
                if chunk.c_type == *PLTE_TYPE {
                    has_plte = true;
                    let temp = PLTE::try_from(chunk.clone());
                    if temp.is_err() {
                        error_found = true;
                        message = "Error parsing PLTE chunk".to_string();
                    } else {
                        plte = Some(temp.unwrap());
                    }
                } else if chunk.c_type == *IHDR_TYPE {
                    let temp = parse_ihdr_data(&chunk.data);
                    if temp.is_err() {
                        message = String::from("Invalid IHDR data");
                    } else {
                        header_found = true;
                        header = temp.unwrap();
                    }
                } else if chunk.c_type == *IEND_TYPE {
                    iend_found = true;
                }
                if critical_chunks.contains(&&chunk.c_type) {
                    if !chunk.crc_okay() {
                        message = format!("Invalid CRC check for CHUNK: '{:?}'.", chunk.c_type);
                        error_found = true;
                        false
                    } else {
                        true
                    }
                } else {
                    chunk.crc_okay()
                }
            })
            .collect();

        if error_found || !header_found || !idat_found || !iend_found {
            return Err(Box::new(PNGError::ParssingError(message)));
        }

        if !valid_bit_depth(header.color_type, header.bit_depth) {
            message = format!(
                "Invalid color type bit depth combination: c: {}, bd: {}",
                header.color_type, header.bit_depth
            );
            return Err(Box::new(PNGError::ParssingError(message)));
        }
        Ok(Self {
            header,
            plte,
            signature,
            chunks,
        })
    }

    pub fn header(&self) -> IHDR {
        self.header
    }
    pub fn width(&self) -> u32 {
        self.header.width
    }
    pub fn height(&self) -> u32 {
        self.header.height
    }
    pub fn bytes_per_pixel(&self) -> usize {
        let channels = match self.header.color_type {
            GRAY_SCALE_CTYPE => 1,
            RGB_CTYPE => 3,
            PALETTE_INDEX_CTYPE => 1,
            GREY_SCALE_ALPHA_CTYPE => 2,
            RGB_ALPHA_CTYPE => 4,
            _ => 1,
        };
        let bits = self.header.bit_depth * channels;
        return (bits as f32 / 8.0).ceil() as usize;
    }
    pub fn row_length(&self) -> usize {
        let channels: usize = match self.header.color_type {
            GRAY_SCALE_CTYPE => 1,
            RGB_CTYPE => 3,
            PALETTE_INDEX_CTYPE => 1,
            GREY_SCALE_ALPHA_CTYPE => 2,
            RGB_ALPHA_CTYPE => 4,
            _ => 1,
        };

        return channels * (self.header.bit_depth as usize) * (self.width() as usize) / 8;
    }

    /// Collect all the image data on the image and return it as a vector
    pub fn image_data(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut idat_vec = Vec::<u8>::new();
        self.chunks
            .iter()
            .filter(|chunk| chunk.c_type == *IDAT_TYPE)
            .for_each(|chunk| {
                idat_vec.extend_from_slice(&chunk.data[..]);
            });

        let decompressed: Vec<u8> = gz::inflate_idat(&idat_vec)?;
        println!("Decompressed: {}", decompressed.len());
        let bpp = self.bytes_per_pixel();
        let row_len = self.row_length();

        let mut unfiltered = Vec::<u8>::with_capacity(row_len * self.height() as usize);
        let filter_decoder = RowDecoder::new(decompressed, row_len, bpp);
        for row in filter_decoder {
            unfiltered.extend_from_slice(&row?[..]);
        }

        Ok(unfiltered)
    }

    pub fn image_pixels(&self) -> Result<Vec<Color>, Box<dyn Error>> {
        let mut pixels = Vec::<Color>::with_capacity((self.width() * self.height()) as usize);
        let image_data = self.image_data()?;
        match self.header.color_type {
            GRAY_SCALE_CTYPE => match self.header.bit_depth {
                1 => {
                    for byte in image_data {
                        let mut value = byte;
                        for _ in 0..8 {
                            let scaled = (value >> 7) * 0xff;
                            value <<= 1;
                            pixels.push(Color::rgb(scaled, scaled, scaled));
                        }
                    }
                }

                // This was a bit weird to figure out
                // thankfully go has a sample
                //https://golang.org/src/image/png/reader.go
                2 => {
                    for byte in image_data {
                        let mut value = byte;
                        for _ in 0..4 {
                            let scaled = (value >> 6) * 0x55;
                            value <<= 2;
                            pixels.push(Color::rgb(scaled, scaled, scaled));
                        }
                    }
                }
                4 => {
                    for byte in image_data {
                        let mut value = byte;
                        for _ in 0..2 {
                            let scaled = (value >> 4) * 0x11;
                            value <<= 4;
                            pixels.push(Color::rgb(scaled, scaled, scaled));
                        }
                    }
                }
                8 => {
                    for byte in image_data {
                        pixels.push(Color::rgb(byte, byte, byte));
                    }
                }
                16 => {}
                _ => (),
            },
            RGB_CTYPE => {
                match self.header.bit_depth {
                    8 => {
                        for chunk in image_data[..].chunks_exact(3) {
                            let color = Color::from_slice(&chunk[..]);
                            pixels.push(color)
                        }
                    }
                    16 => {
                        let max = 65535.0;
                        for chunk in image_data[..].chunks_exact(6) {
                            let r = (255.0 * (((chunk[0] as u32) << 8) as f32 + (chunk[1] as f32))
                                / max) as u8;
                            let g = (255.0 * (((chunk[2] as u32) << 8) as f32 + (chunk[3] as f32))
                                / max) as u8;
                            let b = (255.0 * (((chunk[4] as u32) << 8) as f32 + (chunk[5] as f32))
                                / max) as u8;
                            let color = Color::rgb(r, g, b);
                            pixels.push(color)
                        }
                    }
                    _ => (),
                };
            }
            PALETTE_INDEX_CTYPE => {
                match self.header.bit_depth {
                    1 => {
                        if let Some(plte) = &self.plte {
                            for byte in image_data {
                                let mut index = byte;
                                for _ in 0..8 {
                                    let scaled_index = index >> 7;
                                    index <<= 1;
                                    pixels.push(plte.colors[scaled_index as usize]);
                                }
                            }
                        }
                    }
                    2 => {
                        if let Some(plte) = &self.plte {
                            for byte in image_data {
                                let mut index = byte;
                                for _ in 0..4 {
                                    let scaled_index = index >> 6;
                                    index <<= 2;
                                    pixels.push(plte.colors[scaled_index as usize]);
                                }
                            }
                        }
                    }
                    4 => {
                        if let Some(plte) = &self.plte {
                            for byte in image_data {
                                let mut index = byte;
                                for _ in 0..2 {
                                    let scaled_index = index >> 4;
                                    index <<= 4;
                                    pixels.push(plte.colors[scaled_index as usize]);
                                }
                            }
                        }
                    }
                    8 => {
                        if let Some(plte) = &self.plte {
                            for color_index in &image_data[..] {
                                if (*color_index as usize) < plte.colors.len() {
                                    pixels.push(plte.colors[*color_index as usize]);
                                }
                            }
                        }
                    }
                    _ => (),
                };
            }
            GREY_SCALE_ALPHA_CTYPE => {
                match self.header.bit_depth {
                    8 => {
                        for chunk in image_data[..].chunks_exact(2) {
                            let color = Color::rgba(chunk[0], chunk[0], chunk[0], chunk[1]);
                            pixels.push(color)
                        }
                    }
                    16 => {
                        let max = 65535.0;
                        for chunk in image_data[..].chunks_exact(4) {
                            let col = (255.0
                                * (((chunk[0] as u32) << 8) as f32 + (chunk[1] as f32))
                                / max) as u8;
                            let a = (255.0 * (((chunk[2] as u32) << 8) as f32 + (chunk[3] as f32))
                                / max) as u8;
                            let color = Color::rgba(col, col, col, a);
                            pixels.push(color)
                        }
                    }
                    _ => (),
                };
            }
            RGB_ALPHA_CTYPE => {
                match self.header.bit_depth {
                    8 => {
                        for chunk in image_data[..].chunks_exact(4) {
                            let color = Color::from_slice(&chunk[..]);
                            if color.alpha() == 0 {
                                pixels.push(color::WHITE);
                            } else {
                                pixels.push(color)
                            }
                        }
                    }
                    16 => {
                        let max = 65535.0;
                        for chunk in image_data[..].chunks_exact(8) {
                            let r = (255.0 * (((chunk[0] as u32) << 8) as f32 + (chunk[1] as f32))
                                / max) as u8;
                            let g = (255.0 * (((chunk[2] as u32) << 8) as f32 + (chunk[3] as f32))
                                / max) as u8;
                            let b = (255.0 * (((chunk[4] as u32) << 8) as f32 + (chunk[5] as f32))
                                / max) as u8;
                            let a = (255.0 * (((chunk[6] as u32) << 8) as f32 + (chunk[7] as f32))
                                / max) as u8;
                            let color = Color::rgba(r, g, b, a);
                            pixels.push(color)
                        }
                    }
                    _ => (),
                };
            }
            _ => (),
        };

        Ok(pixels)
    }
}

///
/// Convert a IHDR struct into a chunck
/// This includes CRC calculation
impl From<IHDR> for Chunk {
    fn from(ihdr: IHDR) -> Chunk {
        let c_type = *IHDR_TYPE;
        let mut data = Vec::<u8>::new();

        data.extend_from_slice(&ihdr.width.to_be_bytes());
        data.extend_from_slice(&ihdr.height.to_be_bytes());

        data.push(ihdr.bit_depth);
        data.push(ihdr.color_type);
        data.push(ihdr.compression);
        data.push(ihdr.filter);
        data.push(ihdr.interlace);
        let mut raw_data: Vec<u8> = c_type.iter().map(|element| *element).collect();
        raw_data.extend_from_slice(&data[..]);
        let crc = crc(&raw_data);
        return Chunk {
            length: SZ_IHDR as u32,
            c_type,
            data,
            crc,
        };
    }
}

/// Attempt parse a gamma struct form a chunk if valid
impl TryFrom<&Chunk> for gAMA {
    type Error = PNGError;
    fn try_from(chunk: &Chunk) -> Result<gAMA, Self::Error> {
        if chunk.c_type == *gAMA_TYPE {
            if chunk.data.len() != 4 {
                Err(PNGError::ParssingError("Invalid gamma data size".into()))
            } else {
                let value =
                    be_bytes_to_u32(&[chunk.data[0], chunk.data[1], chunk.data[2], chunk.data[3]]);
                Ok(gAMA { gamma: value })
            }
        } else {
            Err(PNGError::ParssingError("Invalid gamma format".into()))
        }
    }
}

/******************************************************
 *          Helper functions
 ******************************************************/

///
/// Helper: Parses IHDR data from a given vector.
/// Validates data length
fn parse_ihdr_data(data: &Vec<u8>) -> Result<IHDR, Box<dyn Error>> {
    if data.len() != 13 {
        return Err(Box::new(PNGError::ParssingError(
            "Could not parse IHDR information".into(),
        )));
    } else {
        // Parse each field for the IHDR header
        let width = (data[0] as u32) << 24
            | (data[1] as u32) << 16
            | (data[2] as u32) << 8
            | (data[3] as u32) << 0;

        let height = (data[4] as u32) << 24
            | (data[5] as u32) << 16
            | (data[6] as u32) << 8
            | (data[7] as u32) << 0;

        let bit_depth = data[8];
        let color_type = data[9];
        let compression = data[10];
        let filter = data[11];
        let interlace = data[12];
        return Ok(IHDR {
            width,
            height,
            bit_depth,
            color_type,
            compression,
            filter,
            interlace,
        });
    }
}

///
/// Convert the length array which is given in order of byte MSB-LSB
/// to an integer
/// source: https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
///
fn be_bytes_to_u32(array: &[u8; 4]) -> u32 {
    (array[0] as u32) << 24
        | (array[1] as u32) << 16
        | (array[2] as u32) << 8
        | (array[3] as u32) << 0
}

#[allow(dead_code, unused_variables)]
fn make_crc_table() {
    let mut c: u32;
    for n in 0..256 {
        c = n as u32;
        for _k in 0..8 {
            if c & 1 > 0 {
                c = 0xedb88320_u32 ^ (c >> 1);
            } else {
                c = c >> 1;
            }
        }
        unsafe {
            CRC_TABLE[n] = c;
        };
    }
    unsafe { CRC_INITIALIZED = true };
}
#[allow(dead_code, unused_variables)]
fn update_crc(crc: u32, data: &Vec<u8>) -> u32 {
    let mut c = crc;
    unsafe {
        if !CRC_INITIALIZED {
            make_crc_table();
        }
        for n in 0..data.len() {
            c = CRC_TABLE[(c ^ data[n] as u32) as usize & 0xff] ^ (c >> 8);
        }
    }
    return c;
}

#[allow(dead_code, unused_variables)]
fn crc(data: &Vec<u8>) -> [u8; 4] {
    return (update_crc(0xffffffff_u32, data) ^ 0xffffffff_u32).to_be_bytes();
}

/// validate the given bit depth for the IHDR header chunk
#[allow(dead_code, unused_variables)]
fn valid_bit_depth(color_type: u8, value: u8) -> bool {
    return match color_type {
        GRAY_SCALE_CTYPE => vec![1, 2, 4, 8, 16].contains(&value),
        RGB_CTYPE => vec![8, 12].contains(&value),
        PALETTE_INDEX_CTYPE => vec![1, 2, 4, 8].contains(&value),
        GREY_SCALE_ALPHA_CTYPE => vec![8, 16].contains(&value),
        RGB_ALPHA_CTYPE => vec![8, 16].contains(&value),
        _ => false,
    };
}

/// Remove the applied filters from the picture
/// Different filter algortims are supported.
///
/// Basically the first byte of each scannline in the PNG  is the type of filter applied
/// One must read that byte determine for this line what to do.
///
/// Currently supported filters are:
///     None
///     Sub
///     UP
///     Average
///     Paeth
///
/// Each has its own implementation of course and they all can be found here:
/// https://www.w3.org/TR/REC-png-961001#R.Filtering
///
/// Naturally we error if an unknown filter is given.
///
///     data -> Vector of bytes containing all the decompressed picture data
///  
///     Return the unfiltered image data.
#[allow(dead_code, unused_variables)]
fn remove_filter(
    current_row: &mut Vec<u8>,
    previous_row: &Vec<u8>,
    filter_type: FilterType,
    bpp: usize,
) -> Result<(), PNGError> {
    match filter_type {
        FilterType::Sub => {
            // println!("SUB");
            for j in bpp..current_row.len() {
                current_row[j] = current_row[j].wrapping_add(current_row[j - bpp]);
            }
        }
        FilterType::Up => {
            // println!("UP");
            for j in 0..current_row.len() {
                current_row[j] = current_row[j].wrapping_add(previous_row[j]);
            }
        }
        FilterType::Average => {
            // println!("AVERAGE");
            for j in 0..current_row.len() {
                let mut a = 0;
                let b = previous_row[j];
                if j > bpp {
                    a = current_row[j - bpp];
                }
                current_row[j] = current_row[j].wrapping_add(((a as u32 + b as u32) / 2) as u8);
            }
        }
        FilterType::Paeth => {
            // println!("PAETH");
            for j in 0..bpp {
                current_row[j] =
                    current_row[j].wrapping_add(paeth_predictor(0, previous_row[j], 0));
            }
            for j in bpp..current_row.len() {
                let a = current_row[j - bpp];
                let b = previous_row[j];
                let c = previous_row[j - bpp];
                current_row[j] = current_row[j].wrapping_add(paeth_predictor(a, b, c));
            }
        }
        // no change
        FilterType::None => {}

        // Invalid filter
        _ => {
            // return Err(PNGError::ParssingError(
            //     "Could not identify filter method".into(),
            // ));
        }
    }

    Ok(())
}

/// Paeth filer predictor function
/// https://www.w3.org/TR/REC-png-961001#R.Filtering
fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i32 + b as i32 - c as i32;
    let pa = (p - a as i32).abs();
    let pb = (p - b as i32).abs();
    let pc = (p - c as i32).abs();
    if pa <= pb && pa <= pc {
        return a;
    } else if pb <= pc {
        return b;
    } else {
        return c;
    }
}
