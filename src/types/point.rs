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
