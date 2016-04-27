extern crate ppm;
use std::io;
use ppm::PPMWriter;

fn main() {
    let width  = 200;
    let height = 100;

    let image_data = generate_image_data(width, height);

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut writer = PPMWriter::new(&mut out);
    writer.write(&image_data[..], width, height).unwrap();
}

fn generate_image_data(width: usize, height: usize) -> Vec<f32> {
    let mut image_data = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.2;

            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
        }
    }

    image_data
}
