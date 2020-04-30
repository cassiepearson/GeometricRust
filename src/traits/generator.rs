//! Generates Test Geometries
use crate::{
    traits::numbers::{Floats, Integers},
    types::point::Point,
};
use rand::distributions::Uniform;
use rand::prelude::*;

// Traits ----------------------------------------------------------------------
pub trait IntCircleGenerator<T>
where
    T: Integers,
{
    fn generate_int_circle(&self, max: T) -> (Point<T>, T);
}

pub trait FloatCircleGenerator<T>
where
    T: Floats,
{
    fn generate_circle(&self, max: T) -> (Point<T>, T);
}

pub trait IntRectangleGenerator<T>
where
    T: Integers,
{
    fn generate_int_rectangle(&self, min: T) -> [Point<T>; 4];
}

pub trait FloatRectangleGenerator<T>
where
    T: Floats,
{
    fn generate_rectangle(&self, min: T) -> [Point<T>; 4];
}
// Implementations -------------------------------------------------------------
impl<T: Integers> IntCircleGenerator<T> for Point<T> {
    /// Generate a circle with the assumption point is the center
    fn generate_int_circle(&self, max: T) -> (Point<T>, T) {
        let mut rng = rand::thread_rng();
        let radius: T = rng.sample::<T, _>(Uniform::new::<T, _>(self.x, max));

        (self.clone(), radius)
    }
}

impl<T: Floats> FloatCircleGenerator<T> for Point<T> {
    /// Generate a circle with the assumption point is the center
    fn generate_circle(&self, max: T) -> (Point<T>, T) {
        let mut rng = rand::thread_rng();
        let radius: T = rng.sample::<T, _>(Uniform::new::<T, _>(self.x, max));

        (self.clone(), radius)
    }
}

impl<T: Integers> IntRectangleGenerator<T> for Point<T> {
    /// Generate a rectangle with the assumption point is the top left point
    fn generate_int_rectangle(&self, min: T) -> [Point<T>; 4] {
        // Generate random values
        let mut rng = rand::thread_rng();
        let rand_x: T = rng.sample::<T, _>(Uniform::new::<T, _>(min, self.x));
        let rand_y: T = rng.sample::<T, _>(Uniform::new::<T, _>(min, self.y));

        // Create points
        let top_right: Point<T> = Point::new(rand_x, rand_y);
        let bottom_right: Point<T> = Point::new(self.x, rand_y);
        let top_left: Point<T> = Point::new(rand_x, self.y);

        [self.clone(), bottom_right, top_right, top_left]
    }
}

impl<T: Floats> FloatRectangleGenerator<T> for Point<T> {
    /// Generate a rectangle with the assumption point is the top left point
    fn generate_rectangle(&self, min: T) -> [Point<T>; 4] {
        // Generate random values
        let mut rng = rand::thread_rng();
        let rand_x: T = rng.sample::<T, _>(Uniform::new::<T, _>(min, self.x));
        let rand_y: T = rng.sample::<T, _>(Uniform::new::<T, _>(min, self.y));

        // Create points
        let top_right: Point<T> = Point::new(rand_x, rand_y);
        let bottom_right: Point<T> = Point::new(self.x, rand_y);
        let top_left: Point<T> = Point::new(rand_x, self.y);

        [self.clone(), bottom_right, top_right, top_left]
    }
}

// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_new_integer_square() {
        // Test the generate sqaure implementations

        // Point
        // unsigned int
        let pt_u8: Point<u8> = Point::new(10u8, 10u8);
        let pt_u16: Point<u16> = Point::new(10u16, 10u16);
        let pt_u32: Point<u32> = Point::new(100u32, 100u32);
        let pt_u64: Point<u64> = Point::new(100u64, 100u64);
        let pt_u128: Point<u128> = Point::new(100u128, 100u128);
        // int
        let pt_i8: Point<i8> = Point::new(0i8, 0i8);
        let pt_i16: Point<i16> = Point::new(0i16, 0i16);
        let pt_i32: Point<i32> = Point::new(0i32, 0i32);
        let pt_i64: Point<i64> = Point::new(0i64, 0i64);
        let pt_i128: Point<i128> = Point::new(0i128, 0i128);

        dbg!(pt_u8.generate_int_rectangle(9u8));
        dbg!(pt_u16.generate_int_rectangle(9u16));
        dbg!(pt_u32.generate_int_rectangle(99u32));
        dbg!(pt_u64.generate_int_rectangle(25u64));
        dbg!(pt_u128.generate_int_rectangle(50u128));
        dbg!(pt_i8.generate_int_rectangle(-6i8));
        dbg!(pt_i16.generate_int_rectangle(-15i16));
        dbg!(pt_i32.generate_int_rectangle(-25i32));
        dbg!(pt_i64.generate_int_rectangle(-500i64));
        dbg!(pt_i128.generate_int_rectangle(-500000i128));
    }
}
