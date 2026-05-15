#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use display_info::DisplayInfo;
use minifb::{Scale, Window, WindowOptions};

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;

fn main() -> anyhow::Result<()> {
    let mut buffer: Vec<u32> = vec![0x98AAB3; WINDOW_WIDTH * WINDOW_HEIGHT];

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

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;
    }

    Ok(())
}
