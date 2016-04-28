extern crate ppm;
extern crate rand;
extern crate vector;
extern crate weekendraytracer;

use std::io;
use ppm::PPMWriter;
use rand::Rng;
use vector::Vector4;

use weekendraytracer::*;

fn main() {
    let width  = 200;
    let height = 100;
    let anti_alias_sample_size = 200;

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

    let lambertian_a = Lambertian::new(Vector4::new3(0.8, 0.3, 0.3));
    let lambertian_b = Lambertian::new(Vector4::new3(0.8, 0.8, 0.0));

    let metal_a = Metal::new(Vector4::new3(0.8, 0.6, 0.2), 0.3);
    let metal_b = Metal::new(Vector4::new3(0.8, 0.8, 0.8), 1.0);

    renderable_list.push(Box::new(Sphere::new(Vector4::new3(0.0, 0.0, -1.0), 0.5, lambertian_a)));
    renderable_list.push(Box::new(Sphere::new(Vector4::new3(0.0, -100.5, -1.0), 100.0, lambertian_b)));
    renderable_list.push(Box::new(Sphere::new(Vector4::new3(1.0, 0.0, -1.0), 0.5, metal_a)));
    renderable_list.push(Box::new(Sphere::new(Vector4::new3(-1.0, 0.0, -1.0), 0.5, metal_b)));

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
                anti_aliased_colour += colour(&ray, &renderable_list[..], 0);
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


// Linearly blends white and blue depending on the y value of the target
// screen coordinate
fn colour(ray: &Ray, renderable_list: &[Box<Renderable>], depth: u32) -> Vector4 {
    if let Some(renderable_intersection) = renderable_list.intersects(ray, 0.01, std::f32::MAX) {

        if depth < 50 {
            if let Some((scattered, attenuation)) = renderable_intersection.material.scatter(ray, &renderable_intersection) {
                return attenuation * colour(&scattered, renderable_list, depth + 1);
            }
        }

        Vector4::new3(0.0, 0.0, 0.0)
    } else {

        // Make unit so (-1.0 < y < 1.0) holds true
        let unit_direction = ray.direction().unit_vector();

        // Scale to 0.0 < t < 1.0
        let t = 0.5 * (unit_direction.y + 1.0);

        // Linear interpolation (lerp) of white (1.0, 1.0, 1.0) and blue (0.5, 0.7, 1.0)
        //   blended_value = (1 - t) * start_value + t * end_value
        (Vector4::new3(1.0, 1.0, 1.0) * (1.0 - t)) + (Vector4::new3(0.5, 0.7, 1.0) * t)
    }
}
