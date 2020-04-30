//! Polygon Structure
use crate::{
    errors::{GeometryError, MathError},
    traits::distance::Distance,
    traits::numbers::Floats,
    types::ordering::PointOrder,
    types::point::Point,
    types::rectangle::Rectangle,
};
use itertools::Itertools;

// Struct ----------------------------------------------------------------------
/// Polygon Data Structure
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon<T>
where
    T: Floats,
{
    pub vertices: Vec<Point<T>>,
}

// Implementation --------------------------------------------------------------
impl<T: Floats> Polygon<T> {
    /// Create a new Polygon
    ///
    /// This function will check orientation and the number of vertices
    pub fn new(vertices: Vec<Point<T>>) -> Result<Self, GeometryError> {
        if vertices.len() < 3 {
            return Err(GeometryError::InstantiationFailure(
                "Not a valid polygon, less than 3 verts.".to_string(),
            ));
        }
        let poly = Polygon { vertices };
        let area = match poly.area() {
            Ok(area) => area,
            Err(_) => {
                return Err(GeometryError::InstantiationFailure(
                    "Not a valid polygon, check that vertices are in counterclockwise order."
                        .to_string(),
                ))
            }
        };
        if area > T::from(0f64).unwrap() {
            Ok(poly)
        } else {
            let mut vertices = poly.vertices;
            vertices.reverse();
            Ok(Polygon { vertices })
        }
    }

    /// Determine where vertex belongs in the polygon and insert
    pub fn insert_vertex(self, pt: Point<T>) -> Result<Self, GeometryError> {
        if self.vertices.contains(&pt) {
            println!("{:?} is already in polygon", pt);
            return Ok(self);
        }
        let mut success = false;
        let vertices = self
            .iter_tuple_segments()
            .flat_map(|(pt1, pt2)| {
                // Three points are said to be collinear iff:
                //
                // At least one point falls within both of the dimensional bounds (x and y bounds) of the other two
                // points.
                //
                // The distance of a to b is equal to the distance from ac + bc where a,b,c represent the points
                if ((pt.x >= pt1.x && pt.x <= pt2.x) || (pt.x <= pt1.x && pt.x >= pt2.x))
                    && ((pt.y >= pt1.y && pt.y <= pt2.y) || (pt.y <= pt1.y && pt.y >= pt2.y))
                {
                    let eps = T::from(f64::EPSILON).unwrap();
                    let diff = pt1.distance(&pt).unwrap() + pt2.distance(&pt).unwrap()
                        - pt1.distance(&pt2).unwrap();
                    let collinear = diff <= eps && diff >= -eps;
                    if collinear {
                        success = true;
                        vec![pt1, pt]
                    } else {
                        vec![pt1]
                    }
                } else {
                    vec![pt1]
                }
            })
            .collect::<Vec<Point<T>>>();
        if success {
            Polygon::new(vertices)
        } else {
            Err(GeometryError::MutationFailure(
                "Failed to insert vertex into polygon.".to_string(),
            ))
        }
    }

    /// Find the perimeter of a polygon
    ///
    /// Walk edges and sum distances for the Euclidean Space
    pub fn perimeter(&self) -> Result<T, MathError> {
        Ok(self
            .iter_tuple_segments()
            .fold(T::from(0f64).unwrap(), |sum, seg| {
                sum + seg.0.distance(&seg.1).unwrap()
            }))
    }

    /// Calculate the area of a Regular Polygon
    pub fn area(&self) -> Result<T, MathError> {
        // http://paulbourke.net/geometry/polygonmesh/
        Ok(self
            .iter_tuple_segments()
            .fold(T::from(0f64).unwrap(), |sum, seg| {
                sum + ((seg.0.x * seg.1.y) - (seg.1.x * seg.0.y))
            })
            * T::from(0.5f64).unwrap())
    }

    /// Calculate the area of a Regular Polygon
    pub fn orientation(&self) -> Result<PointOrder, MathError> {
        let area = self.area()?;
        if area + T::from(f64::EPSILON).unwrap() > T::from(0f64).unwrap() {
            Ok(PointOrder::Clockwise)
        } else {
            Ok(PointOrder::CounterClockwise)
        }
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
impl<T: Floats> From<Rectangle<T>> for Polygon<T> {
    fn from(rect: Rectangle<T>) -> Self {
        Polygon {
            vertices: rect.vertices.to_vec(),
        }
    }
}

// Operations ------------------------------------------------------------------
// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_new_invalid_polygon() {
        let _p = Polygon::new(vec![Point::new(1f64, 1f64), Point::new(1f64, 1f64)]).unwrap();
    }

    #[test]
    fn test_new_valid_polygon() {
        let vertices = vec![
            Point::new(1f64, 1f64),
            Point::new(1f64, 1f64),
            Point::new(1f64, 1f64),
        ];
        let p = Polygon::new(vertices.clone()).unwrap();
        assert_eq!(p, Polygon { vertices })
    }

    #[test]
    fn test_polygon_perimeter() {
        let vertices = vec![
            Point::new(1f64, 1f64),
            Point::new(1f64, -1f64),
            Point::new(3f64, -1f64),
            Point::new(3f64, 1f64),
        ];
        let p = Polygon::new(vertices).unwrap();
        assert_eq!(p.perimeter().unwrap(), 8f64)
    }

    #[test]
    fn test_polygon_area() {
        let vertices = vec![
            Point::new(1f64, 1f64),
            Point::new(1f64, -1f64),
            Point::new(3f64, -1f64),
            Point::new(3f64, 1f64),
        ];
        let p = Polygon::new(vertices).unwrap();
        assert_eq!(p.area().unwrap(), 4f64)
    }

    #[test]
    fn test_insert_vertex() {
        let in_pentagon_pt = vec![
            Point::new(0f64, 0f64),
            Point::new(15f64, 0f64),
            Point::new(25f64, 10f64),
            Point::new(10f64, 15f64),
            Point::new(-5f64, 10f64),
        ];

        let out_pentagon_pt = vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(15f64, 0f64),
            Point::new(25f64, 10f64),
            Point::new(10f64, 15f64),
            Point::new(-5f64, 10f64),
        ];

        let in_rectangle_pt = vec![
            Point::new(0.00000, 0.0f64),
            Point::new(0.00000, 10.0f64),
            Point::new(10.00000, 10.0f64),
            Point::new(10.00000, 0.0f64),
        ];

        let out_rectangle_pt = vec![
            Point::new(0.00000, 0.0f64),
            Point::new(0.00000, 10.0f64),
            Point::new(10.00000, 10.0f64),
            Point::new(10.00000, 5.0f64),
            Point::new(10.00000, 0.0f64),
        ];

        let in_pentagon = Polygon::new(in_pentagon_pt).unwrap();
        let out_pentagon = Polygon::new(out_pentagon_pt).unwrap();
        let in_rectangle = Polygon::new(in_rectangle_pt).unwrap();
        let out_rectangle = Polygon::new(out_rectangle_pt).unwrap();

        assert_eq!(
            out_rectangle,
            in_rectangle.insert_vertex(Point::new(10f64, 5f64)).unwrap()
        );
        assert_eq!(
            out_pentagon,
            in_pentagon.insert_vertex(Point::new(10f64, 0f64)).unwrap()
        );
    }
}
