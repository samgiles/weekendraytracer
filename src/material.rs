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

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric {
            refractive_index: refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &IntersectionRecord) -> Option<(Ray, Vector4)> {
        let attenuation = Vector4::new3(1.0, 1.0, 0.0);
        let (outward_normal, ratio) =
            if hit_record.normal.dot3(ray_in.direction()) > 0.0 {
                (-hit_record.normal, self.refractive_index)
            } else {
                let refractive_index_of_air = 1.0;
                (hit_record.normal, refractive_index_of_air / self.refractive_index)
            };

        if let Some(refracted) = refract(ray_in.direction(), outward_normal, ratio) {
            Some((Ray::new(hit_record.intersection_point, refracted), attenuation))
        } else {
            None
            //let reflected = ray_in.direction().reflect(hit_record.normal);
            //Some((Ray::new(hit_record.intersection_point, reflected), attenuation))
        }
    }
}

// http://math.stackexchange.com/questions/936936/deduction-of-vector-form-of-snells-law
fn refract(vector_in: Vector4, normal: Vector4, ratio: f32) -> Option<Vector4> {
    let cos_i = normal.dot3(vector_in);
    let sin_t2 = ratio * ratio * (1.0 - cos_i * cos_i);

    if sin_t2 > 1.0 {
        None
    } else {
        Some(vector_in * ratio - (normal + (1.0 - sin_t2).sqrt()) * normal)
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
