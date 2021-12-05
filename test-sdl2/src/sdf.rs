use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::ops;
use rayon::prelude::*;

const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_WIDTH: u32 = 1000;
type Distance = f64;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Vec3 {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

struct Sphere {
    position: Vec3,
    radius: f64,
}

impl Sphere {
    fn sdf(&self, point: Vec3) -> Distance {
        (point - self.position).length() - self.radius
    }
}

// struct Torus {
//     fn sdf(p: Vec3, t: (f64, f64)) -> Distance {


fn signed_dst_to_scene(p: Vec3) -> Distance {
    let a = Sphere {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -10.0,
        },
        radius: 2.0,
    };
    let b = Sphere {
        position: Vec3 {
            x: 5.0,
            y: 1.0,
            z: -10.0,
        },
        radius: 1.0,
    };
    vec![a, b]
        .iter()
        .map(|x| x.sdf(p))
        .fold(f64::MAX, |a, b| a.min(b))
}

fn ray_march(p: Vec3, direction: Vec3) -> Color {
    let mut ret = Color::BLACK;
    let max_step = 64;
    let mut depth = 0.0;

    for i in 0..max_step {
        let d = signed_dst_to_scene(p + direction * depth);
        if d < 0.001 {
            // let I_p = k_a * i_a + (k_d * Ln_m.dot(Nn) * i_d + k_s * Rn_m.dot(Vn)
            let light = Vec3 {
                x: 1.0,
                y: 10.0,
                z: -7.0
            };
            let dt = 1e-3;
            let xx = Vec3 { x: p.x + dt, y: p.y, z: p.z };
            let yy = Vec3 { x: p.x, y: p.y + dt, z: p.z };
            let zz = Vec3 { x: p.x, y: p.y, z: p.z + dt };
            let xx2 = Vec3 { x: p.x - dt, y: p.y, z: p.z };
            let yy2 = Vec3 { x: p.x, y: p.y - dt, z: p.z };
            let zz2 = Vec3 { x: p.x, y: p.y, z: p.z - dt };
            // let normal = Vec3 { x: (signed_dst_to_scene(xx) - d - p.x) / dt, y: (signed_dst_to_scene(yy) - d - p.y) / dt, z: (signed_dst_to_scene(zz) - d - p.z) / dt };
            let normal = Vec3 { x: signed_dst_to_scene(xx) - signed_dst_to_scene(xx2),
                                y: signed_dst_to_scene(yy) - signed_dst_to_scene(yy2),
                                z: signed_dst_to_scene(zz) - signed_dst_to_scene(zz2),
            }.normalize();
            //if normal.length() < 1e-9 {
            //    return Color::BLACK;
            //}
            let diffuse = normal.dot(light * (-1.0)); // light.dot(normal);
            ret = Color::RGB(((diffuse * 10.0) as u32 % 255) as u8, (diffuse) as u8, (diffuse) as u8);
            break;
        }
        depth += d;
    }

    ret
}

fn shade(p: Vec3) -> Vec3 {
    let l = Vec3 {
        x: 50.0,
        y: 20.0,
        z: 50.0
    };
    l.normalize()
}

pub fn main_sdf() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context
        .video()?
        .window("Geometry", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?
        .into_canvas()
        .build()?;

    let cache: Vec<Color> = (0..WINDOW_HEIGHT*WINDOW_WIDTH).into_par_iter().map(|i| {
            let x = i % WINDOW_WIDTH;
            let y = i / WINDOW_WIDTH;
            let x1 = (x as f64 / WINDOW_WIDTH as f64 * 2.0 - 1.0) * WINDOW_WIDTH as f64
                / WINDOW_HEIGHT as f64;
            let y1 = y as f64 / WINDOW_HEIGHT as f64 * 2.0 - 1.0;
            let camera_pos = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 3.0,
            };
            let pixel_pos = Vec3 {
                x: x1,
                y: y1,
                z: 2.0,
            };
            let ray_dir = (pixel_pos - camera_pos).normalize();
            ray_march(camera_pos, ray_dir)
    }).collect();
    for (j, i) in cache.iter().enumerate() {
        let x = j as u32 % WINDOW_WIDTH;
        let y = j as u32/ WINDOW_WIDTH;
        canvas.set_draw_color(*i);
        canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
    }
    println!("RENDERING FINISHED");
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}
