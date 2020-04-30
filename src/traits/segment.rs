//! Segment Trait
use crate::{
    errors::MathError,
    traits::{distance::Distance, numbers::Floats},
    types::point::Point,
};
use std::result::Result::*;

// Trait -----------------------------------------------------------------------
/// Segment Trait
pub trait Segment<T, E> {
    fn cross(&self, other: &Point<T>) -> T;
    fn on_seg(&self, other: &Point<T>) -> Result<bool, E>;
    fn magnitude(&self) -> Result<T, E>;
    fn magnitude2(&self) -> Result<T, E>;
}

// Implementation --------------------------------------------------------------
impl<T: Floats> Segment<T, MathError> for (Point<T>, Point<T>) {
    fn cross(&self, other: &Point<T>) -> T {
        // https://algorithmtutor.com/Computational-Geometry/Check-if-two-line-segment-intersect/
        // This results in a scalar value that is the area of a parallelogram resulting between these points.
        // This area is necesarily signed and helpful in determining the relation between the points.
        // That is to say, vector a is counter clockwise of vector b, etc.
        self.0.x * self.1.y + self.1.x * other.y + other.x * self.0.y
            - self.0.x * other.y
            - self.1.x * self.0.y
            - other.x * self.1.y
    }

    fn on_seg(&self, other: &Point<T>) -> Result<bool, MathError> {
        let eps =
            match T::from(f64::EPSILON) {
                Some(eps) => eps,
                None => return Err(MathError::ConversionFailure(
                    "Failed to convert f64 epsilon to T when checking if two segments intersect."
                        .to_string(),
                )),
            };
        let diff = self.0.distance(&other).unwrap() + self.1.distance(&other).unwrap()
            - self.magnitude().unwrap();
        Ok(diff <= eps && diff >= -eps)
    }

    fn magnitude(&self) -> Result<T, MathError> {
        self.0.distance(&self.1)
    }

    fn magnitude2(&self) -> Result<T, MathError> {
        self.0.distance2(&self.1)
    }
}

// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // #[ignore]
    // fn test_new_() {
    // TODO: Add test
    // }
}
