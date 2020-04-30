//! Intersects and Intersection Traits
use crate::{
    errors::MathError,
    traits::{numbers::Floats, segment::Segment},
    types::point::Point,
};
use std::result::Result::*;

// Trait -----------------------------------------------------------------------
/// Intersects Trait
pub trait Intersects<T, E> {
    fn intersects(&self, other: &T) -> Result<bool, E>;
}

// Implementation --------------------------------------------------------------
impl<T: Floats> Intersects<(Point<T>, Point<T>), MathError> for (Point<T>, Point<T>) {
    fn intersects(&self, other: &Self) -> Result<bool, MathError> {
        // https://algorithmtutor.com/Computational-Geometry/Check-if-two-line-segment-intersect/

        let eps =
            match T::from(f64::EPSILON) {
                Some(eps) => eps,
                None => return Err(MathError::ConversionFailure(
                    "Failed to convert f64 epsilon to T when checking if two segments intersect."
                        .to_string(),
                )),
            };
        let d1 = other.cross(&self.0);
        let d2 = other.cross(&self.1);
        let d3 = self.cross(&other.0);
        let d4 = self.cross(&other.1);

        if ((d1 >= eps && d2 <= eps) || (d1 <= eps && d2 >= eps))
            && ((d3 >= eps && d4 <= eps) || (d3 <= eps && d4 >= eps))
        {
            Ok(true)
        } else if d1 <= eps && d1 >= -eps && other.on_seg(&self.0).unwrap() {
            Ok(true)
        } else if d2 <= eps && d2 >= -eps && other.on_seg(&self.1).unwrap() {
            Ok(true)
        } else if d3 <= eps && d3 >= -eps && self.on_seg(&other.0).unwrap() {
            Ok(true)
        } else if d4 <= eps && d4 >= -eps && self.on_seg(&other.1).unwrap() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// IMPL INTERSECTIONS
// http://paulbourke.net/geometry/pointlineplane/

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
