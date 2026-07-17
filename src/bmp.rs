use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Write};

pub fn save_bmp(fb: &Framebuffer, path: &str) -> io::Result<()> {
    let width = fb.width;
    let height = fb.height;

    let row_size = ((width * 3 + 3) / 4) * 4;
    let padding = row_size - width * 3;
    let pixel_data_size = row_size * height;

    let file_header_size = 14;
    let info_header_size = 40;
    let data_offset = file_header_size + info_header_size;
    let file_size = data_offset + pixel_data_size;

    let mut buf = Vec::with_capacity(file_size as usize);

    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&(file_size as u32).to_le_bytes());
    buf.extend_from_slice(&0u16.to_le_bytes()); // reservado
    buf.extend_from_slice(&0u16.to_le_bytes()); // reservado
    buf.extend_from_slice(&(data_offset as u32).to_le_bytes());

    buf.extend_from_slice(&(info_header_size as u32).to_le_bytes());
    buf.extend_from_slice(&width.to_le_bytes());
    buf.extend_from_slice(&height.to_le_bytes()); 
    buf.extend_from_slice(&1u16.to_le_bytes()); 
    buf.extend_from_slice(&24u16.to_le_bytes()); 
    buf.extend_from_slice(&0u32.to_le_bytes()); 
    buf.extend_from_slice(&(pixel_data_size as u32).to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes()); 
    buf.extend_from_slice(&2835i32.to_le_bytes()); 
    buf.extend_from_slice(&0u32.to_le_bytes()); 
    buf.extend_from_slice(&0u32.to_le_bytes()); 

    for y in (0..height).rev() {
        for x in 0..width {
            let idx = ((y * width + x) * 3) as usize;
            let r = fb.data[idx];
            let g = fb.data[idx + 1];
            let b = fb.data[idx + 2];
            buf.push(b);
            buf.push(g);
            buf.push(r);
        }
        for _ in 0..padding {
            buf.push(0);
        }
    }

    let mut file = File::create(path)?;
    file.write_all(&buf)?;
    Ok(())
}