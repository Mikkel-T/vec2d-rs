use crate::Vec2D;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Implement addition of two vectors
impl<T, O> Add<Vec2D<T>> for Vec2D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;

    fn add(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Implement subtraction of two vectors
impl<T, O> Sub<Vec2D<T>> for Vec2D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;

    fn sub(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Implement multiplication of a vector with a number
impl<T, O> Mul<T> for Vec2D<T>
where
    T: Mul<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Implement addition assignment for vectors
impl<T> AddAssign<Vec2D<T>> for Vec2D<T>
where
    T: Add<T, Output = T> + Copy + Clone,
{
    fn add_assign(&mut self, rhs: Vec2D<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

// Implement subtraction assignment for vectors
impl<T> SubAssign<Vec2D<T>> for Vec2D<T>
where
    T: Sub<T, Output = T> + Copy + Clone,
{
    fn sub_assign(&mut self, rhs: Vec2D<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

// Implement multiplication assignment for a vector and a number
impl<T> MulAssign<T> for Vec2D<T>
where
    T: Mul<T, Output = T> + Copy + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

// Implement negative vectors
impl<T, O> Neg for Vec2D<T>
where
    T: Neg<Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;

    fn neg(self) -> Self::Output {
        Vec2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec2D::new(7, 9);
        let b = Vec2D::new(2, 4);
        assert_eq!(a + b, Vec2D::new(9, 13));
    }

    #[test]
    fn sub() {
        let a = Vec2D::new(5, 6);
        let b = Vec2D::new(5, 1);
        assert_eq!(a - b, Vec2D::new(0, 5));
    }

    #[test]
    fn mul() {
        let a = Vec2D::new(8, 2);
        assert_eq!(a * 5, Vec2D::new(40, 10));
    }

    #[test]
    fn add_assign() {
        let mut a = Vec2D::new(1, 7);
        let b = Vec2D::new(4, 5);
        a += b;
        assert_eq!(a, Vec2D::new(5, 12));
    }

    #[test]
    fn sub_assign() {
        let mut a = Vec2D::new(9, 1);
        let b = Vec2D::new(7, 5);
        a -= b;
        assert_eq!(a, Vec2D::new(2, -4));
    }

    #[test]
    fn mul_assign() {
        let mut a = Vec2D::new(9, 3);
        a *= 4;
        assert_eq!(a, Vec2D::new(36, 12));
    }

    #[test]
    fn neg() {
        let a = Vec2D::new(7, 1);
        assert_eq!(-a, Vec2D::new(-7, -1));
    }
}
