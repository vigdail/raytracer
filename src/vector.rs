use rand::Rng;

use crate::util::{Random, RandomRange};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        *self * (*self)
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn normalized(&self) -> Vector3 {
        let mut r = self.clone();
        r.normalize();
        r
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random_range(-1.0, 1.0);
            if p.squared_length() <= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        let a = f32::random_range(0.0, std::f32::consts::PI * 2.0);
        let z = f32::random_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();

        Vector3::xyz(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: Vector3) -> Vector3 {
        let v = Vector3::random_in_unit_sphere();

        if v * normal > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let v = Vector3::xyz(f32::random(), f32::random(), 0.0);
            if v.squared_length() < 1.0 {
                return v;
            }
        }
    }
}

impl Random for Vector3 {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vector3::xyz(rng.gen(), rng.gen(), rng.gen())
    }
}

impl RandomRange<f32> for Vector3 {
    fn random_range(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        Vector3::xyz(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::xyz(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::xyz(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = f32;

    fn mul(self, rhs: Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::xyz(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::xyz(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3::xyz(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::xyz(-self.x, -self.y, -self.z)
    }
}

impl std::ops::BitXor<Vector3> for Vector3 {
    type Output = Vector3;

    fn bitxor(self, rhs: Vector3) -> Self::Output {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vector3::xyz(x, y, z)
    }
}

impl std::cmp::PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_new() {
        assert_eq!(
            Vector3::new(),
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn vector_xyz() {
        assert_eq!(
            Vector3::xyz(1.0, 2.0, 3.0),
            Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        )
    }

    #[test]
    fn add() {
        let a = Vector3::xyz(2.0, 3.0, 5.0);
        let b = Vector3::xyz(3.0, 2.0, 0.0);

        let expected = Vector3::xyz(5.0, 5.0, 5.0);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn sub() {
        let a = Vector3::xyz(2.0, 11.5, 4.0);
        let b = Vector3::xyz(1.0, -11.5, 5.0);

        let expected = Vector3::xyz(1.0, 23.0, -1.0);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn dot_product() {
        let a = Vector3::xyz(2.0, 3.0, 5.0);
        let b = Vector3::xyz(3.0, -2.0, 0.0);

        let expected = 6.0 - 6.0 + 0.0;

        assert_eq!(a * b, expected);
    }

    #[test]
    fn length() {
        let a = Vector3::xyz(3.0, 4.0, 0.0);
        assert_eq!(a.length(), 5.0);

        let zero = Vector3::xyz(0.0, 0.0, 0.0);
        assert_eq!(zero.length(), 0.0);

        let up = Vector3::xyz(0.0, 1.0, 0.0);
        assert_eq!(up.length(), 1.0);
    }

    #[test]
    fn normalize() {
        let mut a = Vector3::xyz(20.0, 10.0, -18.0);
        a.normalize();
        assert!((a.length() - 1.0).abs() < 1e-7);
    }

    #[test]
    fn neg() {
        let a = Vector3::xyz(1.0, 2.0, -3.0);
        let b = -a;

        assert_eq!(a.length(), b.length());
        assert_eq!(
            b,
            Vector3 {
                x: -1.0,
                y: -2.0,
                z: 3.0
            }
        )
    }

    #[test]
    fn cross() {
        let a = Vector3::xyz(1.0, 0.0, 0.0);
        let b = Vector3::xyz(0.0, 1.0, 0.0);

        let expect = Vector3::xyz(0.0, 0.0, 1.0);

        assert_eq!(a ^ b, expect);

        let a = Vector3::xyz(0.0, 0.0, 1.0);
        let b = Vector3::xyz(0.0, 1.0, 0.0);

        let expect = Vector3::xyz(-1.0, 0.0, 0.0);

        assert_eq!(a ^ b, expect);

        let a = Vector3::xyz(0.0, 0.0, 1.0);
        let b = Vector3::xyz(0.0, 0.0, 1.0);

        let expect = Vector3::xyz(0.0, 0.0, 0.0);

        assert_eq!(a ^ b, expect);
    }
}
