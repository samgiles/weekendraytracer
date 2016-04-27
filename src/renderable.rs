use vector::Vector4;
use ray::Ray;

pub struct IntersectionRecord {
    pub distance: f32,
    pub intersection_point: Vector4,
    pub normal: Vector4,
}

impl IntersectionRecord {
    pub fn new(distance: f32, intersection_point: Vector4, normal: Vector4) -> Self {
        IntersectionRecord {
            distance: distance,
            intersection_point: intersection_point,
            normal: normal,
        }
    }
}

pub trait Renderable {
    fn intersects(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<IntersectionRecord>;
}

impl Renderable for [Box<Renderable>] {
    fn intersects(&self, ray: &Ray, distance_min: f32, distance_max: f32) -> Option<IntersectionRecord> {
        let mut closest_so_far = distance_max;
        let mut record = None;

        for renderable in self.iter() {
            let intersection = renderable.intersects(ray, distance_min, closest_so_far);

            if let Some(intersection_record) = intersection {
                closest_so_far = intersection_record.distance;
                record = Some(intersection_record);
            }
        }

        record
    }
}
