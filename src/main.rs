extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // my actual code for now
    let grid_cell_size : usize = 18;
    let grid_width : usize = 64;
    let grid_height : usize = 64;
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

    let glider_starting_point_x = 5;
    let glider_starting_point_y = 5;

    // ================================================================
    // Seed Generation; n = 0;
    grid[glider_starting_point_x + 1][glider_starting_point_y + 2] = 1;
    grid[glider_starting_point_x + 2][glider_starting_point_y + 2] = 1;
    grid[glider_starting_point_x + 3][glider_starting_point_y + 2] = 1;
    grid[glider_starting_point_x + 4][glider_starting_point_y + 2] = 1;
    grid[glider_starting_point_x + 5][glider_starting_point_y + 3] = 1;
    grid[glider_starting_point_x + 4][glider_starting_point_y + 3] = 1;
    // ================================================================


    let window_width : i32 = (grid_width as i32 * grid_cell_size as i32) + 1;
    let window_height : i32 = (grid_height as i32 * grid_cell_size as i32) + 1;

    // ===========================================================================
    // Colour Scheme taken from https://github.com/catsocks/sdl-grid/blob/master/main.c#L27
    let grid_background : Color = Color::RGBA(22, 22, 22, 255); // Barely Black
    let grid_line_colour : Color = Color::RGBA(44, 44, 44, 255); // Dark Gray
    let grid_cursor_ghost_colour : Color = Color::RGBA(44, 44, 44, 255);
    let grid_cursor_colour : Color = Color::RGBA(255, 255, 255, 255);
    // ==========================================

    let window = video_subsystem.window("Simple Naive Implementation of Conway's Game Of Life", window_width as u32, window_height as u32)
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
                // We Move one Generation forward on Key Press X;
                Event::KeyDown {keycode: Some(Keycode::X), ..} => {
                },
                _ => {}
            }
        }
        
        //===========================
        // Perform Game Of Life Simulation/Calculations
        
        x = 0;
        y = 0;
        while x < grid_width {
            y = 0;
            while y < grid_height {
                //println!("We Are calculating [{:?},{:?}]", x, y);
                // first we calculate the living neighbours of a cell
                // using the following
                // [-1,-1], [-1, 0], [-1, 1]
                // [0, -1], [0, 0],  [0, +1]
                // [+1, -1],[+1, 0], [+1, +1]
                let mut n_x = x;
                let mut n_y = y;
                let c_x = x;
                let c_y = y;
                let mut living = 0;
                for m_x in -1..2 {
                    for m_y in -1..2 {
                        //println!("Looking at location relative [{:?},{:?}]", m_x, m_y);
                        
                        n_x = match add(n_x, m_x) {
                            Some(n) => n,
                            None => 0,
                        };
                        n_y = match add(n_y, m_y) {
                            Some(n) => n,
                            None => 0,
                        };
                        //println!("{:?} / {:?}", n_x, n_y);
                        if n_x < grid_width && n_y < grid_height {
                            if grid[n_x][n_y] == 1 {
                                living += 1;
                                //println!("Living Entity Found [{:?}, {:?}]", n_x, n_y);
                            }
                        }
                    }

                    n_x = c_x;
                    n_y = c_y;
                }
                //println!("Living Cells: {:?}", living);
                // Rules taken from https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
                // cell is alive
                if grid[x][y] == 1 {
                    // Any live cell with two or three live neighbours lives on to the next generation.
                    if living == 2 || living == 3 {
                        grid[x][y] = 1;
                    }
                    // Any live cell with more than three live neighbours dies, as if by overpopulation.
                    else if living > 3 {
                        grid[x][y] = 0;
                    }
                    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    else {
                        grid[x][y] = 0;
                    }
                }
                // cell is dead
                else if grid[x][y] == 0 {
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    if living == 3 {
                        grid[x][y] = 1;
                    }
                }

                y += 1;
            }
    
            x += 1;
        }
        
        //===========================
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