use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;

pub fn view_bmp<P: AsRef<Path>>(path: P) {
    let mut bmp_file = File::open(path).unwrap();

    let header = BmpFileHeader::read_from_here(&mut bmp_file).unwrap();
    println!("{:#?}", header);
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
            size: u32::from_le_bytes([bfr[2], bfr[3], bfr[4], bfr[5]]),
            pixel_arr_offset: u32::from_le_bytes([bfr[10], bfr[11], bfr[12], bfr[13]]),
        })
    }

    /// Goes to the start of file to read, then returns
    fn read_from_start<R: Read + Seek>(file: &mut R) -> Option<Self> {
        let original_pos = file.stream_position().unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut bfr = [0; 14];

        if file.read(&mut bfr).unwrap() < bfr.len() {
            return None
        }

        file.seek(SeekFrom::Start(original_pos)).unwrap();
        Some(Self{
            size: u32::from_le_bytes([bfr[2], bfr[3], bfr[4], bfr[5]]),
            pixel_arr_offset: u32::from_le_bytes([bfr[10], bfr[11], bfr[12], bfr[13]]),
        })
    }
}