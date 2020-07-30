#![allow(dead_code)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::error::Error;
use std::time::Duration;

const WINDOW_WIDTH: u32  = 1900;
const WINDOW_HEIGHT: u32 = 1040;
const BACKGROUND: Color = Color::RGB(255, 255, 255); // Color::RGB(18, 18, 18);
const FOREGROUND: Color = Color::RGB(0, 0, 0); // Color::RGB(255, 150, 150);

type Complex = (f64, f64);

fn mand((a, b): Complex) -> Color {
    let mut t = (0.0, 0.0);
    for _ in 0..100 {
        t = (t.0 * t.0 - t.1 * t.1 + a, 2.0 * t.0 * t.1 + b);
        if f64::sqrt(t.0 * t.0 + t.1 * t.1) > 2.0 {
            return Color::RGB(255, 255, 255);
        }
    }
    return Color::RGB(0, 0, 0);
}

fn what((a, b): Complex) -> Color {
    let (mut x, mut y) = (0.0, 0.0);
    for _ in 0..100 {
        x = x * x - y * y + a;
        y = 2.0 * x * y + b;
        if f64::sqrt(x * x + y * y) > 2.0 {
            return Color::RGB(255, 255, 255);
        }
    }
    return Color::RGB(0, 0, 0);
}

fn burning((x, y): Complex) -> Color {
    let mut zx = x + 0.5;
    let mut zy = y + 0.5;
    let mut iter = 0;
    let mi: u32 = 100;
    while (zx*zx + zy*zy < 4.0 && iter < mi) {
        let xtemp = zx*zx - zy*zy + x;
        zy = f64::abs(2.0*zx*zy + y);
        zx = f64::abs(xtemp);
        iter += 1;
    }
    if iter == mi {
        return Color::RGB(100, 0, 0);
    } // Belongs to the set
    return Color::RGB(0, iter as u8, 0);//iter as u8, iter as u8);
}

fn to_compl_plain(x: u32, y: u32) -> Complex {
    return ((x as f64 - WINDOW_WIDTH as f64 / 2.0) / (WINDOW_WIDTH as f64 / 3.0),
            (y as f64 - WINDOW_HEIGHT as f64 / 2.0) / (WINDOW_HEIGHT as f64 / 2.0))
}

pub fn main_fractals() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    //let texture_creator = canvas.texture_creator();

    // let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WINDOW_HEIGHT, WINDOW_WIDTH).map_err(|e| e.to_string())?;
    // // Create a red-green gradient
    // texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
    //     for y in 0..WINDOW_WIDTH {
    //         for x in 0..WINDOW_HEIGHT {
    //             let offset: usize = (y*pitch as u32 + x*3) as usize;
    //             buffer[offset] = x as u8;
    //             buffer[offset + 1] = y as u8;
    //             buffer[offset + 2] = 0;
    //         }
    //     }
    // })?;

    //canvas.clear();
    // canvas.copy(&texture, None, Some(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)))?;
    // canvas.copy_ex(&texture, None,
    //     Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false)?;
    //canvas.present();

    let mut cache: Vec<Vec<Color>> = vec![vec![Color::RGB(0, 0, 0); WINDOW_HEIGHT as usize]; WINDOW_WIDTH as usize];

    for i in 0..WINDOW_WIDTH {
        for j in 0..WINDOW_HEIGHT {
            cache[i as usize][j as usize] = burning(to_compl_plain(i, j))
        }
    }

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(1, 1, 1));
        canvas.clear();

        // for i in 0..WINDOW_WIDTH {
        //     for j in 0..WINDOW_HEIGHT {
        //         let t: Point = Point::new(i as i32, j as i32);
        //         if cache[i as usize][j as usize] {
        //             canvas.set_draw_color(FOREGROUND);
        //         } else {
        //             canvas.set_draw_color(BACKGROUND);
        //         }
        //         canvas.draw_point(t);
        //     }
        // }
        
        for i in 0..WINDOW_WIDTH {
            for j in 0..WINDOW_HEIGHT {
                let t: Point = Point::new(i as i32, j as i32);
                canvas.set_draw_color(cache[i as usize][j as usize]);
                canvas.draw_point(t)?;
            }
        }

        canvas.present();

        const FPS: u32 = 1;
        ::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / FPS));
    }

    Ok(())
}