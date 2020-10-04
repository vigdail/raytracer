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
        self * self
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::xyz(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::xyz(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<&Vector3> for &Vector3 {
    type Output = f32;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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

impl std::ops::Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
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
    fn dot_product() {
        let a = Vector3::xyz(2.0, 3.0, 5.0);
        let b = Vector3::xyz(3.0, 2.0, 0.0);

        let expected = 6.0 + 6.0 + 0.0;

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
}
