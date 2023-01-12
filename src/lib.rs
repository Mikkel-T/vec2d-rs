mod utils;
mod ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2D<T> {
    x: T,
    y: T,
}

impl<T: Copy + Clone> Vec2D<T> {
    pub fn new(x: T, y: T) -> Vec2D<T> {
        Vec2D { x, y }
    }

    pub fn from_vector<O: Into<T> + Copy + Clone>(v: Vec2D<O>) -> Vec2D<T> {
        Vec2D {
            x: v.x.into(),
            y: v.y.into(),
        }
    }

    pub fn into_vector<O: From<T> + Copy + Clone>(self) -> Vec2D<O> {
        Vec2D {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let v_u32 = Vec2D::new(6u32, 8u32);
        assert_eq!(v_u32.x, 6u32);
        assert_eq!(v_u32.y, 8u32);

        let v_i32 = Vec2D::new(8i32, 9i32);
        assert_eq!(v_i32.x, 8i32);
        assert_eq!(v_i32.y, 9i32);

        let v_f64 = Vec2D::new(9f64, 3f64);
        assert_eq!(v_f64.x, 9f64);
        assert_eq!(v_f64.y, 3f64);
    }

    #[test]
    fn from_vector() {
        let v_i32 = Vec2D::new(-5, -3);
        let v_f64: Vec2D<f64> = Vec2D::from_vector(v_i32);
        assert_eq!(v_f64, Vec2D::new(-5., -3.))
    }

    #[test]
    fn into_vector() {
        let v_i32 = Vec2D::new(4, -6);
        let v_f64: Vec2D<f64> = v_i32.into_vector();
        assert_eq!(v_f64, Vec2D::new(4., -6.))
    }
}
