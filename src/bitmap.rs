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

    pub fn output(&self, mut out_stream: impl Write) -> std::io::Result<()> {
        let row_length = ((self.width * 24 + 31) / 32) * 4;
        let pad_bytes = (self.width * 3) % 4;
        let padding = vec![0; pad_bytes];

        let nrow = (self.data.len() as f32 / self.width as f32).ceil() as usize;
        let bytes_len = (nrow * row_length) as i32;
        let size = &(bytes_len + 54 as i32).to_le_bytes();

        let w = (self.width as i32).to_le_bytes();
        let h = (-(((self.data.len()) / self.width) as i32)).to_le_bytes(); 
        let bytes_len = bytes_len.to_le_bytes();

        let header: Vec<u8> = vec![
            // File Header:
            0x42, 0x4D,             // Header Field (BM)
            size[0], size[1], size[2], size[3], // Size of BMP File (in bytes)
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
            bytes_len[0], bytes_len[1], bytes_len[2], bytes_len[3], // Image Size.
            0x13, 0x0B, 0x00, 0x00, // Horizontal resolution (pixels-per-metre)
            0x13, 0x0B, 0x00, 0x00, // Vertical resolution (pixels-per-metre)
            0x00, 0x00, 0x00, 0x00, // Number of colours in palette (0 defaults to 2^n)
            0x00, 0x00, 0x00, 0x00  // Important colours used - ignored.
        ];
        
        // Write Header:
        out_stream.write_all(&header)?;

        // Write pixel array, row by row:
        for pixel in self.data.chunks(self.width) {
            for &[r, g, b] in pixel {
                out_stream.write_all(&[b, g, r])?;
            }
            out_stream.write_all(&padding)?;
        }

        Ok(())
    }
}