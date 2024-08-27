use crate::metric::{
    Chebyshev, Cityblock, Euclidean, Hamming, Minkowski, SqEuclidean, TotalVariation,
};
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

impl<T: num_traits::Float> Distance<T> for Cityblock {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let abs_diff = diff.mapv(|a| a.abs());
        abs_diff.sum_axis(axis)
    }
}

impl<T: num_traits::Float> Distance<T> for TotalVariation {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let abs_diff = diff.mapv(|a| a.abs());
        let sum = abs_diff.sum_axis(axis);
        sum.mapv(|a| a * T::from(0.5).unwrap())
    }
}

impl<T: num_traits::Float> Distance<T> for Minkowski {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.mapv(|a| a.abs().powf(T::from(self.p).unwrap()));
        let sum = square.sum_axis(axis);
        sum.mapv(|a| a.powf(T::from(1.0 / self.p).unwrap()))
    }
}

impl<T: num_traits::Float> Distance<T> for Chebyshev {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let abs_diff = diff.mapv(|a| a.abs());
        abs_diff.fold_axis(axis, T::zero(), |a, b| a.max(*b))
    }
}

impl<T: num_traits::Float> Distance<T> for Hamming {
    fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let x_neq_y = ArrayD::from_shape_fn(x.raw_dim(), |idx| {
            let idx_copy = idx.clone();
            let x_idx = x.get(idx).unwrap();
            let y_idx = y.get(idx_copy).unwrap();
            match x_idx != y_idx {
                true => T::one(),
                false => T::zero(),
            }
        });
        x_neq_y.sum_axis(axis)
    }
}
