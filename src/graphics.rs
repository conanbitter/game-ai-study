pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub data: &'static [u32],
}

pub struct Framebuffer {
    width: usize,
    height: usize,
    pub data: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background: u32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            data: vec![background; width * height],
        }
    }

    pub fn blit(&mut self, image: &Bitmap, x: usize, y: usize) {
        for src_y in 0..image.height {
            let src_row = src_y * image.width;
            let src_slice = &image.data[src_row..src_row + image.width];

            let dst_y = src_y + y;
            let dst_row = x + dst_y * self.width;
            let dst_slice = &mut self.data[dst_row..dst_row + image.width];

            dst_slice.copy_from_slice(src_slice);
        }
    }
}
