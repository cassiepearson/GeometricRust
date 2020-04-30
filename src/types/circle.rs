//! Circle Data Structure
use crate::{
    errors::{GeometryError, MathError},
    traits::numbers::Floats,
    types::point::*,
};

// Struct ----------------------------------------------------------------------
/// Polygon Data Structure
#[derive(Clone, Debug, PartialEq)]
pub struct Circle<T>
where
    T: Floats,
{
    pub center: Point<T>,
    pub radius: T,
}

// Implementation --------------------------------------------------------------
impl<T: Floats> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Circle<T> {
        Circle { center, radius }
    }

    pub fn area(&self) -> Result<T, MathError> {
        let pi: T = match T::from(std::f64::consts::PI) {
            Some(pi) => pi,
            None => {
                return Err(MathError::ConversionFailure(
                    "Failed to convert PI into type T.".to_string(),
                ))
            }
        };
        Ok(self.radius.powi(2) * pi)
    }

    pub fn circumference(&self) -> Result<T, MathError> {
        let pi: T = match T::from(std::f64::consts::PI) {
            Some(pi) => pi,
            None => {
                return Err(MathError::ConversionFailure(
                    "Failed to convert PI into type T.".to_string(),
                ))
            }
        };
        Ok((self.radius + self.radius) * pi)
    }
}
// From ------------------------------------------------------------------------
// Operations ------------------------------------------------------------------
// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_circle_area() {
        let circle_f32 = Circle::new(Point::new(0f32, 0f32), 10f32);
        let circle_f64 = Circle::new(Point::new(0f64, 0f64), 10f64);

        assert!(circle_f32.area().unwrap() - 314f32 <= 1f32);
        assert!(circle_f64.area().unwrap() - 314f64 <= 1f64);
    }

    #[test]
    // #[ignore]
    fn test_circle_circumference() {
        let circle_f32 = Circle::new(Point::new(0f32, 0f32), 10f32);
        let circle_f64 = Circle::new(Point::new(0f64, 0f64), 10f64);

        assert!(circle_f32.circumference().unwrap() - 62f32 <= 1f32);
        assert!(circle_f64.circumference().unwrap() - 62f64 <= 1f64);
    }
}
