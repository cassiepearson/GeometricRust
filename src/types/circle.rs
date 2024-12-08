//! Circle Data Structure
use crate::errors::GeomError;
use crate::types::point::Point;
use anyhow::Result;
use crypticRust::general::numbers::RealNumber;

/// Circle Structure
///
/// All points in a circle are equal distance from the center.
#[derive(Clone, Debug, PartialEq)]
pub struct Circle<T>
where
    T: RealNumber,
{
    pub center: Point<T>,
    pub radius: T,
}

impl<T: RealNumber> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Circle<T> {
        Circle { center, radius }
    }

    pub fn pi(&self) -> Result<T, GeomError> {
        match T::from(std::f64::consts::PI) {
            Some(pi) => Ok(pi),
            None => Err(GeomError::UnsupportedType(
                "convert pi to expected return type.".to_string(),
            )),
        }
    }

    pub fn area(&self) -> Result<T, GeomError> {
        Ok(self.radius.powi(2) * self.pi()?)
    }

    pub fn circumference(&self) -> Result<T, GeomError> {
        Ok((self.radius + self.radius) * self.pi()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use rstest::rstest;

    #[rstest]
    #[case(Point::new(0_f32, 0_f32), 10_f32, 314.15927_f32)]
    #[case(Point::new(0_f32, 0_f32), 1_f32, 3.1415927_f32)]
    fn test_circle_area_f32(
        #[case] center: Point<f32>,
        #[case] radius: f32,
        #[case] expected: f32,
    ) -> Result<()> {
        let circle = Circle::new(center, radius);
        let area = circle.area()?;

        assert!(approx_eq!(f32, area, expected));
        Ok(())
    }

    #[rstest]
    #[case(Point::new(0_f64, 0_f64), 10_f64, 314.1592653589793_f64)]
    #[case(Point::new(0_f64, 0_f64), 1_f64, 3.141592653589793_f64)]
    fn test_circle_area_f64(
        #[case] center: Point<f64>,
        #[case] radius: f64,
        #[case] expected: f64,
    ) -> Result<()> {
        let circle = Circle::new(center, radius);
        let area = circle.area()?;

        assert!(approx_eq!(f64, area, expected));
        Ok(())
    }

    #[rstest]
    #[case(Point::new(0_f64, 0_f64), 10_f64, 62.83185307179586_f64)]
    #[case(Point::new(0_f64, 0_f64), 1_f64, 6.283185307179586_f64)]
    fn test_circle_circumference_f64(
        #[case] center: Point<f64>,
        #[case] radius: f64,
        #[case] expected: f64,
    ) -> Result<()> {
        let circle = Circle::new(center, radius);
        let circumference = circle.circumference()?;

        assert!(approx_eq!(f64, circumference, expected));
        Ok(())
    }

    #[rstest]
    #[case(Point::new(0_f32, 0_f32), 10_f32, 62.83185_f32)]
    #[case(Point::new(0_f32, 0_f32), 1_f32, 6.283185_f32)]
    fn test_circle_circumference_f32(
        #[case] center: Point<f32>,
        #[case] radius: f32,
        #[case] expected: f32,
    ) -> Result<()> {
        let circle = Circle::new(center, radius);
        let circumference = circle.circumference()?;

        assert!(approx_eq!(f32, circumference, expected));
        Ok(())
    }
}
