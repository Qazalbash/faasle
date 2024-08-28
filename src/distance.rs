use crate::metric::{
    BrayCurtis, Chebyshev, ChiSqDist, Cityblock, Euclidean, Hamming, KLDivergence, Minkowski,
    SqEuclidean, TotalVariation,
};
use ndarray::{ArrayD, Axis};
pub trait Distance<T>
where
    T: num_traits::Float,
{
    /// # Safety
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T>;
    fn evaluate(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> Result<ArrayD<T>, String> {
        if x.ndim() != y.ndim() {
            return Err("x and y must have the same number of dimensions".to_string());
        }
        if x.shape() != y.shape() {
            return Err("x and y must have the same shape".to_string());
        }
        Ok(unsafe { self.distance(x, y, axis) })
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Euclidean {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.pow2();
        let sum = square.sum_axis(axis);
        sum.sqrt()
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for SqEuclidean {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.pow2();
        square.sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Cityblock {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let abs_diff = diff.abs();
        abs_diff.sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float + ndarray::ScalarOperand> Distance<T> for TotalVariation {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let abs_diff = diff.abs();
        let sum = abs_diff.sum_axis(axis);
        sum * T::from(0.5).unwrap()
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Minkowski {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let square = diff.abs().powf(T::from(self.p).unwrap());
        let sum = square.sum_axis(axis);
        sum.powf(T::from(1.0 / self.p).unwrap())
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Chebyshev {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let abs_diff = diff.abs();
        abs_diff.fold_axis(axis, T::zero(), |a, b| a.max(*b))
    }
}

impl<T: num_traits::Float> Distance<T> for Hamming {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
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

impl<T: 'static + num_traits::Float> Distance<T> for BrayCurtis {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff = x - y;
        let sum = x + y;
        let abs_diff = diff.abs();
        let abs_sum = sum.abs();
        let sum_abs_diff = abs_diff.sum_axis(axis);
        let sum_abs_sum = abs_sum.sum_axis(axis);
        sum_abs_diff / sum_abs_sum
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for ChiSqDist {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let diff_sq = (x - y).pow2();
        let sum = x + y;
        let sum_diff_sq = diff_sq / sum;
        sum_diff_sq.sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for KLDivergence {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let log_x = x.ln();
        let log_y = y.ln();
        let parital_kld = x * (log_x - log_y);
        parital_kld.sum_axis(axis)
    }
}
