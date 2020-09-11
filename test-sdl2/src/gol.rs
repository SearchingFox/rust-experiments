#![allow(dead_code)]
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::error::Error;
use std::time::Duration;
use rand::Rng;

const WINDOW_WIDTH:  u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const BACKGROUND: Color = Color::RGB(255, 255, 255);
const FOREGROUND: Color = Color::RGB(0, 0, 0);

type Field = [[u8; 100]; 100]; // ? maybe add phanton rows and columns

fn gol_step(f: &Field) -> Field {
    let mut result: Field = [[0; 100]; 100];
    for i in 0..100 {
        for j in 0..100 {
            let mut cnt = 0;
            for x in -1..2 {
                for y in -1..2 {
                    if x != 0 || y != 0 { // ? test without checking
                        let xx = i + x as i32;
                        let yy = j + y as i32;
                        if xx > 0 && xx < 100 && yy > 0 && yy < 100 {
                            if f[xx as usize][yy as usize] == 1 {
                                cnt += 1;
                            }
                        }
                    }
                }
            }
            if f[i as usize][j as usize] == 1 {
                if cnt == 2 || cnt == 3 {
                    result[i as usize][j as usize] = 1;
                }
            } else {
                if cnt == 3 {
                    result[i as usize][j as usize] = 1;
                }
            }
        }
    }
    result
}

fn rule30_step(f: &Field) -> Field {
    let mut result: Field = [[0; 100]; 100];
    result[0] = f[0];
    for i in 1..100 {
        for j in 0..100 {
            let a = if j > 0  { f[i-1][j-1] } else { 0 };
            let c = if j < 99 { f[i-1][j+1] } else { 0 };
            result[i][j] = match(a, f[i-1][j], c) {
                (1, 1, 1) => { 0 }
                (1, 1, 0) => { 0 }
                (1, 0, 1) => { 0 }
                (1, 0, 0) => { 1 }
                (0, 1, 1) => { 1 }
                (0, 1, 0) => { 1 }
                (0, 0, 1) => { 1 }
                (0, 0, 0) => { 0 }
                (_, _, _) => { 0 }
            }
        }
    }
    result
}

fn rule22_step(f: &Field) -> Field {
    let mut result: Field = [[0; 100]; 100];
    result[0] = f[0];
    for i in 1..100 {
        for j in 0..100 {
            let a = if j > 0  { f[i-1][j-1] } else { 0 };
            let c = if j < 99 { f[i-1][j+1] } else { 0 };
            result[i][j] = match(a, f[i-1][j], c) {
                (1, 1, 1) => { 0 }
                (1, 1, 0) => { 0 }
                (1, 0, 1) => { 0 }
                (1, 0, 0) => { 1 }
                (0, 1, 1) => { 0 }
                (0, 1, 0) => { 1 }
                (0, 0, 1) => { 1 }
                (0, 0, 0) => { 0 }
                (_, _, _) => { 0 }
            }
        }
    }
    result
}

fn rule110_step(f: &Field) -> Field {
    let mut result: Field = [[0; 100]; 100];
    result[0] = f[0];
    for i in 1..100 {
        for j in 0..100 {
            let a = if j > 0  { f[i-1][j-1] } else { 0 };
            let c = if j < 99 { f[i-1][j+1] } else { 0 };
            result[i][j] = match(a, f[i-1][j], c) {
                (1, 1, 1) => { 0 }
                (1, 1, 0) => { 1 }
                (1, 0, 1) => { 1 }
                (1, 0, 0) => { 0 }
                (0, 1, 1) => { 1 }
                (0, 1, 0) => { 1 }
                (0, 0, 1) => { 1 }
                (0, 0, 0) => { 0 }
                (_, _, _) => { 0 }
            }
        }
    }
    result
}

fn rule30_step_1(f: &Field) -> Field {
    let mut result: Field = [[0; 100]; 100];
    for i in 1..100 {
        for j in 0..100 {
            let a = if j > 0  { f[j-1][i-1] } else {0};
            let c = if j < 99 { f[j+1][i-1] } else {0};
            result[i][j] = match(a, f[j][i-1], c) {
                (1, 1, 1) => { 0 }
                (1, 1, 0) => { 0 }
                (1, 0, 1) => { 0 }
                (1, 0, 0) => { 1 }
                (0, 1, 1) => { 1 }
                (0, 1, 0) => { 1 }
                (0, 0, 1) => { 1 }
                (0, 0, 0) => { 0 }
                (_, _, _) => { 0 }
            }
        }
    }
    result
}

pub fn main_gol() -> Result<(), Box<dyn Error>>{
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
    // glider:
    // field[50][50] = 1;
    // field[50][51] = 1;
    // field[50][52] = 1;
    // field[49][52] = 1;
    // field[48][51] = 1;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut running = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                    Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    if running {
                        running = false;
                    } else {
                        running = true;
                    }
                }
                _ => {}
            }
        }
        // canvas.set_draw_color(BACKGROUND);
        // ! canvas.clear(); why don't I need this?
        // canvas.set_draw_color(FOREGROUND);
        // for i in (0..WINDOW_HEIGHT).step_by(10) {
        //     for j in (0..WINDOW_WIDTH).step_by(10) {
        for i in 0..100 {
            for j in 0..100 {
                let t: Rect = Rect::new(j as i32 * 10, i as i32 * 10, 9, 9);
                if field[i as usize][j as usize] == 1 {
                    canvas.set_draw_color(FOREGROUND);
                } else {
                    canvas.set_draw_color(BACKGROUND);
                }
                canvas.fill_rect(t)?;
                canvas.draw_rect(t)?;
            }
        }
        canvas.present();
        if running {
            field = rule110_step(&field);
        }
        const FPS: u32 = 10;
        let dt = 1.0 / FPS as f64;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        // canvas.clear();
    }
    Ok(())
}