use vector::Vector4;

/// A ray of the form:
///  p(t) = A + t*B
///
/// Where p is a 3D position along a line in three dimensions
/// A, is the ray origin, and B is the ray direction.
/// t is some distance from the origin
pub struct Ray {
    origin:    Vector4, // 'a'
    direction: Vector4, // 'b'
}

impl Ray {

    pub fn point_at_distance(&self, distance: f32) -> Vector4 {
        self.origin + (distance * self.direction)
    }

    pub fn origin(&self) -> Vector4 {
        self.origin
    }

    pub fn direction(&self) -> Vector4 {
        self.direction
    }
}
