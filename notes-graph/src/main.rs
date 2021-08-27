use rand::Rng;
use regex::Regex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const BACKGROUND_COLOR: Color = Color::RGB(45, 45, 45);
const FOREGROUND_COLOR: Color = Color::RGB(194, 194, 194);
const SIZE: u32 = 16;

fn get_notes(folder_path: &str) -> (HashMap<String, Rect>, HashMap<String, Vec<String>>) {
    let mut rng = rand::thread_rng();
    let mut note_point = HashMap::new();
    let mut note_notes = HashMap::new();

    for f in std::fs::read_dir(folder_path).unwrap() {
        let file = f.unwrap();
        let os_str = file.file_name();
        let file_name = os_str.to_str().unwrap();

        note_point.insert(
            "[[".to_string() + &file_name[0..file_name.len() - 3] + "]]",
            Rect::new(rng.gen_range(10..1900), rng.gen_range(10..1040), SIZE, SIZE),
        );
        note_notes.insert(
            "[[".to_string() + &file_name[0..file_name.len() - 3] + "]]",
            Regex::new(r"\[\[.*\]\]")
                .unwrap()
                .find_iter(&std::fs::read_to_string(file.path()).unwrap())
                .map(|x| x.as_str().to_string())
                .collect::<Vec<_>>(),
        );
    }

    (note_point, note_notes)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let mut canvas = sdl_context
        .video()?
        .window("ObsVis", WINDOW_WIDTH, WINDOW_HEIGHT)
        .maximized()
        .build()?
        .into_canvas()
        .build()?;
    let mut event_pump = sdl_context.event_pump()?;

    let (mut note_point, note_notes) = get_notes("D:/ObsLogseq/Data");
    let mut hit_node_name = "".to_string();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    for (n, p) in &note_point {
                        if p.contains_point(Point::new(x, y)) {
                            hit_node_name = n.to_string();
                        }
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if !hit_node_name.is_empty() {
                        hit_node_name.clear();
                    }
                }
                Event::MouseMotion {
                    mousestate, x, y, ..
                } => {
                    if mousestate.left() && !hit_node_name.is_empty() {
                        note_point
                            .entry(hit_node_name.to_string())
                            .or_insert(Rect::new(0, 0, 0, 0))
                            .center_on(Point::new(x, y));
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        canvas.set_draw_color(FOREGROUND_COLOR);

        canvas.draw_rects(&note_point.values().cloned().collect::<Vec<_>>())?;
        for (note, notes) in &note_notes {
            for point in notes {
                match note_point.get(point) {
                    Some(end_point) => {
                        canvas.draw_line(note_point[note].center(), end_point.center())?;
                    }
                    None => {}
                }
            }
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 120));
    }

    Ok(())
}
