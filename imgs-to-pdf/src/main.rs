use printpdf::{image, CustomPdfConformance, Image, Mm, PdfConformance, PdfDocument};
use std::ffi::OsString;
use std::fs::{read_dir, DirEntry, File};
use std::io::{stdin, BufWriter, Error};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: calculate dimensions
    const X: f64 = 162.0;
    const Y: f64 = 92.0;

    let args = std::env::args();
    let mut dir_path: String = String::new();
    if args.len() > 1 {
        dir_path = args.skip(1).next().unwrap();
    } else {
        stdin().read_line(&mut dir_path)?;
        dir_path = dir_path.trim_end().to_string();
    }

    let (mut doc, mut page, mut layer) = PdfDocument::new("_", Mm(X), Mm(Y), "_");
    let start = Instant::now();

    let dir: Vec<std::result::Result<DirEntry, Error>> = read_dir(Path::new(&dir_path))?.collect();
    let mut files_number = dir.len();
    for path in dir {
        files_number -= 1;
        let file = path?.path();

        if file.extension() == Some(&OsString::from("jpg"))
            || file.extension() == Some(&OsString::from("png"))
        {
            let img = Image::from_dynamic_image(&image::open(&file)?.resize(
                1920,
                1080,
                image::imageops::Lanczos3,
            ));

            // https://docs.rs/printpdf/0.4.0/printpdf/types/plugins/graphics/two_dimensional/image/struct.Image.html#method.add_to_layer
            img.add_to_layer(
                doc.get_page(page).get_layer(layer),
                None,
                None,
                None,
                None,
                None,
                None,
            );
            if files_number > 0 {
                let pl = doc.add_page(Mm(X), Mm(Y), "_");
                page = pl.0;
                layer = pl.1;
            }
        } else {
            println!("{} is not a jpg or png file", file.to_str().unwrap());
        }
    }

    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
        ..Default::default()
    }));
    doc.save(&mut BufWriter::new(File::create(
        Path::new(&dir_path).join(Path::new("out_rust.pdf")),
    )?))?;
    println!("Elapsed: {} s", start.elapsed().as_secs());

    Ok(())
}
