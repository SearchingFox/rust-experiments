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

const WINDOW_WIDTH: u32  = 1900; //2500
const WINDOW_HEIGHT: u32 = 1060;

type Complex = (f64, f64);

fn mandelbrot((a, b): Complex) -> Color {
    let mut t = (0.0, 0.0);
    for i in 0..1000 {
        t = (t.0*t.0 - t.1*t.1 + a, 2.0*t.0*t.1 + b);
        if f64::sqrt(t.0*t.0 + t.1*t.1) > 4.0 {
            return Color::RGB((i % 255) as u8, (i % 255) as u8, (i * 10 % 255) as u8)
            // return Color::RGB(i * 20 % 255, i * 20 % 255, i * 20 % 255);
        }
    }
    Color::RGB(0, 0, 0)
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
    Color::RGB(0, 0, 0)
}

fn burning_ship((a, b): Complex) -> Color {
    let mut zx = a + 0.5;
    let mut zy = b;
    let mut iter = 0;
    let mi: u32 = 1000;
    while (zx*zx + zy*zy < 4.0 && iter < mi) {
        let xtemp = zx*zx - zy*zy + a;
        zy = f64::abs(2.0*zx*zy + b);
        zx = f64::abs(xtemp);
        iter += 1;
    }
    if iter == mi { // Belongs to the set
        return Color::RGB(100, 0, 0);
    }
    return Color::RGB(220, (iter * 10 % 255) as u8, (iter * 10 % 255) as u8);
}

fn to_compl_plain(x: u32, y: u32) -> Complex {
    return ((x as f64 - WINDOW_WIDTH as f64 / 2.) * 3.4 / WINDOW_WIDTH as f64,
            (y as f64 - WINDOW_HEIGHT as f64 / 2.) * 2. / WINDOW_HEIGHT as f64)
}

fn to_window((a, b): Complex) -> (u32, u32) {
    return ((a as f64 * WINDOW_WIDTH as f64 / 3.4 + WINDOW_WIDTH as f64 / 2.) as u32,
            (b as f64 * WINDOW_HEIGHT as f64 / 2. - WINDOW_HEIGHT as f64 / 2.) as u32)
}

pub fn main_fractals() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let window = sdl_context.video()?
        .window("Fractals", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let mut cache: Vec<Vec<Color>> = vec![vec![Color::RGB(0, 0, 0); WINDOW_HEIGHT as usize]; WINDOW_WIDTH as usize];
    for i in 0..WINDOW_WIDTH {
        for j in 0..WINDOW_HEIGHT {
            // ? maybe change transition to from complex plane to screen
            cache[i as usize][j as usize] = burning_ship(to_compl_plain(i, j));
        }
    }

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running },
                _ => {}
            }
        }
        
        for i in 0..WINDOW_WIDTH {
            for j in 0..WINDOW_HEIGHT {
                let t: Point = Point::new(i as i32, j as i32);
                canvas.set_draw_color(cache[i as usize][j as usize]);
                canvas.draw_point(t)?;
            }
        }

        canvas.present();

        const FPS: u32 = 5;
        std::thread::sleep(Duration::new(1, 1_000_000_000u32 / FPS));
    }

    Ok(())
}