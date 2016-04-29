use std::ops;

/// A four-dimensional homogenous Vector. (4xf32)
#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    /// Create a new 4D Vector
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    /// Create a 3D vector (4D with zeroed `w` component)
    pub fn new3(x: f32, y: f32, z: f32) -> Self {
        Vector4::new(x, y, z, 0.0)
    }

    /// Calculates the 3D dot product of two 4D Vectors
    #[inline]
    pub fn dot3(self, rhs: Vector4) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }

    /// Calculates the 4D dot product.
    #[inline]
    pub fn dot4(self, rhs: Vector4) -> f32 {
        self.w * rhs.w +
        self.dot3(rhs)
    }

    /// Calculates the 3D cross product of two 4D Vectors
    pub fn cross(self, rhs: Vector4) -> Self {
        Vector4::new3(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x +
        self.y * self.y +
        self.z * self.z +
        self.w * self.w
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn reflect(self, normal: Vector4) -> Self {
        self - normal * (2.0 * self.dot3(normal))
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector4::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        )
    }
}

impl ops::MulAssign<f32> for Vector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl ops::Mul<Vector4> for Vector4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl ops::MulAssign<Vector4> for Vector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl ops::Div<f32> for Vector4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let _rhs = 1.0 / rhs;
        self * _rhs
    }
}

impl ops::DivAssign<f32> for Vector4 {

    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        let _rhs = 1.0 / rhs;
        let _x = self.x * _rhs;
        let _y = self.y * _rhs;
        let _z = self.z * _rhs;
        let _w = self.w * _rhs;

        // TOOD: Trying to use *= fails to build
        // self *= _rhs;
        self.x = _x;
        self.y = _y;
        self.z = _z;
        self.w = _w;
    }
}

impl ops::Add<Vector4> for Vector4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl ops::AddAssign for Vector4 {
    #[inline]
    fn add_assign(&mut self, rhs: Vector4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl ops::Add<f32> for Vector4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Vector4::new(
            self.x + rhs,
            self.y + rhs,
            self.z + rhs,
            self.w + rhs,
        )
    }
}

impl ops::Sub<f32> for Vector4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Vector4::new(
            self.x - rhs,
            self.y - rhs,
            self.z - rhs,
            self.w - rhs
        )
    }
}

impl ops::SubAssign<f32> for Vector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl ops::Sub<Vector4> for Vector4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl ops::SubAssign<Vector4> for Vector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl ops::Neg for Vector4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_multiply() {
        let a = Vector4::new(10.0, 20.0, 30.0, 40.0);
        let b = Vector4::new(1.0, 2.0, 3.0, 4.0);

        let result = a.clone() * b.clone();
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 40.0);
        assert_eq!(result.z, 90.0);
        assert_eq!(result.w, 160.0);

        let mut a = a;
        a *= b;
        assert_eq!(a.x, 10.0);
        assert_eq!(a.y, 40.0);
        assert_eq!(a.z, 90.0);
        assert_eq!(a.w, 160.0);
    }

    #[test]
    fn test_scalar_multiply() {
        let test = Vector4::new(10.0, 20.0, 30.0, 40.0);
        let result = test.clone() * 2.0;

        assert_eq!(result.x, 20.0);
        assert_eq!(result.y, 40.0);
        assert_eq!(result.z, 60.0);
        assert_eq!(result.w, 80.0);

        let mut test = test;
        test *= 2.0;
        assert_eq!(test.x, 20.0);
        assert_eq!(test.y, 40.0);
        assert_eq!(test.z, 60.0);
        assert_eq!(test.w, 80.0);
    }

    #[test]
    fn test_scalar_division() {
        let test = Vector4::new(10.0, 20.0, 30.0, 40.0);

        // test div
        let result = test.clone() / 2.0;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, 15.0);
        assert_eq!(result.w, 20.0);

        // Test div assign
        let mut test = test;
        test /= 2.0;
        assert_eq!(test.x, 5.0);
        assert_eq!(test.y, 10.0);
        assert_eq!(test.z, 15.0);
        assert_eq!(test.w, 20.0);
    }

    #[test]
    fn test_scalar_sub() {
        let a = Vector4::new(10.0, 20.0, 30.0, 40.0);

        // Test sub
        let result = a.clone() - 2.0;
        assert_eq!(result.x, 8.0);
        assert_eq!(result.y, 18.0);
        assert_eq!(result.z, 28.0);
        assert_eq!(result.w, 38.0);

        // Test assign_sub
        let mut a = a;
        a -= 2.0;
        assert_eq!(a.x, 8.0);
        assert_eq!(a.y, 18.0);
        assert_eq!(a.z, 28.0);
        assert_eq!(a.w, 38.0);
    }

    #[test]
    fn test_vector_sub() {
        let a = Vector4::new(10.0, 20.0, 30.0, 40.0);
        let b = Vector4::new(2.0, 2.0, 2.0, 2.0);

        // Test sub
        let result = a.clone() - b.clone();
        assert_eq!(result.x, 8.0);
        assert_eq!(result.y, 18.0);
        assert_eq!(result.z, 28.0);
        assert_eq!(result.w, 38.0);

        // Test assign_sub
        let mut a = a;
        a -= b;
        assert_eq!(a.x, 8.0);
        assert_eq!(a.y, 18.0);
        assert_eq!(a.z, 28.0);
        assert_eq!(a.w, 38.0);
    }

    #[test]
    fn test_vector_addition() {
        let a = Vector4::new(10.0, 20.0, 30.0, 40.0);
        let b = Vector4::new(2.0, 2.0, 2.0, 2.0);

        // Test add
        let result = a.clone() + b.clone();
        assert_eq!(result.x, 12.0);
        assert_eq!(result.y, 22.0);
        assert_eq!(result.z, 32.0);
        assert_eq!(result.w, 42.0);

        // Test assign_add
        let mut a = a;
        a += b;
        assert_eq!(a.x, 12.0);
        assert_eq!(a.y, 22.0);
        assert_eq!(a.z, 32.0);
        assert_eq!(a.w, 42.0);
    }

    #[test]
    fn test_vector_negation() {
        let a = Vector4::new(10.0, 20.0, 30.0, 40.0);
        let result = -a;
        assert_eq!(result.x, -10.0);
        assert_eq!(result.y, -20.0);
        assert_eq!(result.z, -30.0);
        assert_eq!(result.w, -40.0);
    }

    #[test]
    fn test_dot3() {
        let a = Vector4::new(10.0, 11.0, 12.0, 0.0);
        let b = Vector4::new(2.0, 4.0, 5.0, 0.0);

        let result = a.dot3(b);

        assert_eq!(result, 124.0);
    }

    #[test]
    fn test_dot4() {
        let a = Vector4::new(10.0, 11.0, 12.0, 1.0);
        let b = Vector4::new(2.0, 4.0, 5.0, 2.0);

        let result = a.dot4(b);

        assert_eq!(result, 126.0);
    }

    #[test]
    fn test_cross() {
        let a = Vector4::new(2.0, 3.0, 4.0, 0.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 0.0);

        let result = a.cross(b);
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, -3.0);
    }
}
