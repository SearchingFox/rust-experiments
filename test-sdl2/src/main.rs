#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod fractals;
mod geometry;
mod gol;
mod ifs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fractals::main_fractals()
    // geometry::main_geometry()
    // gol::main_gol()
    // ifs::main_ifs()
}
