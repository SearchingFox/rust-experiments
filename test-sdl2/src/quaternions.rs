use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const CUBE: [[f64; 3]; 8] = [
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [-1.0, 1.0, 1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, -1.0, 1.0],
];
const SQUARE: [[f64; 2]; 4] = [[-1.0, -1.0], [-1.0, 1.0], [1.0, 1.0], [1.0, -1.0]];

fn mul_m2_to_v2([[m0, m1], [m2, m3]]: [[f64; 2]; 2], [v0, v1]: [f64; 2]) -> [f64; 2] {
    return [m0 * v0 + m1 * v1, m2 * v0 + m3 * v1];
}

fn mul_m3_to_v3(
    [[m0, m1, m2], [m3, m4, m5], [m6, m7, m8]]: [[f64; 3]; 3],
    [v0, v1, v2]: [f64; 3],
) -> [f64; 3] {
    return [
        m0 * v0 + m1 * v1 + m2 * v2,
        m3 * v0 + m4 * v1 + m5 * v2,
        m6 * v0 + m7 * v1 + m8 * v2,
    ];
}

fn render() {}

pub fn main_quaternions() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context
        .video()?
        .window("Quaternions", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?
        .into_canvas()
        .build()?;

    let mut theta = 0.0;
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

        theta += 0.1;

        let rot_x: [[f64; 3]; 3] = [
            [1.0, 0.0, 0.0],
            [0.0, f64::cos(theta), -f64::sin(theta)],
            [0.0, f64::sin(theta), f64::cos(theta)],
        ];

        let rot_y: [[f64; 3]; 3] = [
            [f64::cos(theta), 0.0, f64::sin(theta)],
            [0.0, 1.0, 0.0],
            [-f64::sin(theta), 0.0, f64::cos(theta)],
        ];

        for i in 0..8_usize {
            let [x1n, y1n, z1n] = mul_m3_to_v3(rot_y, mul_m3_to_v3(rot_x, CUBE[i]));
            let [x2n, y2n, z2n] = mul_m3_to_v3(rot_y, mul_m3_to_v3(rot_x, CUBE[(i + 1) % 8]));

            let x1 = WINDOW_WIDTH as f64 / 2.0 + 50.0 * (x1n / z1n);
            let y1 = WINDOW_HEIGHT as f64 / 2.0 + 50.0 * (y1n / z1n);
            let x2 = WINDOW_WIDTH as f64 / 2.0 + 50.0 * (x2n / z2n);
            let y2 = WINDOW_HEIGHT as f64 / 2.0 + 50.0 * (y2n / z1n);

            canvas.draw_line(
                Point::new(x1 as i32, y1 as i32),
                Point::new(x2 as i32, y2 as i32),
            )?;
        }

        canvas.present();

        const FPS: u32 = 10;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
