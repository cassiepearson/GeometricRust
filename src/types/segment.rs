//! Segment Data Structure
use crate::types::point::Point;
use crypticRust::general::numbers::Number;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Segment<T>
where
    T: Number,
{
    p0: Point<T>,
    p1: Point<T>,
}

impl<T> Segment<T>
where
    T: Number,
{
    pub fn new(p0: Point<T>, p1: Point<T>) -> Self {
        Self { p0, p1 }
    }
}

#[cfg(test)]
mod segment_tests {
    use super::*;

    #[test]
    fn test_new_usize() {
        let p0 = Point::new(0_usize, 0_usize);
        let p1 = Point::new(0_usize, 1_usize);
        let out: Segment<usize> = Segment::new(p0, p1);
        let expected: Segment<usize> = Segment { p0, p1 };
        assert_eq!(out, expected);
    }

    #[test]
    fn test_new_isize() {
        let p0 = Point::new(0_isize, 0_isize);
        let p1 = Point::new(0_isize, 1_isize);
        let out: Segment<isize> = Segment::new(p0, p1);
        let expected: Segment<isize> = Segment { p0, p1 };
        assert_eq!(out, expected);
    }

    #[test]
    fn test_new_float() {
        let p0 = Point::new(0_f64, 0_f64);
        let p1 = Point::new(0_f64, 1_f64);
        let out: Segment<f64> = Segment::new(p0, p1);
        let expected: Segment<f64> = Segment { p0, p1 };
        assert_eq!(out, expected);
    }
}
