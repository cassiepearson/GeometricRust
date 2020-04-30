use inline_python::python;

use crate::{
    traits::numbers::{Floats, Integers},
    types::point::Point,
};

// Trait -----------------------------------------------------------------------
pub trait FloatPlot<T> {
    fn plot(&self);
}

pub trait IntPlot<T> {
    fn int_plot(&self);
}

// Implementation -------------------------------------------------------------
impl<T: Floats> FloatPlot<T> for Vec<Point<T>> {
    fn plot(&self) {
        let x_values: Vec<f64> = x_iter(self).collect();
        let y_values: Vec<f64> = y_iter(self).collect();

        python! {
            import plotly.graph_objects as go

            fig = go.Figure(go.Scatter(x='x_values, y='y_values, fill="toself"))
            fig.show()
        }
    }
}

impl<T: Integers> IntPlot<T> for Vec<Point<T>> {
    fn int_plot(&self) {
        let x_values: Vec<i64> = int_x_iter(self).collect();
        let y_values: Vec<i64> = int_y_iter(self).collect();

        python! {
            import plotly.graph_objects as go

            fig = go.Figure(go.Scatter(x='x_values, y='y_values, fill="toself"))
            fig.show()
        }
    }
}

// Iterators ------------------------------------------------------------------
pub fn x_iter<'a, I: Floats>(points: &'a Vec<Point<I>>) -> impl Iterator<Item = f64> + 'a {
    points.iter().map(|pt| pt.cast::<f64>().unwrap().x)
}

pub fn y_iter<'a, I: Floats>(points: &'a Vec<Point<I>>) -> impl Iterator<Item = f64> + 'a {
    points.iter().map(|pt| pt.cast::<f64>().unwrap().y)
}

pub fn int_x_iter<'a, I: Integers>(points: &'a Vec<Point<I>>) -> impl Iterator<Item = i64> + 'a {
    points.iter().map(|pt| pt.cast::<i64>().unwrap().x)
}

pub fn int_y_iter<'a, I: Integers>(points: &'a Vec<Point<I>>) -> impl Iterator<Item = i64> + 'a {
    points.iter().map(|pt| pt.cast::<i64>().unwrap().y)
}

// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::generator::*;

    #[test]
    // #[ignore]
    fn test_float_rect_plot() {
        let pt = Point { x: 0f64, y: 0f64 };
        let rect = pt.generate_rectangle(-25f64).to_vec();
        assert!(rect.len() != 0);
        rect.plot();
    }

    // fn test_float_circle_plot() {
    //     let pt = Point { x: 0f64, y: 0f64 };
    //     let circle = pt.generate_circle(-25f64).to_polygon(10).to_vec();
    //     assert!(circle.len() != 0);
    //     circle.plot();
    // }
}
