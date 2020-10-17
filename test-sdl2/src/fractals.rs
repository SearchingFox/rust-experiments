use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const WINDOW_HEIGHT: u32 = 1080;
const WINDOW_WIDTH:  u32 = 1920;

type Complex = (f64, f64);

fn mandelbrot((a, b): Complex) -> Color {
    // let xs = 0.0;
    // let ys = 0.0;
    // let s = 1.0;
    let xs = -6.;
    let ys = -4.;
    let s = 3.;

    let mut z = (0.0, 0.0);
    for i in 0..500 {
        z = ((z.0*z.0 - z.1*z.1 + a + xs) / s, (2.0*z.0*z.1 + b + ys) / s);
        if f64::sqrt(z.0*z.0 + z.1*z.1) > 4.0 { // try without sqrt
        // if t.0*t.0 + t.1*t.1 > 16.0 {
            return Color::RGB( (i * 3 % 255) as u8
                             , (i % 255) as u8
                             , (i * 10 % 255) as u8)
        }
    }
    Color::BLACK
}

// Formula: Z_n+1 = tan((Z_n^(1+(t/200)))+c), t is a frame number
fn test((a, b): Complex) -> Color {
    let xs = 0.0; // -26.;
    let ys = 0.0; // -2.;
    let s = 1.0; // 30.;
    let mut z = (0.1, 0.1);
    for i in 0..500 {
        // t = ((t.0*t.0 - t.1*t.1 + a + xs) / s, (2.0*t.0*t.1 + b + ys) / s);
        // t = (f64::sin(t.0) * f64::cosh(t.1), f64::cos(t.0) * f64::sinh(t.1)); // sin
        // t = (f64::cos(t.0) * f64::cosh(t.1), -f64::sin(t.0) * f64::sinh(t.1)); // cos
        let phi = f64::atan2(z.1, z.0) * (1.0 / 200.0 + 1.0); // swap args ?
        let rho = f64::sqrt(z.0*z.0 + z.1*z.1).powf(1.0 / 200.0 + 1.0);
        z = (rho * f64::cos(phi), rho * f64::sin(phi));
        z = (f64::sin(2.0 * z.0) / (f64::cos(2.0 * z.0) + f64::cosh(2.0 * z.1)),
             f64::sinh(2.0 * z.1) / (f64::cos(2.0 * z.0) + f64::cosh(2.0 * z.1)));
        if f64::sqrt(z.0*z.0 + z.1*z.1) > 4.0 { // try without sqrt
            return Color::RGB((i % 255) as u8, (i % 255) as u8, (i * 10 % 255) as u8)
            // return Color::RGB(i * 20 % 255, i * 20 % 255, i * 20 % 255);
        }
    }
    Color::BLACK
}

fn broken_mandelbrot((mut a, mut b): Complex) -> Color {
    a += -17.;
    b += -0.5;
    let s = 10.;
    a /= s;
    b /= s;
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

fn burning_ship((a, b): Complex) -> Color {
    let xs = -17.;
    let ys = -0.5;
    let s = 10.;

    let mut z = (0.0, 0.0);
    for i in 0..1000 {
        z = (z.0*z.0 - z.1*z.1 + (a + xs) / s, 2.0 * f64::abs(z.0*z.1) + (b + ys) / s);
        if f64::sqrt(z.0*z.0 + z.1*z.1) > 4.0 {
            return Color::RGB(100 + (i % 155) as u8, 30 + (i % 100) as u8, 0)
        }
    }
    Color::BLACK
}

fn to_compl_plain(x: u32, y: u32) -> Complex {
    return ((x as f64 - WINDOW_WIDTH as f64 / 2.) * 3.4 / WINDOW_WIDTH as f64,
            (y as f64 * 2. / WINDOW_HEIGHT as f64 - 1.0))
}

pub fn main_fractals() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let mut cache: Vec<Vec<Color>> = Vec::new();
    for i in 0..WINDOW_WIDTH {
        let mut t = vec![];
        for j in 0..WINDOW_HEIGHT {
            t.push(burning_ship(to_compl_plain(i, j)));
        }
        cache.push(t);
    }
    // let temp_cache: Arc<Mutex<Vec<(u32, Vec<Color>)>>> = Arc::new(Mutex::new(vec![]));
    // let mut threads = vec![];
    // let threads_num = 16;
    // for t_num in 0..16 {
    //     threads.push(thread::spawn({
    //         let cln = Arc::clone(&temp_cache);
    //         move || {
    //             let mut v = cln.lock().unwrap();
    //             let a = (WINDOW_WIDTH / 16) as u32;
    //             for i in a*t_num..a*(t_num+1) {
    //                 let mut temp = Vec::<Color>::with_capacity(WINDOW_HEIGHT as usize);
    //                 for j in 0..WINDOW_HEIGHT {
    //                     temp.push(test(to_compl_plain(i, j)));
    //                 }
    //                 v.push((i, temp));
    //             }
    //         }
    //     }));
    // }
    // for t in threads {
    //     t.join().unwrap();
    // }
    // println!("{}", now.elapsed().as_secs());

    // let mut cache = Arc::try_unwrap(temp_cache).unwrap().into_inner().unwrap();
    // cache.sort_by(|(x, _), (y, _)| x.cmp(y));
    println!("{}", now.elapsed().as_millis());

    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context.video()?
        .window("Fractals", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered().build()?
        .into_canvas().build()?;

    for i in 0..WINDOW_WIDTH {
        for j in 0..WINDOW_HEIGHT {
            canvas.set_draw_color(cache[i as usize][j as usize]);  // mandelbrot(to_compl_plain(i, j))
            canvas.draw_point(Point::new(i as i32, j as i32))?;
            
        }
    }
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running },
                _ => {}
            }
        }
        thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
    }

    Ok(())
}