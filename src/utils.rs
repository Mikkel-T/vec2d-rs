use crate::Vec2D;
use std::ops::{Add, Div, Mul, Neg};

impl<T> Vec2D<T>
where
    T: Neg<Output = T> + Copy + Clone,
{
    /// Get a the normal vector of a vector
    pub fn normal(self) -> Self {
        Vec2D {
            x: -self.y,
            y: self.x,
        }
    }
}

impl<T> Vec2D<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Copy + Clone,
{
    /// Get the dot product of two vectors
    pub fn dot(v1: Self, v2: Self) -> T {
        v1.x * v2.x + v1.y * v2.y
    }

    /// Get the squared length of a vector
    pub fn len_sq(self) -> T {
        Vec2D::dot(self, self)
    }

    /// Get the length of a vector
    pub fn len(self) -> f64
    where
        T: Into<f64>,
    {
        f64::sqrt(self.len_sq().into())
    }

    /// Get the determinant of two vectors
    pub fn det(v1: Self, v2: Self) -> T
    where
        T: Neg<Output = T>,
    {
        Vec2D::dot(v1.normal(), v2)
    }

    /// Get the projection of the current vector on `v2`
    pub fn proj(self, v2: Self) -> Self
    where
        T: Div<T, Output = T>,
    {
        v2 * (Vec2D::dot(self, v2) / v2.len_sq())
    }

    /// Get the angle between two vectors in radians
    pub fn angle(v1: Self, v2: Self) -> f64
    where
        T: Into<f64>,
    {
        f64::acos(Vec2D::dot(v1, v2).into() / (v1.len() * v2.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let a = Vec2D::new(6, 8);
        assert_eq!(a.normal(), Vec2D::new(-8, 6));
    }

    #[test]
    fn dot() {
        let a = Vec2D::new(6, 1);
        let b = Vec2D::new(4, 2);
        assert_eq!(Vec2D::dot(a, b), 26);
    }

    #[test]
    fn len_sq() {
        let a = Vec2D::new(5, 7);
        assert_eq!(a.len_sq(), 74);
    }

    #[test]
    fn len() {
        let a = Vec2D::new(3, 4);
        assert_eq!(a.len(), 5.);
    }

    #[test]
    fn det() {
        let a = Vec2D::new(-6, 1);
        let b = Vec2D::new(9, 4);
        assert_eq!(Vec2D::det(a, b), -33);
    }

    #[test]
    fn proj() {
        let a = Vec2D::new(-2, 4);
        let b = Vec2D::new(3, -1);
        assert_eq!(a.proj(b), Vec2D::new(-3, 1));
    }

    #[test]
    fn angle() {
        let a = Vec2D::new(2, 2);
        let b = Vec2D::new(1, 0);
        assert!((Vec2D::angle(a, b) - (std::f64::consts::PI / 4.0)).abs() < 1e-10);
        assert!((Vec2D::angle(a, b).to_degrees() - 45.).abs() < 1e-10)
    }
}
