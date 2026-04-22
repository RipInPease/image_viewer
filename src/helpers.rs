pub fn u16_from_le_slice(slice: &[u8], start_i: usize) -> u16 {
    u16::from_le_bytes([
        slice[start_i], 
        slice[start_i + 1]]
    )
}

pub fn u32_from_le_slice(slice: &[u8], start_i: usize) -> u32 {
    u32::from_le_bytes([
        slice[start_i], 
        slice[start_i + 1], 
        slice[start_i + 2], 
        slice[start_i + 3]]
    )
}