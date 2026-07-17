pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, bg_color: (u8, u8, u8)) -> Self {
        let mut fb = Framebuffer {
            width,
            height,
            data: vec![0; (width * height * 3) as usize],
        };
        fb.clear(bg_color);
        fb
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for px in self.data.chunks_exact_mut(3) {
            px[0] = color.0;
            px[1] = color.1;
            px[2] = color.2;
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 3) as usize;
        self.data[idx] = color.0;
        self.data[idx + 1] = color.1;
        self.data[idx + 2] = color.2;
    }
}