use crate::metric::{Euclidean, SqEuclidean};
use ndarray::{ArrayD, Axis};
pub trait Distance<T>
where
    T: num_traits::Float,
{
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T>;
}

impl<T: num_traits::Float> Distance<T> for Euclidean {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.mapv(|a| a * a);
        let sum = square.sum_axis(axis);
        sum.mapv(|a| a.sqrt())
    }
}

impl<T: num_traits::Float> Distance<T> for SqEuclidean {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.mapv(|a| a * a);
        square.sum_axis(axis)
    }
}
