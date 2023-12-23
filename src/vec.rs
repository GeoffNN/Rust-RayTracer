use rand::Rng;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn zeros() -> Vec3 {
        return Vec3::new(0., 0., 0.);
    }

    pub fn ones() -> Vec3 {
        return Vec3::new(1., 1., 1.);
    }

    pub fn close_to_with_tol(self, other: Vec3, tol: f64) -> bool {
        (self - other).length_squared() < tol.powi(2)
    }

    pub fn close_to(self, other: Vec3) -> bool {
        let tol = 1e-8;
        self.close_to_with_tol(other, tol)
    }

    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_bounds(rng: &mut rand::rngs::ThreadRng, lower: f64, upper: f64) -> Vec3 {
        Vec3::new(
            rng.gen_range(lower..upper),
            rng.gen_range(lower..upper),
            rng.gen_range(lower..upper),
        )
    }

    pub fn random_in_unit_ball(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        loop {
            let candidate = Vec3::random_in_bounds(rng, -1., 1.);
            if candidate.length_squared() < 1. {
                return candidate;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        Vec3::random_in_unit_ball(rng).normalize()
    }

    pub fn random_in_hemisphere(rng: &mut rand::rngs::ThreadRng, normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_ball(rng);
        if in_unit_sphere.dot(&normal) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        loop {
            let candidate = Vec3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), 0.);
            if candidate.length_squared() < 1. {
                return candidate;
            }
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        return *self / len;
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2. * self.dot(normal) * (*normal)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

pub type Point3 = Vec3;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a + b;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_subtraction() {
        let a = Vec3::new(5.0, 7.0, 9.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let result = a - b;
        assert_eq!(result, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_vector_negation() {
        let a = Vec3::new(1.0, -2.0, 3.0);
        let result = -a;
        assert_eq!(result, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a.dot(&b);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a.cross(&b);
        assert_eq!(result, Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_length_squared() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let result = a.length_squared();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_length() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let result = a.length();
        assert_eq!(result, 14.0f64.sqrt());
    }

    #[test]
    fn test_multiplication_by_scalar() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;
        let result = a * scalar;
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_division_by_scalar() {
        let a = Vec3::new(2.0, 4.0, 6.0);
        let scalar = 2.0;
        let result = a / scalar;
        assert_eq!(result, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_normalize() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let result = a.normalize();
        assert_eq!(
            result,
            Vec3::new(
                1.0 / 3.0_f64.sqrt(),
                1.0 / 3.0_f64.sqrt(),
                1.0 / 3.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn test_normalize_gives_unit_norm() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let result = a.normalize();
        assert_eq!(result.length(), 1.0);
    }

    #[test]
    fn test_cross_product_orthogonality() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let result = a.cross(&b);
        assert_eq!(result, Vec3::new(0.0, 0.0, 1.0));
        let dot_with_a = result.dot(&a);
        let dot_with_b = result.dot(&b);
        assert!(dot_with_a.abs() < std::f64::EPSILON && dot_with_b.abs() < std::f64::EPSILON);
    }
    #[test]
    fn test_cross_product_anticommutativity() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result_ab = a.cross(&b);
        let result_ba = b.cross(&a);
        assert_eq!(result_ab, -result_ba);
    }
}
