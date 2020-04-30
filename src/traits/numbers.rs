//! Number Trait - For more convenient trait bounding
use cgmath::{BaseFloat, BaseNum};
use num::{Bounded, FromPrimitive, ToPrimitive};
use rand::distributions::uniform::SampleUniform;

pub trait Integers:
    BaseNum + ToPrimitive + FromPrimitive + SampleUniform + Bounded + Send + Sync
{
}

impl<T> Integers for T where
    T: BaseNum + ToPrimitive + FromPrimitive + SampleUniform + Bounded + Send + Sync
{
}

pub trait Floats:
    BaseFloat + ToPrimitive + FromPrimitive + SampleUniform + Copy + Send + Sync
{
}

impl<T> Floats for T where
    T: BaseFloat + ToPrimitive + FromPrimitive + SampleUniform + Copy + Send + Sync
{
}
