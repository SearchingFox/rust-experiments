use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_WIDTH: u32 = 1000;

type Field = [[u8; 100]; 100];

fn gol_step(f: &Field) -> Field {
    let mut result = [[0; 100]; 100];
    for i in 1..99 {
        for j in 1..99 {
            let alive = f[i - 1][j - 1]
                + f[i - 1][j]
                + f[i - 1][j + 1]
                + f[i][j - 1]
                + f[i][j + 1]
                + f[i + 1][j - 1]
                + f[i + 1][j]
                + f[i + 1][j + 1];

            if f[i as usize][j as usize] == 1 {
                if alive == 2 || alive == 3 {
                    result[i as usize][j as usize] = 1;
                }
            } else {
                if alive == 3 {
                    result[i as usize][j as usize] = 1;
                }
            }
        }
    }
    result
}

fn rules_step(f: &Field, rule: u8) -> Field {
    let mut result: Field = [[0; 100]; 100];
    result[0] = f[0];
    for i in 1..100 {
        for j in 0..100 {
            let a = if j > 0 { f[i - 1][j - 1] } else { 0 };
            let c = if j < 99 { f[i - 1][j + 1] } else { 0 };
            if rule == 22 {
                result[i][j] = match (a, f[i - 1][j], c) {
                    (1, 1, 1) => 0,
                    (1, 1, 0) => 0,
                    (1, 0, 1) => 0,
                    (1, 0, 0) => 1,
                    (0, 1, 1) => 0,
                    (0, 1, 0) => 1,
                    (0, 0, 1) => 1,
                    (0, 0, 0) => 0,
                    _ => 0,
                }
            } else if rule == 30 {
                result[i][j] = match (a, f[i - 1][j], c) {
                    (1, 1, 1) => 0,
                    (1, 1, 0) => 0,
                    (1, 0, 1) => 0,
                    (1, 0, 0) => 1,
                    (0, 1, 1) => 1,
                    (0, 1, 0) => 1,
                    (0, 0, 1) => 1,
                    (0, 0, 0) => 0,
                    _ => 0,
                }
            } else if rule == 110 {
                result[i][j] = match (a, f[i - 1][j], c) {
                    (1, 1, 1) => 0,
                    (1, 1, 0) => 1,
                    (1, 0, 1) => 1,
                    (1, 0, 0) => 0,
                    (0, 1, 1) => 1,
                    (0, 1, 0) => 1,
                    (0, 0, 1) => 1,
                    (0, 0, 0) => 0,
                    (_, _, _) => 0,
                }
            }
        }
    }
    result
}

// fn rule30_step_1(f: &Field) -> Field {
//     let mut result: Field = [[0; 100]; 100];
//     for i in 1..100 {
//         for j in 0..100 {
//             let a = if j > 0  { f[j-1][i-1] } else { 0 };
//             let c = if j < 99 { f[j+1][i-1] } else { 0 };
//             result[i][j] = match(a, f[j][i-1], c) {
//                 (1, 1, 1) => { 0 }
//                 (1, 1, 0) => { 0 }
//                 (1, 0, 1) => { 0 }
//                 (1, 0, 0) => { 1 }
//                 (0, 1, 1) => { 1 }
//                 (0, 1, 0) => { 1 }
//                 (0, 0, 1) => { 1 }
//                 (0, 0, 0) => { 0 }
//                 _         => { 0 }
//             }
//         }
//     }
//     result
// }

pub fn main_gol() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context
        .video()?
        .window("Cellular automata", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?
        .into_canvas()
        .build()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut field: Field = [[0; 100]; 100];
    field[0][50] = 1;
    // field[5][30] = 1;
    // for i in 1..100 {
    //     for j in 0..100 {
    //         field[i][j] = 1;
    //     }
    // }
    // field = rule30_step(&field);
    // for i in 0..2 {
    //     for j in 0..100 {
    //         print!("{}", field[i][j]);
    //     }
    //     print!("\n");
    // }
    // println!("{:?}", field);
    // std::process::exit(0);
    // WOW!!!
    // for i in 0..100 {
    //     for j in 0..100 {
    //         field[j][i] = ((i+j) % 2) as u8;
    //     }
    // }
    // Random
    // let mut rng = rand::thread_rng();
    // for i in 0..100 {
    //     for j in 0..100 {
    //         field[j][i] = rng.gen_range(0, 2);
    //     }
    // }
    // Glider
    // field[50][50] = 1;
    // field[50][51] = 1;
    // field[50][52] = 1;
    // field[49][52] = 1;
    // field[48][51] = 1;

    let mut is_running = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => is_running = !is_running,
                _ => {}
            }
        }

        // for i in (0..WINDOW_HEIGHT).step_by(10) {
        //     for j in (0..WINDOW_WIDTH).step_by(10) {
        for i in 0..100 {
            for j in 0..100 {
                let t: Rect = Rect::new(j as i32 * 10, i as i32 * 10, 9, 9);
                if field[i as usize][j as usize] == 1 {
                    canvas.set_draw_color(Color::BLACK);
                } else {
                    canvas.set_draw_color(Color::WHITE);
                }
                canvas.fill_rect(t)?;
            }
        }
        canvas.present();

        if is_running {
            field = rules_step(&field, 30);
        }

        const FPS: u32 = 12;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / FPS));
    }

    Ok(())
}
