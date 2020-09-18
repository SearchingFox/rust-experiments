use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const WINDOW_WIDTH:  u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

type Complex = (f64, f64);

fn mandelbrot((a, b): Complex) -> Color {
    let mut t = (0.0, 0.0);
    for i in 0..1000 {
        t = (t.0*t.0 - t.1*t.1 + a, 2.0*t.0*t.1 + b);
        if f64::sqrt(t.0*t.0 + t.1*t.1) > 4.0 { // try without sqrt
            return Color::RGB((i % 255) as u8, (i % 255) as u8, (i * 10 % 255) as u8)
            // return Color::RGB(i * 20 % 255, i * 20 % 255, i * 20 % 255);
        }
    }
    Color::BLACK
}

fn broken_mandelbrot((a, b): Complex) -> Color {
    let (mut x, mut y) = (0.0, 0.0);
    for i in 0..1000 {
        x = x * x - y * y + a;
        y = 2.0 * x * y + b;  // wrong x
        if f64::sqrt(x * x + y * y) > 2.0 {
            return Color::RGB((i * 20 % 255) as u8, (i * 20 % 255) as u8, (i * 20 % 255) as u8);
        }
    }
    Color::BLACK
}

fn burning_ship((mut a, mut b): Complex) -> Color {
    a += -17.;
    b += -0.5;
    let s = 10.;
    a /= s;
    b /= s;

    let mut z = (0.0, 0.0);
    for i in 0..1000 {
        z = (z.0*z.0 - z.1*z.1 + a, 2.0 * f64::abs(z.0*z.1) + b);
        if z.0*z.0 + z.1*z.1 > 4.0 {
            return Color::RGB(100 + (i % 155) as u8, 30 + (i % 100) as u8, 0)
        }
    }
    Color::BLACK
}

fn to_compl_plain(x: u32, y: u32) -> Complex {
    return ((x as f64 - WINDOW_WIDTH as f64 / 2.) * 3.4 / WINDOW_WIDTH as f64,
            (y as f64 - WINDOW_HEIGHT as f64 / 2.) * 2. / WINDOW_HEIGHT as f64)
}

pub fn main_fractals() -> Result<(), Box<dyn Error>> {
    let temp_cache: Arc<Mutex<Vec<(u32, Vec<Color>)>>> = Arc::new(Mutex::new(vec![]));
    let mut threads = vec![];
    for t_num in 0..5 {
        threads.push(thread::spawn({
            let cln = Arc::clone(&temp_cache);
            move || {
                let mut v = cln.lock().unwrap();
                let a = (WINDOW_WIDTH / 5) as u32;
                for i in a*t_num..a*(t_num+1) {
                    let mut temp = Vec::<Color>::with_capacity(WINDOW_HEIGHT as usize);
                    for j in 0..WINDOW_HEIGHT {
                        temp.push(broken_mandelbrot(to_compl_plain(i, j)));
                    }
                    v.push((i, temp));
                }
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }

    let mut cache = Arc::try_unwrap(temp_cache).unwrap().into_inner().unwrap();
    cache.sort_by(|(x, _), (y, _)| x.cmp(y));

    let sdl_context = sdl2::init()?;
    let window = sdl_context.video()?
        .window("Fractals", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().build()?;

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
                canvas.set_draw_color(cache[i as usize].1[j as usize]);
                canvas.draw_point(t)?;
            }
        }

        canvas.present();

        const FPS: u32 = 5;
        std::thread::sleep(Duration::new(1, 1_000_000_000u32 / FPS));
    }

    Ok(())
}