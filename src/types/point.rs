use crypticRust::general::numbers::Number;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point<T>
where
    T: Number,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test_new_usize() {
        let out = Point::new(0_usize, 0_usize);
        let expected = Point {
            x: 0_usize,
            y: 0_usize,
        };
        assert_eq!(out, expected);
    }

    #[test]
    fn test_new_isize() {
        let out = Point::new(0_isize, 0_isize);
        let expected = Point {
            x: 0_isize,
            y: 0_isize,
        };
        assert_eq!(out, expected);
    }

    #[test]
    fn test_new_float() {
        let out = Point::new(0_f64, 0_f64);
        let expected = Point { x: 0_f64, y: 0_f64 };
        assert_eq!(out, expected);
    }
}
