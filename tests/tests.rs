mod common;
use faasle_rs::{
    BrayCurtis, Chebyshev, ChiSqDist, Cityblock, Distance, Euclidean, GenKLDivergence, Hamming,
    JSDivergence, KLDivergence, MeanAbsDeviation, MeanSqDeviation, Minkowski, RMSDeviation,
    SqEuclidean, TotalVariation,
};
use ndarray::{Array, Axis};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
const ERR_MARGIN: f64 = 1e-6;

#[test]
fn unequal_shape() {
    let x = Array::random(vec![10, 8, 6, 4, 2], Uniform::new(0.0, 1.0));
    let y = Array::random(vec![10, 8, 6, 4, 3], Uniform::new(0.0, 1.0));
    let metric = Euclidean::new();
    let error = metric.evaluate(&x, &y, Axis(2)).unwrap_err();
    assert_eq!(error, "x and y must have the same shape");
}

#[test]
fn unequal_dimension() {
    let x = Array::random(vec![10, 8, 6, 4, 2], Uniform::new(0.0, 1.0));
    let y = Array::random(vec![10, 8, 6, 4], Uniform::new(0.0, 1.0));
    let metric = Euclidean::new();
    let error = metric.evaluate(&x, &y, Axis(2)).unwrap_err();
    assert_eq!(error, "x and y must have the same number of dimensions");
}

macro_rules! enumerate_tests {
    ($($metric_type:ident: ($name:ident, $metric:expr), )*) => {
        $(
            paste::paste! {
                [<$metric_type _test>]! {
                    [<shape_10_8_6_4_2_axis_2_metric_$name>] : (vec![10, 8, 6, 4, 2], Axis(2), $metric),
                    [<shape_100_axis_0_metric_$name>] : (vec![100,], Axis(0), $metric),
                    [<shape_11_11_axis_0_metric_$name>] : (vec![11, 11], Axis(0), $metric),
                    [<shape_12_12_12_axis_2_metric_$name>] : (vec![12, 12, 12], Axis(2), $metric),
                    [<shape_13_4_6_19_axis_1_metric_$name>] : (vec![13, 4, 6, 19], Axis(1), $metric),
                    [<shape_14_14_2_axis_1_metric_$name>] : (vec![14, 14, 2], Axis(1), $metric),
                    [<shape_15_4_axis_1_metric_$name>] : (vec![15, 4], Axis(1), $metric),
                    [<shape_16_5_17_axis_2_metric_$name>] : (vec![16, 5, 17], Axis(2), $metric),
                    [<shape_17_2_axis_1_metric_$name>] : (vec![17, 2], Axis(1), $metric),
                    [<shape_18_12_6_3_axis_3_metric_$name>] : (vec![18, 12, 6, 3], Axis(3), $metric),
                    [<shape_19_6_4_14_axis_1_metric_$name>] : (vec![19, 6, 4, 14], Axis(1), $metric),
                    [<shape_19_6_4_14_axis_2_metric_$name>] : (vec![19, 6, 4, 14], Axis(2), $metric),
                    [<shape_19_6_4_14_axis_3_metric_$name>] : (vec![19, 6, 4, 14], Axis(3), $metric),
                    [<shape_20_30_10_5_3_axis_4_metric_$name>] : (vec![20, 30, 10, 5, 3], Axis(4), $metric),
                    [<shape_22_10_axis_0_metric_$name>] : (vec![22, 10], Axis(0), $metric),
                    [<shape_25_15_5_axis_2_metric_$name>] : (vec![25, 15, 5], Axis(2), $metric),
                    [<shape_25_3_7_9_axis_1_metric_$name>] : (vec![25, 3, 7, 9], Axis(1), $metric),
                    [<shape_4_6_8_2_9_5_axis_3_metric_$name>] : (vec![4, 6, 8, 2, 9, 5], Axis(3), $metric),
                    [<shape_40_5_5_axis_1_metric_$name>] : (vec![40, 5, 5], Axis(1), $metric),
                    [<shape_5_5_5_5_5_axis_4_metric_$name>] : (vec![5, 5, 5, 5, 5], Axis(4), $metric),
                    [<shape_5_7_3_9_4_axis_4_metric_$name>] : (vec![5, 7, 3, 9, 4], Axis(4), $metric),
                    [<shape_50_50_50_2_axis_3_metric_$name>] : (vec![50, 50, 50, 2], Axis(3), $metric),
                    [<shape_6_4_2_1_axis_3_metric_$name>] : (vec![6, 4, 2, 1], Axis(3), $metric),
                    [<shape_7_axis_0_metric_$name>] : (vec![7,], Axis(0), $metric),
                    [<shape_8_15_7_axis_2_metric_$name>] : (vec![8, 15, 7], Axis(2), $metric),
                    [<shape_8_8_8_8_axis_1_metric_$name>] : (vec![8, 8, 8, 8], Axis(1), $metric),
                    [<shape_9_1_axis_1_metric_$name>] : (vec![9, 1], Axis(1), $metric),
                    [<shape_9_2_axis_0_metric_$name>] : (vec![9, 2], Axis(0), $metric),
                    [<shape_9_3_7_11_axis_2_metric_$name>] : (vec![9, 3, 7, 11], Axis(2), $metric),
                    [<shape_9_5_3_axis_2_metric_$name>] : (vec![9, 5, 3], Axis(2), $metric),
                }
            }
        )*
    };
}

enumerate_tests! {
    metric: (chebyshev, Chebyshev::new()),
    metric: (cityblock, Cityblock::new()),
    metric: (euclidean, Euclidean::new()),
    metric: (hamming, Hamming::new()),
    metric: (mean_abs_deviation, MeanAbsDeviation::new()),
    metric: (minkowski_1, Minkowski::new(1.0)),
    metric: (minkowski_10, Minkowski::new(10.0)),
    metric: (minkowski_3, Minkowski::new(3.0)),
    metric: (minkowski_e, Minkowski::new(std::f64::consts::E)),
    metric: (minkowski_pi, Minkowski::new(std::f64::consts::PI)),
    metric: (rms_deviation, RMSDeviation::new()),
    metric: (total_variation, TotalVariation::new()),
    pre_metric: (gen_kl_divergence, GenKLDivergence::new()),
    pre_metric: (kl_divergence, KLDivergence::new()),
    semi_metric: (bray_curtis, BrayCurtis::new()),
    semi_metric: (chi_sq_dist, ChiSqDist::new()),
    semi_metric: (js_divergence, JSDivergence::new()),
    semi_metric: (mean_sq_deviation, MeanSqDeviation::new()),
    semi_metric: (sq_euclidean, SqEuclidean::new()),
}
