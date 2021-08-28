use rand::Rng;
use regex::Regex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const BACKGROUND_COLOR: Color = Color::RGB(45, 45, 45);
const FOREGROUND_COLOR: Color = Color::RGB(200, 200, 200);
const SIZE: u32 = 18;

fn get_notes(folder_path: &str) -> (Vec<String>, Vec<Rect>, Vec<Vec<String>>) {
    let mut rng = rand::thread_rng();
    let mut names = Vec::new();
    let mut rects = Vec::new();
    let mut links = Vec::new();

    for f in std::fs::read_dir(folder_path).unwrap().take(70) {
        let file_path = f.unwrap().path();
        let name = file_path.file_stem().unwrap().to_str().unwrap().to_string();

        names.push(name);
        rects.push(Rect::new(
            rng.gen_range(10..1900),
            rng.gen_range(10..1040),
            SIZE,
            SIZE,
        ));
        links.push(
            Regex::new(r"\[\[.*\]\]")
                .unwrap()
                .find_iter(&std::fs::read_to_string(file_path).unwrap())
                .map(|x| {
                    let t = x.as_str();
                    t[2..t.len() - 2].to_string()
                })
                .collect::<Vec<_>>(),
        );
    }

    (names, rects, links)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = sdl_context
        .video()?
        .window("ObsVis", WINDOW_WIDTH, WINDOW_HEIGHT)
        .maximized()
        .vulkan()
        .build()?
        .into_canvas()
        .build()?;
    let texture_creator = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init()?;
    let font = ttf_context.load_font("FiraCode-Regular.ttf", 24)?;

    let mut hit_node: Option<usize> = None;
    let (names, mut rects, links) = get_notes("Data");
    let textures = names
        .iter()
        .map(|name| {
            texture_creator
                .create_texture_from_surface(&font.render(&name).blended(FOREGROUND_COLOR).unwrap())
                .unwrap() // blended is slow but nice looking
        })
        .collect::<Vec<_>>();

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
                    hit_node = rects
                        .iter()
                        .position(|rect| rect.contains_point(Point::new(x, y)));
                }
                Event::MouseMotion {
                    mousestate, x, y, ..
                } => {
                    if mousestate.left() {
                        match hit_node {
                            Some(i) => rects[i].center_on(Point::new(x, y)),
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        canvas.set_draw_color(FOREGROUND_COLOR);

        // Nodes
        for i in 0..names.len() {
            canvas.fill_rect(Some(rects[i]))?;

            let sdl2::render::TextureQuery { width, height, .. } = textures[i].query();
            canvas.copy(
                &textures[i],
                None,
                Some(Rect::from_center(
                    rects[i].center().offset(0, -28),
                    width,
                    height,
                )),
            )?;
        }

        // Edges
        for i in 0..names.len() {
            for link_name in &links[i] {
                match names.iter().position(|name| name == link_name) {
                    Some(end) => canvas.draw_line(rects[i].center(), rects[end].center())?,
                    None => {}
                }
            }
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 120));
    }

    Ok(())
}
