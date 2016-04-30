use vector::Vector4;
use ray::Ray;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vector4,
    lower_left: Vector4,
    horizontal: Vector4,
    vertical: Vector4,
}

impl Camera {
    pub fn new(look_from: Vector4, look_at: Vector4,
               up: Vector4, vertical_fov: f32, aspect: f32) -> Self {
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width  = aspect * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            lower_left: look_from - (u * half_width)  -  (v * half_height) - w,
            horizontal: u * half_width * 2.0,
            vertical:  v * half_height * 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left + (self.horizontal * u) + (self.vertical * v) - self.origin)
    }
}
