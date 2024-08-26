use ndarray::{ArrayD, Axis};

pub trait Distance<T>
where
    T: num_traits::Float,
{
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T>;
}

#[derive(Debug, Copy, Clone)]
pub struct PreMetric {}

#[derive(Debug, Copy, Clone)]
pub struct SemiMetric {}

#[derive(Debug, Copy, Clone)]
pub struct Metric {}
