#![allow(dead_code)]

mod geometry;
mod fractals;
mod gol;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // fractals::main_fractals()
    // gol::main_gol()
    geometry::main_geometry()
}
