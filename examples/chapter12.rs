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
    let width  = 400;
    let height = 200;
    let anti_alias_sample_size = 100;

    let image_data = generate_image_data(width, height, anti_alias_sample_size);

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut writer = PPMWriter::new(&mut out);
    writer.write(&image_data[..], width, height).unwrap();
}

fn random_scene() -> Vec<Box<Renderable>> {
    let mut renderable_list: Vec<Box<Renderable>> = Vec::new();

    renderable_list.push(Box::new(Sphere::new(Vector4::new3(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Vector4::new3(0.5, 0.5, 0.5)))));

    for a in -11..11 {
        let f_a = a as f32;
        for b in -11..11 {
            let f_b = b as f32;

            let choose_mat = rand::random::<f32>();
            let center = Vector4::new3(
                f_a + 0.9 * rand::random::<f32>(),
                0.2,
                f_b + 0.9 * rand::random::<f32>()
            );

            if (center - Vector4::new3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { //diffuse
                    let mat = Lambertian::new(
                        Vector4::new3(
                            rand::random::<f32>(),
                            rand::random::<f32>(),
                            rand::random::<f32>()
                        )
                    );

                    renderable_list.push(Box::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 { // metal
                    let mat = Metal::new(
                        Vector4::new3(
                            1.0 + rand::random::<f32>(),
                            1.0 + rand::random::<f32>(),
                            1.0 + rand::random::<f32>()
                        ) * 0.5,
                        rand::random::<f32>() * 0.5
                    );
                    renderable_list.push(Box::new(Sphere::new(center, 0.2, mat)));
                } else { // glass
                    let mat = Dielectric::new(1.5);
                    renderable_list.push(Box::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let a = Sphere::new(Vector4::new3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5));
    let b = Sphere::new(Vector4::new3(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Vector4::new3(0.4, 0.2, 0.1)));
    let c = Sphere::new(Vector4::new3(4.0, 1.0, 0.0), 1.0, Metal::new(Vector4::new3(0.7, 0.6, 0.5), 0.0));

    renderable_list.push(Box::new(a));
    renderable_list.push(Box::new(b));
    renderable_list.push(Box::new(c));

    renderable_list
}

fn generate_image_data(width: usize, height: usize, anti_alias_sample_size: usize) -> Vec<f32> {
    let mut image_data = Vec::new();

    let look_from = Vector4::new3(20.0 * 0.47_f32.cos(), 20.0 * 0.47_f32, 3.0);
    let look_at   = Vector4::new3(0.0, 0.0, 1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.3;

    let camera = Camera::new(look_from, look_at,
                    Vector4::new3(0.0, 1.0, 0.0), 20.0,
                    width as f32 / height as f32, aperture,
                    dist_to_focus);

    let renderable_list = random_scene();

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
    if let Some(renderable_intersection) = renderable_list.intersects(ray, 0.001, std::f32::MAX) {

        if depth <= 50 {
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
