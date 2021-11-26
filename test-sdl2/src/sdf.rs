use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::ops;

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
            y: 0.0,
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
            // let I_p = k_a * i_a + (k_d * Ln_m.dot(Nn) * i_d + k_s * Rn_m.dot(Vn).)
            ret = Color::RED;
            break;
        }
        depth += d;
    }

    return ret;
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

    for y in 0..WINDOW_HEIGHT {
        for x in 0..WINDOW_WIDTH {
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
            canvas.set_draw_color(ray_march(camera_pos, ray_dir));
            canvas.draw_point(Point::new(x as i32, y as i32))?;
        }
    }

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
