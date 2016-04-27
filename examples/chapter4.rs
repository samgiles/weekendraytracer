extern crate ppm;
extern crate vector;
extern crate weekendraytracer;

use std::io;
use ppm::PPMWriter;
use vector::Vector4;

use weekendraytracer::Ray;

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

    let lower_left_corner = Vector4::new3(-2.0, -1.0, -1.0);
    let horizontal = Vector4::new3(4.0, 0.0, 0.0);
    let vertical   = Vector4::new3(0.0, 2.0, 0.0);
    let origin     = Vector4::new3(0.0, 0.0, 0.0);

    // Traverses the screen space from the left to right, starting at
    // the bottom left
    for y in (0..height).rev() {
        for x in 0..width {
            // The 2 dimensional point (u, v) indicates the offset, or the approximate
            // target pixel on the screen
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(origin, lower_left_corner + (horizontal * u) + (vertical * v));
            let col = colour(&ray);

            let r = col.x;
            let g = col.y;
            let b = col.z;
            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
        }
    }

    image_data
}

// Detects whether a Ray hits a sphere where the sphere is defined by its
// center in three dimensions, and its radius:
//
//  A sphere with center: (x0, y0, z0) with radius r is the locus of all points
//  (x, y, z) such that:
//
//  (x - x0) * (x - x0) + (y - y0) * (y - y0) + (z - z0) * (z - z0) = r * r
//
//  In vector terms any point can be tested
//  easily using the dot product:
//
//  p = ( x,  y,  z) // arbitrary point
//  C = (x0, y0, z0) // center of sphere
//
//  (p - C) ● (p - C) = r * r
//
// So, given a ray of the form
//
//  p(t) = O + t * D
//
// where o is the origin, and d is the direction we want to find a value of
// t where this is true:
//
//  (p(t) - C) ● (p(t) - C) = r * r
//
// expanded:
//  ((O + t * D) - C) ●  ((O + t * D) - C) = r * r
//
//  t * t * (D ● D) + 2 * t * (D ● (O - C)) + (O - C ● O - C) - r * r = 0
//
// The only unknown is t here so the equation is quadratic:
//
//  A = D ● D
//  B = 2 * D ● (O - C)
//  C = (O - C) ● (O - C) - r * r
//   At^2 + Bt + C = 0
//
// Solve for t using the quadratic formula:
//  (-B ± sqrt(B^2 - 4AC)) / 2A
//
// but we only need the discriminant to determine what type of solution the
// the equation has to determine intersection:
//
//   b * b - 4 * a * c
//
fn hit_sphere(center: Vector4, radius: f32, ray: &Ray) -> bool {

    let o_minus_c = ray.origin() - center;

    let a = ray.direction().dot3(ray.direction());
    let b = ray.direction().dot3(o_minus_c) * 2.0;
    let c = (o_minus_c).dot3(o_minus_c) - radius * radius;

    let discriminant = b * b - a * c * 4.0;
    return discriminant > 0.0;
}

// Linearly blends white and blue depending on the y value of the target
// screen coordinate
fn colour(ray: &Ray) -> Vector4 {
    if hit_sphere(Vector4::new3(0.0, 0.0, -1.0), 0.5, ray) {
        return Vector4::new3(1.0, 0.0, 0.0);
    }

    // Make unit so (-1.0 < y < 1.0) holds true
    let unit_direction = ray.direction().unit_vector();

    // Scale to 0.0 < t < 1.0
    let t = 0.5 * (unit_direction.y + 1.0);

    // Linear interpolation (lerp) of white (1.0, 1.0, 1.0) and blue (0.5, 0.7, 1.0)
    //   blended_value = (1 - t) * start_value + t * end_value
    (Vector4::new3(1.0, 1.0, 1.0) * (1.0 - t)) + (Vector4::new3(0.5, 0.7, 1.0) * t)
}
