use rand::Rng;
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::error::Error;
use std::time::Duration;

const WINDOW_WIDTH: u32  = 1920;
const WINDOW_HEIGHT: u32 = 1080;

#[derive(Debug, Clone, Copy)]
struct PPoint {
    x: f64,
    y: f64
}

impl Distribution<PPoint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PPoint {
        let (rand_x, rand_y) = rng.gen::<(f64, f64)>();
        PPoint {
            x: rand_x * 2.0 - 1.0,
            y: rand_y * 2.0 - 1.0
        }
    }
}

// impl Distribution<(f64, f64)> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PPoint {
//         let (rand_x, rand_y) = rng.gen::<(f64, f64)>();
//         ( rand_x * 2.0 - 1.0
//         , rand_y * 2.0 - 1.0 )
//     }
// }

// macro_rules! sum {
//     ($($args:expr),*) => {{
//         $(
//             println!("{}", $args);
//         )*
//     }}
// }

fn v0((x, y): (f64, f64)) -> (f64, f64) {
    (2.0*x, 2.0*y)
}

fn v1((x, y): (f64, f64)) -> (f64, f64) {
    (f64::sin(x), f64::sin(y))
}

fn v2((x, y): (f64, f64)) -> (f64, f64) {
    let r = 1.0 / f64::sqrt(x * x + y * y);
    (r * x, r * y)
}

fn f0(a: (f64, f64)) -> (f64, f64) {
    return ( v0(a).0 + v1(a).0 + v2(a).0
           , v0(a).1 + v1(a).1 + v2(a).1 )
}

fn f1(a: (f64, f64)) -> (f64, f64) {
    return ( v0(a).0 + v1(a).0 + v2(a).0
           , v0(a).1 + v1(a).1 + v2(a).1 )
}

fn f2(a: (f64, f64)) -> (f64, f64) {
    return ( v0(a).0 + v1(a).0 + v2(a).0
           , v0(a).1 + v1(a).1 + v2(a).1 )
}



pub fn main_ifs() -> Result<(), Box<dyn Error>> {
    let mut histogram: [[f64; 100]; 100];
    let functions = [f0, f1, f2];
    let mut rng = rand::thread_rng();
    let mut t: (f64, f64) = rng.gen();
    println!("{:?}", t);

    for i in 0..20 {
        // rng = rand::thread_rng();
        t = (functions.choose(&mut rng).unwrap())(t);
    }
    println!("{:?}", t);

    // println!("{:?}", (functions.choose(&mut rng).unwrap())((1.0, 1.0)));
    Ok(())
}