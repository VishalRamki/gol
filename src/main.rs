extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // my actual code for now
    let grid_cell_size : usize = 36;
    let grid_width : usize = 29;
    let grid_height : usize = 23;
    let mut grid = vec![vec![0; grid_height]; grid_width];
    let mut x = 0;
    let mut y = 0;
    // perform actions on grid
    while x < grid_width {
        y = 0;
        while y < grid_height {
            // zero the grid
            grid[x][y] = 0;
            y += 1;
        }
        x += 1;
    }

    let window_width : i32 = (grid_width as i32 * grid_cell_size as i32) + 1;
    let window_height : i32 = (grid_height as i32 * grid_cell_size as i32) + 1;

    let grid_background : Color = Color::RGBA(22, 22, 22, 255); // Barely Black
    let grid_line_colour : Color = Color::RGBA(44, 44, 44, 255); // Dark Gray
    let grid_cursor_ghost_colour : Color = Color::RGBA(44, 44, 44, 255);
    let grid_cursor_colour : Color = Color::RGBA(255, 255, 255, 255);
    // ==========================================

    let window = video_subsystem.window("rust-sdl2 demo: Video", window_width as u32, window_height as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(grid_background);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    
        // draw the background
        canvas.set_draw_color(grid_background);
        canvas.clear();

        canvas.set_draw_color(grid_line_colour);
        // draw the horizontal lines
        let mut i : i32 = 0;
        while i < 1 + grid_width as i32 * grid_cell_size as i32 {
            let exrr = canvas.draw_line(Point::new(i, 0),Point::new(i, window_height));
            i += grid_cell_size as i32;
        }
        // draw vertical lines
        i = 0;
        while i < 1 + grid_height as i32 * grid_cell_size as i32 {
            let exrr = canvas.draw_line(Point::new(0, i),Point::new(window_width, i));
            i += grid_cell_size as i32;
        }

        // draw 'on' cells
        x = 0;
        y = 0;
        while x < grid_width {
            y = 0;
            while y < grid_height {
                if grid[x][y] == 1 {
                    canvas.set_draw_color(grid_cursor_colour);
                    let exrr = canvas.fill_rect(Rect::new(x as i32 * grid_cell_size as i32, y as i32 * grid_cell_size as i32, grid_cell_size as u32, grid_cell_size as u32));
                }

                y += 1;
            }
    
            x += 1;
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}