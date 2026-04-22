use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;

use crate::helpers::*;

pub fn view_bmp<P: AsRef<Path>>(path: P) {
    let mut bmp_file = File::open(path).unwrap();

    let bmp_header = BmpFileHeader::read_from_here(&mut bmp_file).unwrap();
    let dib_header = BmpInfoHeader::read_from_here(&mut bmp_file).unwrap();
    println!("Header: {:#?}", dib_header);
}


#[derive(Debug)]
struct BmpFileHeader {
    /// File size in bytes
    size: u32,

    /// The starting address of the pixel array
    pixel_arr_offset: u32
}

impl BmpFileHeader {
    /// Assumes you are already at the start of the file
    fn read_from_here<R: Read>(file: &mut R) -> Option<Self> {
        let mut bfr = [0; 14];

        if file.read(&mut bfr).unwrap() < bfr.len() {
            return None
        }

        Some(Self{
            size: u32_from_le_slice(&bfr, 2),
            pixel_arr_offset: u32_from_le_slice(&bfr, 10),
        })
    }

    /// Goes to the start of file to read, then returns
    fn read_from_start<R: Read + Seek>(file: &mut R) -> Option<Self> {
        let original_pos = file.stream_position().unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let bmp_header = Self::read_from_here(file);
        file.seek(SeekFrom::Start(original_pos)).unwrap();

        bmp_header
    }
}


#[derive(Debug)]
struct BmpInfoHeader {
    /// The bitmap width in pixels
    width: u32,

    /// The bitmap height in pixels
    height: u32,

    /// The number of bits per ipxels
    bpp: u16,

    /// The compression method used
    compression: u32,

    /// Horizontal resolution (pixel per metre)
    hor_res: u32,

    /// Vertical resolution (pixel per metre)
    ver_res: u32,

    /// The numbers of colors in the color palatte. 0 to default to 2^bits_per_pixel
    colors: u32,
}


impl BmpInfoHeader {
    fn read_from_here<R: Read>(file: &mut R) -> Option<Self> {
        let mut bfr = [0; 40];

        if file.read(&mut bfr).unwrap() < bfr.len() {
            return None
        }

        if u32_from_le_slice(&bfr, 0) != 40 {
            println!("Unknown DIB header");
            return None;
        }

        let mut header = Self {
            width: u32_from_le_slice(&bfr, 4),
            height: u32_from_le_slice(&bfr, 8),
            bpp: u16_from_le_slice(&bfr, 14),
            compression: u32_from_le_slice(&bfr, 16),
            hor_res: u32_from_le_slice(&bfr, 24),
            ver_res: u32_from_le_slice(&bfr, 28),
            colors: u32_from_le_slice(&bfr, 32),
        };

        if header.colors == 0 {
            // Easy way to do n^2
            header.colors = 1 << header.bpp;
        }

        Some(header)
    }
}