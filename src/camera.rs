use vector::Vector4;
use ray::Ray;

pub struct Camera {
    origin: Vector4,
    lower_left: Vector4,
    horizontal: Vector4,
    vertical: Vector4,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            origin: Vector4::new3(0.0, 0.0, 0.0),
            lower_left: Vector4::new3(-2.0, -1.0, -1.0),
            horizontal: Vector4::new3(4.0, 0.0, 0.0),
            vertical: Vector4::new3(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left + (self.horizontal * u) + (self.vertical * v))
    }
}
