use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

const WINDOW_HEIGHT: u32 = 1080;
const WINDOW_WIDTH: u32 = 1920;

type Complex = (f64, f64);

fn mandelbrot((a, b): Complex) -> Color {
    let mut z = (0.0, 0.0);
    for i in 0..500 {
        z = (
            z.0 * z.0 - z.1 * z.1 + a,
            2.0 * z.0 * z.1 + b,
        );
        if z.0 * z.0 + z.1 * z.1 > 4.0 {
            return Color::RGB((i * 3 % 255) as u8, (i % 255) as u8, (i * 10 % 255) as u8);
        }
    }
    Color::BLACK
}

fn burning_ship((a, b): Complex) -> Color {
    let mut z = (0.0, 0.0);
    for i in 0..1000 {
        z = (
            z.0 * z.0 - z.1 * z.1 + a,
            2.0 * f64::abs(z.0 * z.1) + b,
        );
        if f64::sqrt(z.0 * z.0 + z.1 * z.1) > 4.0 {
            return Color::RGB(100 + (i % 155) as u8, 30 + (i % 100) as u8, 0);
        }
    }
    Color::BLACK
}

fn to_compl_plain_from_center((x_c, y_c): (f64, f64), z: f64, x: u32, y: u32) -> Complex {
    let x_min = x_c - 1.6 / z;
    let x_max = x_c + 1.6 / z;
    let y_min = y_c - 0.9 / z;
    let y_max = y_c + 0.9 / z;
    (
        (x_min + (x_max - x_min) * x as f64 / WINDOW_WIDTH as f64),
        (y_max - (y_max - y_min) * y as f64 / WINDOW_HEIGHT as f64),
    )
}

// TODO: add unlimited floating precision
// TODO: make picture less noisy
pub fn main_fractals() -> Result<(), Box<dyn std::error::Error>> {
    // let now = Instant::now();
    let mut center = (-1.0, 0.0);
    let mut z = 1.0;
    let mut cache: Vec<Color> = (0..(WINDOW_HEIGHT * WINDOW_WIDTH))
        .into_par_iter()
        .map(|l| {
            burning_ship(to_compl_plain_from_center(
                center,
                z,
                l / WINDOW_HEIGHT,
                l % WINDOW_HEIGHT,
            ))
        })
        .collect();

    // println!("{}", now.elapsed().as_millis());

    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context
        .video()?
        .window("Fractals", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .vulkan()
        .build()?
        .into_canvas()
        .build()?;

    let texture_creator = canvas.texture_creator();
    let mut texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 1920, 1080)?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for x in 0..1920 {
            for y in 0..1080 {
                let offset = y * pitch + x * 3;
                let c = cache[x * 1080 + y];
                buffer[offset] = c.r;
                buffer[offset + 1] = c.g;
                buffer[offset + 2] = c.b;
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(0, 0, 1920, 1080)))?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    mouse_btn,
                    x: nx,
                    y: ny,
                    ..
                } => {
                    center = to_compl_plain_from_center(center, z, nx as u32, ny as u32);
                    match mouse_btn {
                        MouseButton::Left => z *= 2.0,
                        MouseButton::Right => z /= 2.0,
                        _ => {}
                    }
                    cache = (0..(WINDOW_HEIGHT * WINDOW_WIDTH))
                        .into_par_iter()
                        .map(|l| {
                            burning_ship(to_compl_plain_from_center(
                                center,
                                z,
                                l / WINDOW_HEIGHT,
                                l % WINDOW_HEIGHT,
                            ))
                        })
                        .collect();
                    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                        for x in 0..1920 {
                            for y in 0..1080 {
                                let offset = y * pitch + x * 3;
                                let c = cache[x * 1080 + y];
                                buffer[offset] = c.r;
                                buffer[offset + 1] = c.g;
                                buffer[offset + 2] = c.b;
                            }
                        }
                    })?;

                    canvas.clear();
                    canvas.copy(&texture, None, Some(Rect::new(0, 0, 1920, 1080)))?;
                }
                _ => {}
            }
        }
	
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
