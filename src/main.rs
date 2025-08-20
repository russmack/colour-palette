extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::process;

pub mod colour;

use colour::HSVf;

const WIDTH: usize = 720;
const HEIGHT: usize = 200;

const BUFFER_WIDTH: usize = 720;
const BUFFER_HEIGHT: usize = 200;

const SAMPLE_WIN_WIDTH: usize = 100;
const SAMPLE_WIN_HEIGHT: usize = 25;

const SAMPLE_WIN_BUF_SIZE: usize = WIDTH * HEIGHT;

const COORDS_ORIGIN_TOP_LEFT: bool = true;

fn coords_to_hsvf(width: usize, height: usize, x: usize, y: usize, invert_y: bool) -> HSVf {
    let hue: f64 = (360.0 / width as f64) * x as f64;

    let sat: f64 = match y {
        _ if y <= height/2 => y as f64 * (1.0 / (height/2) as f64),
        _ => 1.0,
    };

    let val: f64 = match y {
        _ if y > height/2 => (height - y) as f64 * (1.0 / (height/2) as f64),
        _ => 1.0,
    };

    if invert_y {
        HSVf {h: hue, s: val, v: sat}  // Bottom-left origin: 0, 0.
    } else {
        HSVf {h: hue, s: sat, v: val}  // Top-left origin: 0, 0.
    }
}

fn main() {
    // Main, big window for colour palette.
    let mut window = match Window::new("Colour Palette", WIDTH, HEIGHT, 
        WindowOptions {
            ..WindowOptions::default()
        }) {
        Ok(win)     => win,
        Err(err)    => {
            println!("Unable to create main window {}", err);
            return;
        }
    };
    let mut buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);

    // Small window for colour sample.
    let mut sample_win = match Window::new("", SAMPLE_WIN_WIDTH, SAMPLE_WIN_HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        }) {
        Ok(win)     => win,
        Err(err)    => {
            println!("Unable to create colour sample window {}", err);
            return;
        }
    };
    let mut sample_win_buf: [u32; SAMPLE_WIN_BUF_SIZE] = [0; SAMPLE_WIN_BUF_SIZE];

    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
            let new_size = window.get_size();
            if new_size != size {
                size = new_size;
                buffer.resize(size.0 * size.1, 0);
            }
        }

        let mut i = 0;
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                // Use the same coordinate system as the mouse - top-left: 0, 0.
                let coord_y = HEIGHT - y;
                let hsvf = coords_to_hsvf(WIDTH, HEIGHT, x as usize, coord_y as usize, !COORDS_ORIGIN_TOP_LEFT);

                let rgbf = match hsvf.to_rgbf() {
                    Ok(v)   => v,
                    Err(e)  => {
                        println!("error converting hsvf to rgbf: {}", e);
                        process::exit(1);
                    },
                };

                let ir = (255.99 * rgbf.r as f32).floor() as u32 * 65536;
                let ig = (255.99 * rgbf.g as f32).floor() as u32 * 256;
                let ib = (255.99 * rgbf.b as f32).floor() as u32;

                buffer[i] = ir + ig + ib;

                i += 1;
            }
        }

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let hsvf = coords_to_hsvf(WIDTH, HEIGHT, x as usize, y as usize, !COORDS_ORIGIN_TOP_LEFT);

            let rgbf = match hsvf.to_rgbf() {
                Ok(v)   => v,
                Err(e)  => {
                    println!("error converting hsvf to rgbf: {}", e);
                    process::exit(1);
                },
            };

            let ir = (255.99 * rgbf.r as f32).floor() as u32 * 65536;
            let ig = (255.99 * rgbf.g as f32).floor() as u32 * 256;
            let ib = (255.99 * rgbf.b as f32).floor() as u32;

            let rgb = rgbf.to_u8();

            let win_title = format!( "[ x: {}, y: {} ]  r: {}, g: {}, b: {}",
                x.floor(), y.floor(), rgb.r, rgb.g, rgb.b);

            window.set_title(&win_title);
            
            if window.get_mouse_down(MouseButton::Left) {
                println!("{}", win_title);
            }

            // Update the sample window buffer.
            let rgb_colour = ir + ig + ib;
            
            for f in sample_win_buf.iter_mut() {
                *f = rgb_colour;
            }
        };

        window.update_with_buffer(&buffer, BUFFER_WIDTH, BUFFER_HEIGHT).unwrap();
        sample_win.update_with_buffer(&sample_win_buf, BUFFER_WIDTH, BUFFER_HEIGHT).unwrap();
    }
}

