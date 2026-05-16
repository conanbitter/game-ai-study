use crate::graphics::Bitmap;

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

pub const CELL_SIZE: usize = 38;

static BITMAP_CELL_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/cell.raw");

pub static BITMAP_CELL: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_CELL_DATA,
};

static BITMAP_X_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/x.raw");

pub static BITMAP_X: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_X_DATA,
};

static BITMAP_O_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/o.raw");

pub static BITMAP_O: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_O_DATA,
};

static BITMAP_XF_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/x_faded.raw");

pub static BITMAP_XF: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_XF_DATA,
};

static BITMAP_OF_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/o_faded.raw");

pub static BITMAP_OF: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_OF_DATA,
};

static BITMAP_SELECT_DATA: [u32; CELL_SIZE * CELL_SIZE] = include_u32!("../assets/selected.raw");

pub static BITMAP_SELECT: Bitmap = Bitmap {
    width: CELL_SIZE,
    height: CELL_SIZE,
    data: &BITMAP_SELECT_DATA,
};
