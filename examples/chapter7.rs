extern crate ppm;
extern crate rand;
extern crate vector;
extern crate weekendraytracer;

use std::io;
use ppm::PPMWriter;
use rand::Rng;
use vector::Vector4;

use weekendraytracer::{ Camera, Sphere, Ray, Renderable };

fn main() {
    let width  = 200;
    let height = 100;
    let anti_alias_sample_size = 100;

    let image_data = generate_image_data(width, height, anti_alias_sample_size);

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut writer = PPMWriter::new(&mut out);
    writer.write(&image_data[..], width, height).unwrap();
}

fn generate_image_data(width: usize, height: usize, anti_alias_sample_size: usize) -> Vec<f32> {
    let mut image_data = Vec::new();

    let camera = Camera::new();
    let mut renderable_list: Vec<Box<Renderable>> = Vec::new();

    renderable_list.push(Box::new(Sphere::new(Vector4::new3(0.0, 0.0, -1.0), 0.5)));
    renderable_list.push(Box::new(Sphere::new(Vector4::new3(0.0, -100.5, -1.0), 100.0)));

    let mut rng = rand::thread_rng();

    // Traverses the screen space from the left to right, starting at
    // the bottom left
    for y in (0..height).rev() {
        for x in 0..width {
            let mut anti_aliased_colour = Vector4::new3(0.0, 0.0, 0.0);
            for _ in 0..anti_alias_sample_size {
                let u = (x as f32 + rng.next_f32()) / width as f32;
                let v = (y as f32 + rng.next_f32()) / height as f32;

                let ray = camera.get_ray(u, v);
                anti_aliased_colour += colour(&ray, &renderable_list[..]);
            }

            let col = anti_aliased_colour / (anti_alias_sample_size as f32);

            let r = col.x;
            let g = col.y;
            let b = col.z;
            image_data.push(r.sqrt());
            image_data.push(g.sqrt());
            image_data.push(b.sqrt());
        }
    }

    image_data
}

fn random_in_unit_sphere() -> Vector4 {
    loop {
        let p = (Vector4::new3(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>()
        ) * 2.0) - Vector4::new3(1.0, 1.0, 1.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

// Linearly blends white and blue depending on the y value of the target
// screen coordinate
fn colour(ray: &Ray, renderable_list: &[Box<Renderable>]) -> Vector4 {
    if let Some(renderable_intersection) = renderable_list.intersects(ray, 0.01, std::f32::MAX) {
        let target = renderable_intersection.intersection_point + renderable_intersection.normal + random_in_unit_sphere();
        return colour(&Ray::new(renderable_intersection.intersection_point, target - renderable_intersection.intersection_point), renderable_list) * 0.5;
    }

    // Make unit so (-1.0 < y < 1.0) holds true
    let unit_direction = ray.direction().unit_vector();

    // Scale to 0.0 < t < 1.0
    let t = 0.5 * (unit_direction.y + 1.0);

    // Linear interpolation (lerp) of white (1.0, 1.0, 1.0) and blue (0.5, 0.7, 1.0)
    //   blended_value = (1 - t) * start_value + t * end_value
    (Vector4::new3(1.0, 1.0, 1.0) * (1.0 - t)) + (Vector4::new3(0.5, 0.7, 1.0) * t)
}

