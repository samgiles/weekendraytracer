use ray::Ray;
use rand;
use vector::Vector4;

use renderable::IntersectionRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &IntersectionRecord) -> Option<(Ray, Vector4)>;
}

pub struct Lambertian {
    albedo: Vector4,
}

impl Lambertian {
    pub fn new(albedo: Vector4) -> Self {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &IntersectionRecord) -> Option<(Ray, Vector4)> {
        let target = hit_record.intersection_point + hit_record.normal + random_in_unit_sphere();
        Some((Ray::new(hit_record.intersection_point, target - hit_record.intersection_point), self.albedo))
    }
}

pub struct Metal {
    albedo: Vector4,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector4, fuzz: f32) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &IntersectionRecord) -> Option<(Ray, Vector4)> {
        let reflected = ray_in.direction().unit_vector().reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.intersection_point, reflected + (random_in_unit_sphere() * self.fuzz));

        if scattered.direction().dot3(hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
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
