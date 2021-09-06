use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const DDOTS: usize = 30;
const DOTS: usize = 10;
const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_WIDTH: u32 = 1000;

const SQUARE: [[f64; 2]; 4] = [[-1.0, -1.0], [-1.0, 1.0], [1.0, 1.0], [1.0, -1.0]];

fn mul_m_to_v([m0, m1]: [[f64; 2]; 2], [v0, v1]: [f64; 2]) -> [f64; 2] {
    return [m0[0] * v0 + m0[1] * v1, m1[0] * v0 + m1[1] * v1];
}

fn transl(a: [[f64; 2]; DDOTS], [v0, v1]: [f64; 2]) -> [[f64; 2]; DDOTS] {
    // return [
    //     m0[0] * v0 + m0[1] * v1,
    //     m1[0] * v0 + m1[1] * v1
    // ]
    let mut res = [[0.0; 2]; DDOTS];
    for i in 0..DDOTS {
        res[i] = [a[i][0] + 0.5, a[i][1]];
    }
    res
}

fn my_sin() -> [(i32, i32); DOTS] {
    let mut res = [(0, 0); DOTS];
    let h = 3.14 / DOTS as f64;
    let mut s = 0.0;
    for i in 0..DOTS {
        res[i] = (
            450 + f64::round(s * 100.0) as i32,
            300 - f64::round(f64::sin(s) * 100.0) as i32,
        );
        s += h;
    }
    res
}

fn proj(a: [[f64; 3]; 100]) -> [[f64; 2]; 100] {
    let mut res = [[0.0; 2]; 100];
    for i in 0..100 {
        res[i] = [a[i][0] * 3.0 / a[i][2], a[i][1] * 3.0 / a[i][2]]
    }
    res
}

// fn draw_levy(mut can: sdl2::render::Canvas<sdl2::video::Window>, (x1, y1): (f64, f64), (x2, y2): (f64, f64), i: i32) {

//     if i == 0 {
//         can.set_draw_color(Color::BLACK);
//         can.present();
//     } else {
//         let (x3, y3) = (
//             (x1 + x2) / 2.0 + (y2 - y1) / 2.0,
//             (y1 + y2) / 2.0 - (y2 - y1) / 2.0);
//         can.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x3 as i32, y3 as i32));
//         can.draw_line(Point::new(x2 as i32, y2 as i32), Point::new(x3 as i32, y3 as i32));
//         draw_levy(can.copy(), (x1, y1), (x3, y3), i-1);
//         draw_levy(can.copy(), (x3, y3), (x2, y2), i-1);
//     }
// }

fn polar_to_dec(rho: f64, phi: f64) -> (f64, f64) {
    return (f64::cos(phi) * rho, f64::sin(phi) * rho);
}

pub fn main_geometry() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context
        .video()?
        .window("Geometry", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?
        .into_canvas()
        .build()?;

    // let mut s = [[0.0 as f64; 2]; DDOTS];
    // let mut rng = rand::thread_rng();
    // for i in 0..DDOTS {
    //     s[i] = [rng.gen_range(-10.0, 10.0), rng.gen_range(-10.0, 10.0)]
    // }
    // let mut theta = 0.0;
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

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.set_draw_color(Color::BLACK);

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
        // for i in 0..20 {
        // s = transl(s, [0.0, 1.0]);
        // }
        // for i in 0..DDOTS {
        //     let x1 = WINDOW_WIDTH  as f64 / 2.0 + 50.0 * s[i][0];
        //     let y1 = WINDOW_HEIGHT as f64 / 2.0 + 50.0 * s[i][1];
        //     canvas.draw_point(Point::new(x1 as i32, y1 as i32))?;
        // }
        let (x1, y1) = (100, 100);
        let (x2, y2) = (100, 200);
        let (x3, y3) = (150, 150); //x1 + (y2 - y1) / 2

        let z: Vec<(i32, i32)> = Vec::new();
        canvas.draw_line(
            Point::new(x1 as i32, y1 as i32),
            Point::new(x3 as i32, y3 as i32),
        )?;
        canvas.draw_line(
            Point::new(x2 as i32, y2 as i32),
            Point::new(x3 as i32, y3 as i32),
        )?;
        // ------------------------------------------------------------

        canvas.present();

        const FPS: u32 = 10;
        // let dt = 1.0 / FPS as f64;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / FPS));
        // theta += 2.0 * dt;
    }

    Ok(())
}
