mod common;
use faasle_rs::distance::Distance;
use faasle_rs::metric::{Cityblock, Euclidean, SqEuclidean, TotalVariation};
use ndarray::{Array, Axis};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

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
    metric: (cityblock, Cityblock::new()),
    metric: (euclidean, Euclidean::new()),
    metric: (total_variation, TotalVariation::new()),
    semi_metric: (sq_euclidean, SqEuclidean::new()),
}
