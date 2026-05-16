#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod graphics;
mod images;

use display_info::DisplayInfo;
use minifb::{Scale, Window, WindowOptions};

use crate::{
    graphics::Framebuffer,
    images::{BITMAP_CELL, BITMAP_O, BITMAP_OF, BITMAP_SELECT, BITMAP_X, BITMAP_XF, CELL_SIZE},
};

const BORDER: usize = 10;
const BOARD_COLS: usize = 15;
const BOARD_ROWS: usize = 10;

const WINDOW_WIDTH: usize = BORDER * 2 + BOARD_COLS * CELL_SIZE;
const WINDOW_HEIGHT: usize = BORDER * 2 + BOARD_ROWS * CELL_SIZE;

const BACKGROUND_COLOR: u32 = 0x98AAB3;

fn main() -> anyhow::Result<()> {
    let mut buffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT, BACKGROUND_COLOR);

    let mut window = Window::new(
        "Game AI",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X1,
            scale_mode: minifb::ScaleMode::Center,
            ..WindowOptions::default()
        },
    )?;

    let (wx, wy) = window.get_position();

    let display_info = DisplayInfo::from_point(wx as i32, wy as i32)?;

    let new_pos_x = (display_info.width as isize - WINDOW_WIDTH as isize) / 2;
    let new_pos_y = (display_info.height as isize - WINDOW_HEIGHT as isize) / 2;
    window.set_position(new_pos_x, new_pos_y);

    window.set_target_fps(60);

    for y in 0..BOARD_ROWS {
        for x in 0..BOARD_COLS {
            buffer.blit(&BITMAP_CELL, BORDER + x * CELL_SIZE, BORDER + y * CELL_SIZE);
        }
    }

    buffer.blit(&BITMAP_CELL, 100, 100);
    buffer.blit(&BITMAP_X, 140, 100);
    buffer.blit(&BITMAP_XF, 180, 100);
    buffer.blit(&BITMAP_O, 140, 140);
    buffer.blit(&BITMAP_OF, 180, 140);
    buffer.blit(&BITMAP_SELECT, 100, 140);

    let mut cursor_pos: (i32, i32) = (0, 0);
    let mut cursor_show = false;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            let new_cursor_pos = (
                ((mouse_pos.0 - BORDER as f32) / CELL_SIZE as f32).floor() as i32,
                ((mouse_pos.1 - BORDER as f32) / CELL_SIZE as f32).floor() as i32,
            );
            let new_cursor_show = new_cursor_pos.0 >= 0
                && new_cursor_pos.1 >= 0
                && new_cursor_pos.0 < BOARD_COLS as i32
                && new_cursor_pos.1 < BOARD_ROWS as i32;

            if cursor_show && (!new_cursor_show || new_cursor_pos != cursor_pos) {
                buffer.blit(
                    &BITMAP_CELL,
                    BORDER + cursor_pos.0 as usize * CELL_SIZE,
                    BORDER + cursor_pos.1 as usize * CELL_SIZE,
                );
            }

            if new_cursor_pos != cursor_pos && new_cursor_show {
                buffer.blit(
                    &BITMAP_SELECT,
                    BORDER + new_cursor_pos.0 as usize * CELL_SIZE,
                    BORDER + new_cursor_pos.1 as usize * CELL_SIZE,
                );
            }

            cursor_pos = new_cursor_pos;
            cursor_show = new_cursor_show;
        } else {
            let new_cursor_show = false;
            if cursor_show && !new_cursor_show {
                buffer.blit(
                    &BITMAP_CELL,
                    BORDER + cursor_pos.0 as usize * CELL_SIZE,
                    BORDER + cursor_pos.1 as usize * CELL_SIZE,
                );
            }
        }
        window.update_with_buffer(&buffer.data, WINDOW_WIDTH, WINDOW_HEIGHT)?;
    }

    Ok(())
}
