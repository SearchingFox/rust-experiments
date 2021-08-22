use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use std::time::{Duration, Instant};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

const WINDOW_HEIGHT: u32 = 1080;
const WINDOW_WIDTH:  u32 = 1920;

type Complex = (f64, f64);

fn mandelbrot((a, b): Complex) -> Color {
    let xs = 6.1;
    let ys = 0.7;
    let s = 4.9;
    // let xs = -6.;
    // let ys = -4.;
    // let s = 3.;

    let mut z = (0.0, 0.0);
    for i in 0..500 {
        z = ( (z.0*z.0 - z.1*z.1 + a + xs) / s
            , (2.0*z.0*z.1 + b + ys) / s );
        if z.0*z.0 + z.1*z.1 > 4.0 {
            return Color::RGB( (i * 3 % 255) as u8
                             , (i % 255) as u8
                             , (i * 10 % 255) as u8 )
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
        z = ((z.0*z.0 - z.1*z.1 + a + xs) / s, (2.0*z.0*z.1 + b + ys) / s);
        // z = (f64::sin(z.0) * f64::cosh(z.1), f64::cos(z.0) * f64::sinh(z.1)); // sin
        // z = (f64::cos(z.0) * f64::cosh(z.1), -f64::sin(z.0) * f64::sinh(z.1)); // cos

        // let phi = f64::atan2(z.1, z.0) * (1.0 / 200.0 + 1.0); // swap args ?
        // let rho = f64::sqrt(z.0*z.0 + z.1*z.1).powf(1.0 / 200.0 + 1.0);
        // z = (rho * f64::cos(phi), rho * f64::sin(phi));
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

fn feather((a, b): Complex) -> Color {
    let mut z = (a, b);
    for i in 0..1000 {
        let t = cdiv(cpow(z, 3), cmul((1.0, 0.0), cpow(z, 2)));
        z.0 = (t.0 + a) / 1.2;
        z.1 = (t.1 + b) / 1.2;
        if f64::sqrt(z.0 * z.0 + z.1 * z.1) > 4.0 {
            return Color::RGB(100 + (i % 155) as u8, 30 + (i % 100) as u8, 0)
        }
    }
    Color::BLACK
}

fn cmul((a, b): Complex, (c, d): Complex) -> Complex {
    (a * c - b * d, a * d + b * c)
}

fn cdiv((a, b): Complex, (c, d): Complex) -> Complex {
   ( (a * c + b * d) / (c * c + d * d)
   , (b * c - a * d) / (c * c + d * d))
}

fn cpow(z: Complex, n: i32) -> Complex {
    let mut t = (z.0, z.1);
    for _ in 1..n {
        t = cmul(t, z)
    }
    t
}

fn newton((a, b): Complex) -> Color {
    // z = ( 2.0 / 3.0 * z.0 + 1.0 / (3.0 * (z.0 * z.0 - z.1 * z.1) + (6.0 * z.0 * z.1) * (6.0 * z.0 * z.1) / (3.0 * (z.0 * z.0 - z.1 * z.1)))
    //     , 2.0 / 3.0 * z.1 - 1.0 / (6.0 * z.0 * z.1 + (3.0 * (z.0 * z.0 - z.1 * z.1)) * (3.0 * (z.0 * z.0 - z.1 * z.1)) / (6.0 * z.0 * z.1)) );

    fn f(z: Complex) -> Complex {
        let t = cpow(z, 8);
        let t2 = cpow(z, 4);
        (t.0 + 15.0 * t2.0 - 16.0, t.1 + 15.0 * t2.1)
    }

    fn df(z: Complex) -> Complex {
        // let t = cmul(z, z);
        // (3.0 * t.0, 3.0 * t.1)  (3.0 * z.0, 3.0 * z.1)
        let t = cpow(z, 7);
        let t2 = cpow(z, 3);
        (8.0 * t.0 + 15.0 * 4.0 * t2.0, 8.0 * t.1 + 15.0 * 4.0 * t2.1)
    }

    // let roots: [(f64, f64, Color); 3] = [ (1.0, 0.0, Color::RED)
    // 	       	      	   	        , (-0.5, f64::sqrt(3.0)/2.0, Color::BLUE)
    // 					, (-0.5, -f64::sqrt(3.0)/2.0, Color::GREEN)];
    let roots = [ (-1.0, 0.0, Color::RGB(100, 10, 100))
                , ( 1.0, 0.0, Color::RGB(130, 130, 130))
                , (0.0, -1.0, Color::RGB(160, 16, 160))
                , (0.0,  1.0, Color::RGB(190, 190, 190))
                , (-1.41421356237310, -1.41421356237310, Color::RGB(70, 100, 100)) // interesting: 1.4142
                , ( 1.41421356237310,  1.41421356237310, Color::RGB(130, 70, 130))
                , ( 1.41421356237310, -1.41421356237310, Color::RGB(160, 160, 70))
                , (-1.41421356237310,  1.41421356237310, Color::RGB(1, 1, 1))
    ];

    let mut z = (a, b);
    for _ in 0..1000 {
        let t = cdiv(f(z), df(z));
        z = (z.0 - t.0, z.1 - t.1);
        for i in 0..roots.len() {
            let (dx, dy) = ((z.0 - roots[i].0).abs(), (z.1 - roots[i].1).abs());
            if dx < 0.00001 && dy < 0.00001 {
               return roots[i].2
            }
        }
    }
    Color::BLACK
}

// TODO: make function more general
fn to_compl_plain(x_min: f64, x_max: f64, y_min: f64, y_max: f64, x: u32, y: u32) -> Complex {
    // ((x as f64 - WINDOW_WIDTH as f64 / 2.) * 3.4 / WINDOW_WIDTH as f64,
    //  (y as f64 * 2. / WINDOW_HEIGHT as f64 - 1.0))
    ((x_min + (x_max - x_min) * (x as f64 / WINDOW_WIDTH as f64)),
     (y_min + (y_max - y_min) * (y as f64 / WINDOW_HEIGHT as f64)))
}

fn to_compl_plain_old(x: u32, y: u32) -> Complex {
    ((x as f64 - WINDOW_WIDTH as f64 / 2.) * 3.4 / WINDOW_WIDTH as f64,
     (y as f64 * 2. / WINDOW_HEIGHT as f64 - 1.0))
}

pub fn main_fractals() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    // TODO: use textures
    // TODO: less noisy picture
    // let cache: Vec<Color> = (0..(WINDOW_HEIGHT * WINDOW_WIDTH)).collect::<Vec<u32>>().par_iter()
    //             .map(|l| newton(to_compl_plain(-1.8, 1.0, -1.0, 1.0, l / WINDOW_HEIGHT, l % WINDOW_HEIGHT))).collect();
    let cache: Vec<Color> = (0..(WINDOW_HEIGHT * WINDOW_WIDTH)).collect::<Vec<u32>>().par_iter()
                .map(|l| feather(to_compl_plain_old(l / WINDOW_HEIGHT, l % WINDOW_HEIGHT))).collect();

    println!("{}", now.elapsed().as_millis());

    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context.video()?
        .window("Fractals", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered().build()?
        .into_canvas().build()?;

    // let texture_creator = canvas.texture_creator();
    // let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 1920, 1080)?;

    // texture.with_lock(None, |buffer: &mut Vec<u8>, pitch: usize| {
    //     for y in 0..256 {
    //         for x in 0..256 {
    //             let offset = y*pitch + x*3;
    //             buffer[offset] = x as u8;
    //             buffer[offset + 1] = y as u8;
    //             buffer[offset + 2] = 0;
    //         }
    //     }
    // })?;

    // canvas.clear();
    // canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
    // canvas.copy_ex(&texture, None,
    //     Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false)?;
    // canvas.present();

    for i in 0..WINDOW_WIDTH*WINDOW_HEIGHT {
        canvas.set_draw_color(cache[i as usize]);
        canvas.draw_point(Point::new((i / WINDOW_HEIGHT) as i32, (i % WINDOW_HEIGHT) as i32))?;
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
        std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
    }

    Ok(())
}
