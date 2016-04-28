use material::Material;
use vector::Vector4;
use ray::Ray;
use renderable::{ IntersectionRecord, Renderable };
use std::rc::Rc;

pub struct Sphere {
    center: Vector4,
    radius: f32,
    material: Rc<Box<Material>>,
}

impl Sphere {
    pub fn new<TMaterial: Material + 'static>(center: Vector4, radius: f32, material: TMaterial) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: Rc::new(Box::new(material))
        }
    }
}

impl Renderable for Sphere {
    fn intersects(&self, ray: &Ray, distance_min: f32, distance_max: f32)
        -> Option<IntersectionRecord> {

        let o_minus_c = ray.origin() - self.center;

        let a = ray.direction().dot3(ray.direction());
        let b = o_minus_c.dot3(ray.direction());
        let c = (o_minus_c).dot3(o_minus_c) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        // Get the distance (d) value if a hit occurred ahead of the ray
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let distance = (-b - sqrt_discriminant) / a;

            if distance < distance_max && distance > distance_min {
                let intersection_point = ray.point_at_distance(distance);
                return Some(IntersectionRecord::new(
                        distance,
                        intersection_point,
                        (intersection_point - self.center) / self.radius,
                        self.material.clone(),
                    ));
            }

            let distance = (-b + sqrt_discriminant) / a;
            if distance < distance_max && distance > distance_min {
                let intersection_point = ray.point_at_distance(distance);
                return Some(IntersectionRecord::new(
                        distance,
                        intersection_point,
                        (intersection_point - self.center) / self.radius,
                        self.material.clone(),
                     ));
            }
        }

        None
    }
}
