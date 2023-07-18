use std::io::Write;

#[derive(Debug, Clone)]
pub struct Bitmap{ data: Vec<[u8; 3]>, width: usize} 

impl Bitmap {
    pub fn new(data: Vec<[u8; 3]>, width: usize) -> Self {
        Self { data, width }
    }

    pub fn push_pixel(&mut self, rgb: [u8; 3]) {
        self.data.push(rgb);
    }

    fn generate_header(&self, img_size: i32, img_width: i32, img_height: i32) -> [u8; 54] {
        let s = img_size.to_le_bytes();
        let w = img_width.to_le_bytes();
        let h = (-img_height).to_le_bytes();
        let t = (img_size + 54).to_le_bytes();
        [
            // File Header:
            0x42, 0x4D,             // Header Field (BM)
            t[0], t[1], t[2], t[3], // Size of BMP File (in bytes)
            0x00, 0x00,             // Reserved; zeroed.
            0x00, 0x00,             // Reserved; zeroed.
            0x36, 0x00, 0x00, 0x00, // Offset - start address of pixel array. (14 + 40)
            // Windows BITMAPINFO Header:
            0x28, 0x00, 0x00, 0x00, // Size of header
            w[0], w[1], w[2], w[3], // Width of image (in pixels)
            h[0], h[1], h[2], h[3], // Height of image (in pixels)
            0x01, 0x00,             // Number of Colour Planes
            0x18, 0x00,             // Number of bits per pixel (24, 8 per channel)
            0x00, 0x00, 0x00, 0x00, // Compression Method (None)
            s[0], s[1], s[2], s[3], // Image Size.
            0x13, 0x0B, 0x00, 0x00, // Horizontal resolution (pixels-per-metre)
            0x13, 0x0B, 0x00, 0x00, // Vertical resolution (pixels-per-metre)
            0x00, 0x00, 0x00, 0x00, // Number of colours in palette (0 defaults to 2^n)
            0x00, 0x00, 0x00, 0x00  // Important colours used - ignored.
        ]
    }

    pub fn output(&self, mut out_stream: impl Write) -> std::io::Result<()> {
        let padding = vec![0; (self.width * 3) % 4];
        let row_length = (self.width * 3 + padding.len()) as i32;
        let nrow = (self.data.len() / self.width) as i32;
        
        // Write Header:
        let header = self.generate_header(nrow * row_length, self.width as i32, nrow);
        out_stream.write_all(&header)?;
        // Write pixel array, row by row:
        for pixels in self.data.chunks(self.width) {
            for &[r, g, b] in pixels {
                out_stream.write_all(&[b, g, r])?;
            }
            out_stream.write_all(&padding)?;
        }
        Ok(())
    }
}