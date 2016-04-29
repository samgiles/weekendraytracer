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
        let attenuation = Vector4::new3(1.0, 1.0, 1.0);
        let cos_i = ray_in.direction().unit_vector().dot3(hit_record.normal);
        let (outward_normal, ratio, cosine) =
            if cos_i > 0.0 {
                let cosine = self.refractive_index * cos_i / ray_in.direction().length();
                (-hit_record.normal, self.refractive_index, cosine)
            } else {
                let cosine = -cos_i / ray_in.direction().length();
                (hit_record.normal, 1.0 / self.refractive_index, cosine)
            };

        let refract_result = refract(ray_in.direction(), outward_normal, ratio);

        let reflect_probability = schlick(cosine, self.refractive_index);
        if rand::random::<f32>() < reflect_probability {
            let reflected = ray_in.direction().reflect(hit_record.normal);
            Some((Ray::new(hit_record.intersection_point, reflected), attenuation))
        } else {
            if let Some(refracted) = refract_result {
                Some((Ray::new(hit_record.intersection_point, refracted), attenuation))
            } else {
                let reflected = ray_in.direction().reflect(hit_record.normal);
                Some((Ray::new(hit_record.intersection_point, reflected), attenuation))
            }
        }
    }
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

// http://math.stackexchange.com/questions/936936/deduction-of-vector-form-of-snells-law
fn refract(vector_in: Vector4, normal: Vector4, ratio: f32) -> Option<Vector4> {
    let cos_i = vector_in.dot3(normal);
    let sin_t2 = 1.0 - ratio * ratio * (1.0 - cos_i * cos_i);

    if sin_t2 > 0.0 {
        Some((vector_in - (normal * cos_i)) * ratio - (normal * (sin_t2).sqrt()))
    } else {
        None
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
