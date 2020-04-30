//! Distance Trait
use crate::{errors::MathError, traits::numbers::Floats, types::point::Point};
use cgmath::prelude::InnerSpace;
use std::result::Result::*;

// Trait -----------------------------------------------------------------------
/// Distance Trait
pub trait Distance<T, R, E> {
    fn distance(&self, other: &T) -> Result<R, E>;
    fn distance2(&self, other: &T) -> Result<R, E>;
}

// Implementation --------------------------------------------------------------
impl<T: Floats> Distance<Point<T>, T, MathError> for Point<T> {
    fn distance(&self, other: &Self) -> Result<T, MathError> {
        let dist = self.distance2(other).or(Err(MathError::CalculationFailure(
            "Failed to calculate distance2 when calculating distance.".to_string(),
        )))?;
        Ok(dist.sqrt())
    }

    fn distance2(&self, other: &Self) -> Result<T, MathError> {
        let vector = (other - self)
            .cast::<f64>()
            .ok_or(MathError::ConversionFailure(
                "Failed to convert the point to a vector when calculating distance2.".to_string(),
            ))?;
        let dist = match T::from(vector.magnitude2()) {
            Some(dist) => dist,
            None => {
                return Err(MathError::ConversionFailure(
                    "Failed to convert from f64 into T when calculating distance.".to_string(),
                ))
            }
        };
        Ok(dist)
    }
}

impl<T: Floats> Distance<(T, T), T, MathError> for (T, T) {
    fn distance(&self, other: &Self) -> Result<T, MathError> {
        match T::from(
            Point::new(self.0, self.1)
                .distance(&Point::new(other.0, other.1))
                .unwrap(),
        ) {
            Some(dist) => Ok(dist),
            None => {
                return Err(MathError::ConversionFailure(
                    "Failed to convert from f64 into T when calculating distance.".to_string(),
                ))
            }
        }
    }

    fn distance2(&self, other: &Self) -> Result<T, MathError> {
        match T::from(
            Point::new(self.0, self.1)
                .distance2(&Point::new(other.0, other.1))
                .unwrap(),
        ) {
            Some(dist) => Ok(dist),
            None => {
                return Err(MathError::ConversionFailure(
                    "Failed to convert from f64 into T when calculating distance.".to_string(),
                ))
            }
        }
    }
}

// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_point_distance() {
        // Horizontal
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(10f64, 0f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 10f64);

        // Vertical
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(0f64, 10f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 10f64);

        // Slope up
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(10f64, 10f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 200f64.sqrt());

        // Slope down
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(-10f64, -10f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 200f64.sqrt());
    }

    #[test]
    // #[ignore]
    fn test_point_distance2() {
        // Horizontal
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(10f64, 0f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 100f64);

        // Vertical
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(0f64, 10f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 100f64);

        // Slope up
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(10f64, 10f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 200f64);

        // Slope down
        let pt1 = Point::new(0f64, 0f64);
        let pt2 = Point::new(-10f64, -10f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 200f64);
    }

    #[test]
    // #[ignore]
    fn test_f64_tuple_distance() {
        // Horizontal
        let pt1 = (0f64, 0f64);
        let pt2 = (10f64, 0f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 10f64);

        // Vertical
        let pt1 = (0f64, 0f64);
        let pt2 = (0f64, 10f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 10f64);

        // Slope up
        let pt1 = (0f64, 0f64);
        let pt2 = (10f64, 10f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 200f64.sqrt());

        // Slope down
        let pt1 = (0f64, 0f64);
        let pt2 = (-10f64, -10f64);
        assert_eq!(pt1.distance(&pt2).unwrap(), 200f64.sqrt());
    }

    #[test]
    // #[ignore]
    fn test_f64_tuple_distance2() {
        // Horizontal
        let pt1 = (0f64, 0f64);
        let pt2 = (10f64, 0f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 100f64);

        // Vertical
        let pt1 = (0f64, 0f64);
        let pt2 = (0f64, 10f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 100f64);

        // Slope up
        let pt1 = (0f64, 0f64);
        let pt2 = (10f64, 10f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 200f64);

        // Slope down
        let pt1 = (0f64, 0f64);
        let pt2 = (-10f64, -10f64);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 200f64);
    }

    #[test]
    // #[ignore]
    fn test_f32_tuple_distance() {
        // Horizontal
        let pt1 = (0f32, 0f32);
        let pt2 = (10f32, 0f32);
        assert_eq!(pt1.distance(&pt2).unwrap(), 10f32);

        // Vertical
        let pt1 = (0f32, 0f32);
        let pt2 = (0f32, 10f32);
        assert_eq!(pt1.distance(&pt2).unwrap(), 10f32);

        // Slope up
        let pt1 = (0f32, 0f32);
        let pt2 = (10f32, 10f32);
        assert_eq!(pt1.distance(&pt2).unwrap(), 200f32.sqrt());

        // Slope down
        let pt1 = (0f32, 0f32);
        let pt2 = (-10f32, -10f32);
        assert_eq!(pt1.distance(&pt2).unwrap(), 200f32.sqrt());
    }

    #[test]
    // #[ignore]
    fn test_f32_tuple_distance2() {
        // Horizontal
        let pt1 = (0f32, 0f32);
        let pt2 = (10f32, 0f32);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 100f32);

        // Vertical
        let pt1 = (0f32, 0f32);
        let pt2 = (0f32, 10f32);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 100f32);

        // Slope up
        let pt1 = (0f32, 0f32);
        let pt2 = (10f32, 10f32);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 200f32);

        // Slope down
        let pt1 = (0f32, 0f32);
        let pt2 = (-10f32, -10f32);
        assert_eq!(pt1.distance2(&pt2).unwrap(), 200f32);
    }
}
