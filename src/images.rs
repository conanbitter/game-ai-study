use crate::{CELL_SIZE, graphics::Bitmap};

macro_rules! include_u32 {
    ($path:expr) => {{
        const BYTES: &[u8] = include_bytes!($path);
        const _: () = assert!(BYTES.len() % 3 == 0, "The file size must be a multiple of 3 bytes");
        const ARRAY_LEN: usize = BYTES.len() / 3;

        const ARRAY: [u32; ARRAY_LEN] = {
            let mut res = [0u32; ARRAY_LEN];
            let mut i = 0;
            while i < ARRAY_LEN {
                let offset = i * 3;
                res[i] = u32::from_be_bytes([0, BYTES[offset], BYTES[offset + 1], BYTES[offset + 2]]);
                i += 1;
            }
            res
        };
        ARRAY
    }};
}

static BITMAP_CELL_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/cell.raw");

pub static BITMAP_CELL: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_CELL_DATA,
};
