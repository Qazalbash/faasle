use crate::generic::Distance;
use ndarray::{ArrayD, Axis};

pub struct Euclidean {}

impl Euclidean {
    pub fn new() -> Self {
        Euclidean {}
    }
}

impl<T: num_traits::Float> Distance<T> for Euclidean {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.mapv(|a| a * a);
        let sum = square.sum_axis(axis);
        let result = sum.mapv(|a| a.sqrt());
        result
    }
}
