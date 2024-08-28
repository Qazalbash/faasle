use crate::metric::{
    BrayCurtis, Chebyshev, ChiSqDist, Cityblock, Euclidean, GenKLDivergence, Hamming, JSDivergence,
    KLDivergence, MeanAbsDeviation, MeanSqDeviation, Minkowski, RMSDeviation, SqEuclidean,
    TotalVariation,
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
        (x - y).pow2().sum_axis(axis).sqrt()
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for SqEuclidean {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).pow2().sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Cityblock {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).abs().sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float + ndarray::ScalarOperand> Distance<T> for TotalVariation {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).abs().sum_axis(axis) * T::from(0.5).unwrap()
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Minkowski {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y)
            .abs()
            .powf(T::from(self.p).unwrap())
            .sum_axis(axis)
            .powf(T::from(1.0 / self.p).unwrap())
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for Chebyshev {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).abs().fold_axis(axis, T::zero(), |a, b| a.max(*b))
    }
}

impl<T: num_traits::Float> Distance<T> for Hamming {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        ArrayD::from_shape_fn(x.raw_dim(), |idx| {
            let idx_copy = idx.clone();
            let x_idx = x.get(idx).unwrap();
            let y_idx = y.get(idx_copy).unwrap();
            match x_idx != y_idx {
                true => T::one(),
                false => T::zero(),
            }
        })
        .sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float> Distance<T> for BrayCurtis {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let sum_abs_diff = (x - y).abs().sum_axis(axis);
        let sum_abs_sum = (x + y).abs().sum_axis(axis);
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
        (x * (x.ln() - y.ln())).sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float + ndarray::ScalarOperand> Distance<T> for GenKLDivergence {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x * (x.ln() - y.ln() - T::one()) + y).sum_axis(axis)
    }
}

impl<T: 'static + num_traits::Float + ndarray::ScalarOperand> Distance<T> for JSDivergence {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        let m = (x + y) * T::from(0.5).unwrap();
        let kl_div_x_m = (x * (x.ln() - m.ln())).sum_axis(axis);
        let kl_div_y_m = (y * (y.ln() - m.ln())).sum_axis(axis);
        (kl_div_x_m + kl_div_y_m) * T::from(0.5).unwrap()
    }
}

impl<T: 'static + num_traits::Float + num_traits::FromPrimitive> Distance<T> for MeanAbsDeviation {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).abs().mean_axis(axis).unwrap()
    }
}

impl<T: 'static + num_traits::Float + num_traits::FromPrimitive> Distance<T> for MeanSqDeviation {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).pow2().mean_axis(axis).unwrap()
    }
}

impl<T: 'static + num_traits::Float + num_traits::FromPrimitive> Distance<T> for RMSDeviation {
    unsafe fn distance(&self, x: &ArrayD<T>, y: &ArrayD<T>, axis: Axis) -> ArrayD<T> {
        (x - y).pow2().mean_axis(axis).unwrap().sqrt()
    }
}
