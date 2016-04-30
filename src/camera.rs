use vector::Vector4;
use ray::Ray;
use std::f32::consts::PI;
use rand;

fn random_in_unit_disk() -> Vector4 {
    loop {
        let p =
            Vector4::new3(rand::random::<f32>(), rand::random::<f32>(), 0.0)  * 2.0 - Vector4::new3(1.0, 1.0, 0.0);
        if p.dot3(p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vector4,
    lower_left: Vector4,
    horizontal: Vector4,
    vertical: Vector4,

    u: Vector4,
    v: Vector4,
    lens_radius: f32,
}

impl Camera {
    pub fn new(look_from: Vector4, look_at: Vector4,
               up: Vector4, vertical_fov: f32, aspect: f32,
               aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width  = aspect * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            lower_left: look_from - (u * half_width * focus_dist)  -  (v * half_height * focus_dist) - w * focus_dist,
            horizontal: u * half_width * focus_dist * 2.0,
            vertical:  v * half_height * focus_dist * 2.0,
            u: u,
            v: v,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
                 self.lower_left + (self.horizontal * u) + (self.vertical * v) - self.origin - offset)
    }
}
