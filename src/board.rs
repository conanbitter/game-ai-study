#[derive(Clone, Copy)]
pub enum CellState {
    None,
    Player,
    Opponent,
}

pub struct Board {
    state: Vec<CellState>,
    pub lines: Vec<[usize; 5]>,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Board {
        // precalculate all groups of five
        let mut lines = vec![];
        // horisontal lines
        for y in 0..rows {
            for x in 0..cols - 4 {
                let y_offset = y * cols;
                lines.push([
                    y_offset + x,
                    y_offset + x + 1,
                    y_offset + x + 2,
                    y_offset + x + 3,
                    y_offset + x + 4,
                ]);
            }
        }
        // vertical lines
        for x in 0..cols {
            for y in 0..rows - 4 {
                lines.push([
                    y * cols + x,
                    (y + 1) * cols + x,
                    (y + 2) * cols + x,
                    (y + 3) * cols + x,
                    (y + 4) * cols + x,
                ]);
            }
        }
        // diagonal lines (downward)
        for x in 0..cols - 4 {
            for y in 0..rows - 4 {
                lines.push([
                    y * cols + x,
                    (y + 1) * cols + x + 1,
                    (y + 2) * cols + x + 2,
                    (y + 3) * cols + x + 3,
                    (y + 4) * cols + x + 4,
                ]);
            }
        }
        // diagonal lines (upward)
        for x in 0..cols - 4 {
            for y in 4..rows {
                lines.push([
                    y * cols + x,
                    (y - 1) * cols + x + 1,
                    (y - 2) * cols + x + 2,
                    (y - 3) * cols + x + 3,
                    (y - 4) * cols + x + 4,
                ]);
            }
        }
        println!("Lines {}", lines.len());

        Board {
            state: vec![CellState::None; rows * cols],
            lines,
        }
    }
}
