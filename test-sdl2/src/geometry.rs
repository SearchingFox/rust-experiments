#![allow(dead_code)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use std::error::Error;
use rand::Rng;

const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 600;
// const SQUARE_SIZE: f64 = 10.0;
const BACKGROUND: Color = Color::RGB(255, 255, 255); // Color::RGB(18, 18, 18);
const FOREGROUND: Color = Color::RGB(0, 0, 0); // Color::RGB(255, 150, 150);

const SQUARE: [[f64; 2]; 4] = [
    [-1.0, -1.0],
    [-1.0,  1.0],
    [ 1.0,  1.0],
    [ 1.0, -1.0]
];

fn mul_m_to_v([m0, m1]: [[f64; 2]; 2], [v0, v1]: [f64; 2]) -> [f64; 2] {
    return [
        m0[0] * v0 + m0[1] * v1,
        m1[0] * v0 + m1[1] * v1
    ]
}

const DOTS: usize = 10;

fn my_sin() -> [(i32, i32); DOTS] {
    let mut res = [(0,0); DOTS];
    let h = 3.14 / DOTS as f64;
    let mut s = 0.0;
    for i in 0..DOTS {
        res[i] = (450 + f64::round(s*100.0) as i32,
                  300 - f64::round(f64::sin(s)*100.0) as i32);
        s += h;
    }
    res
}

pub fn main_geometry() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    // let video_subsystem = ;
    let window = sdl_context.video()?.window("test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    // canvas.clear();
    // canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    // let mut sz = 0;
    // let q = my_sin();
    // println!("{:?}", q);
    // let mut theta = 0.0;
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

        canvas.set_draw_color(BACKGROUND);
        canvas.clear();
        canvas.set_draw_color(FOREGROUND);

        // i = (i+1) % 255;
        // sz = (sz as f64 + 0.1) as i32 % 10;
        // theta += 0.01;
        // sx += 1.0;
        // sy += 1.0;

        // let t1: Point = Point::new(410, 310);
        // Color::RGB(i, 64, 255-i)
        // canvas.draw_line(t, t1).unwrap();
        // ------------------------------------------------------------
        // for i in 0..4 {
        //     let [x1n, y1n] = mul_m_to_v([[f64::cos(theta),  f64::sin(theta)],
        //                                  [f64::sin(theta), -f64::cos(theta)]], SQUARE[i as usize]);
        //     let [x2n, y2n] = mul_m_to_v([[f64::cos(theta),  f64::sin(theta)],
        //                                  [f64::sin(theta), -f64::cos(theta)]], SQUARE[((i+1) % 4) as usize]);
        //     let x1 = WINDOW_WIDTH  as f64 / 2.0 + 50.0 * x1n;
        //     let y1 = WINDOW_HEIGHT as f64 / 2.0 + 50.0 * y1n;
        //     let x2 = WINDOW_WIDTH  as f64 / 2.0 + 50.0 * x2n;
        //     let y2 = WINDOW_HEIGHT as f64 / 2.0 + 50.0 * y2n;

        //     canvas.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32))?;
        // }
        // ------------------------------------------------------------
        // let mut rng = rand::thread_rng();
        // for _ in 0..100 {
        //     let x1 = rng.gen_range(10, WINDOW_WIDTH);
        //     let y1 = rng.gen_range(10, WINDOW_HEIGHT);
        //     let x2 = rng.gen_range(10, WINDOW_WIDTH);
        //     let y2 = rng.gen_range(10, WINDOW_HEIGHT);

        //     canvas.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32))?;
        // }
        // ------------------------------------------------------------
        // for i in 0..q.len()-1 {
        //     canvas.draw_line(Point::new(q[i].0 as i32, q[i].1 as i32),
        //                      Point::new(q[i+1].0 as i32, q[i+1].1 as i32))?
        // }
        // ------------------------------------------------------------
        canvas.present();

        const FPS: u32 = 2;
        let dt = 1.0 / FPS as f64;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        theta += 2.0 * dt;
    }

    Ok(())
}