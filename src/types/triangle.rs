//! Triangle Structure
use crate::types::point::Point;
use crypticRust::general::numbers::Number;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Triangle<T>
where
    T: Number,
{
    vertices: [Point<T>; 3],
}

impl<T: Number> Triangle<T> {
    pub fn new(vertices: [Point<T>; 3]) -> Self {
        Self { vertices }
    }
}
#[cfg(test)]
mod triangle_tests {
    use super::*;

    #[test]
    fn test_new_usize() {
        let vertices = [
            Point::new(0_usize, 0_usize),
            Point::new(0_usize, 1_usize),
            Point::new(1_usize, 0_usize),
        ];
        let out: Triangle<usize> = Triangle::new(vertices);
        let expected: Triangle<usize> = Triangle { vertices };
        assert_eq!(out, expected);
    }

    #[test]
    fn test_new_isize() {
        let vertices = [
            Point::new(0_isize, 0_isize),
            Point::new(0_isize, 1_isize),
            Point::new(1_isize, 0_isize),
        ];
        let out: Triangle<isize> = Triangle::new(vertices);
        let expected: Triangle<isize> = Triangle { vertices };
        assert_eq!(out, expected);
    }

    #[test]
    fn test_new_float() {
        let vertices = [
            Point::new(0_f64, 0_f64),
            Point::new(0_f64, 1_f64),
            Point::new(1_f64, 0_f64),
        ];
        let out: Triangle<f64> = Triangle::new(vertices);
        let expected: Triangle<f64> = Triangle { vertices };
        assert_eq!(out, expected);
    }
}
