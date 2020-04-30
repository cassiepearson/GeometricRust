//! Rectangle Data Structure
use crate::{
    errors::{GeometryError, MathError},
    traits::{distance::Distance, numbers::Floats},
    types::{point::Point, polygon::Polygon},
};
use itertools::Itertools;
use std::convert::TryInto;

// Struct ----------------------------------------------------------------------
/// Polygon Data Structure
#[derive(Clone, Debug, PartialEq)]
pub struct Rectangle<T>
where
    T: Floats,
{
    pub vertices: [Point<T>; 4],
}

// Implementation --------------------------------------------------------------
impl<T: Floats> Rectangle<T> {
    pub fn new(vertices: [Point<T>; 4]) -> Rectangle<T> {
        Rectangle { vertices }
    }

    pub fn new_vec(vertices: Vec<Point<T>>) -> Result<Rectangle<T>, GeometryError> {
        Ok(Rectangle {
            vertices: vertices[0..3]
                .try_into()
                .expect("Failed to convert vector slice into an array of 4 points."),
        })
    }

    /// Perimeter of a rectangle
    pub fn perimeter(&self) -> Result<T, MathError> {
        Ok(self
            .iter_tuple_segments()
            .fold(T::from(0f64).unwrap(), |sum, seg| {
                sum + seg.0.distance(&seg.1).unwrap()
            }))
    }

    /// Area of a rectangle
    pub fn area(&self) -> Result<T, MathError> {
        let width: T = match self.vertices[0].distance(&self.vertices[1]) {
            Ok(width) => width,
            Err(err) => return Err(err),
        };
        let height: T = match self.vertices[1].distance(&self.vertices[2]) {
            Ok(height) => height,
            Err(err) => return Err(err),
        };
        Ok(width * height)
    }

    fn iter_tuple_segments<'a>(&'a self) -> impl Iterator<Item = (Point<T>, Point<T>)> + 'a {
        self.vertices
            .iter()
            .chain(self.vertices.iter().take(1))
            .tuple_windows()
            .map(move |(pt1, pt2)| (*pt1, *pt2))
    }
}

// From ------------------------------------------------------------------------
/// Create an axis aligned bounding box as a Rectangle from a Polygon
impl<T: Floats> From<Polygon<T>> for Rectangle<T> {
    fn from(polygon: Polygon<T>) -> Self {
        // Create an axis aligned bounding box from the polygon
        let mut min_x = polygon.vertices[0].x;
        let mut min_y = polygon.vertices[0].y;
        let mut max_x = polygon.vertices[0].x;
        let mut max_y = polygon.vertices[0].y;
        polygon.vertices.iter().for_each(|pt| {
            if pt.x >= max_x {
                max_x = pt.x;
            }
            if pt.x <= min_x {
                min_x = pt.x;
            }
            if pt.y >= max_y {
                max_y = pt.y;
            }
            if pt.y <= min_y {
                min_y = pt.y;
            }
        });
        let vertices: [Point<T>; 4] = [
            Point::new(min_x, min_y),
            Point::new(max_x, min_y),
            Point::new(max_x, max_y),
            Point::new(min_x, max_y),
        ];
        Rectangle { vertices }
    }
}

/// Create an axis aligned bounding box as a Rectangle from a vector of points
impl<T: Floats> From<Vec<Point<T>>> for Rectangle<T> {
    fn from(vec: Vec<Point<T>>) -> Self {
        // Create an axis aligned bounding box from the vector of points
        let mut min_x = vec[0].x;
        let mut min_y = vec[0].y;
        let mut max_x = vec[0].x;
        let mut max_y = vec[0].y;
        vec.iter().for_each(|pt| {
            if pt.x >= max_x {
                max_x = pt.x;
            }
            if pt.x <= min_x {
                min_x = pt.x;
            }
            if pt.y >= max_y {
                max_y = pt.y;
            }
            if pt.y <= min_y {
                min_y = pt.y;
            }
        });
        let vertices: [Point<T>; 4] = [
            Point::new(min_x, min_y),
            Point::new(max_x, min_y),
            Point::new(max_x, max_y),
            Point::new(min_x, max_y),
        ];
        Rectangle { vertices }
    }
}
// Operations ------------------------------------------------------------------
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
