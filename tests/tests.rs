#[cfg(test)]
mod test {
    use faasle_rs::distance::Distance;
    use faasle_rs::metric::Euclidean;
    use ndarray::{Array, Axis};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;

    macro_rules! test_zero_distance {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            paste::paste! {
                #[test]
                fn [<zero_distance_$name>]() {
                    let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let distance = $metric.distance(&x, &x, $axis);
                    assert!(distance.iter().all(|&d| d == 0.));
                }
            }
        };
    }

    macro_rules! test_positivity {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            paste::paste! {
                #[test]
                fn [<positivity_$name>]() {
                    let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let y = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let distance = $metric.distance(&x, &y, $axis);
                    assert!(distance.iter().all(|&d| d >= 0.));
                }
            }
        };
    }

    macro_rules! test_symmetry {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            paste::paste! {
                #[test]
                fn [<symmetry_$name>]() {
                    let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let y = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let distance_x_y = $metric.distance(&x, &y, $axis);
                    let distance_y_x = $metric.distance(&y, &x, $axis);
                    assert_eq!(distance_x_y, distance_y_x);
                }
            }
        };
    }

    macro_rules! test_triangular_inequality {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            paste::paste! {
                    #[test]
                    fn [<triangular_inequality_$name>]() {
                    let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let y = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let z = Array::random($shape, Uniform::new(-10.0, 10.0));
                    let distance_x_y = $metric.distance(&x, &y, $axis);
                    let distance_x_z = $metric.distance(&x, &z, $axis);
                    let distance_y_z = $metric.distance(&y, &z, $axis);
                    assert!(distance_x_y
                        .iter()
                        .zip(distance_x_z.iter().zip(distance_y_z.iter()))
                        .all(|(dxy, (dxz, dyz))| *dxy <= dxz + dyz));
                }
            }
        };
    }

    macro_rules! PreMetricTest {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            test_zero_distance!($shape, $axis, $metric, $name);
            test_positivity!($shape, $axis, $metric, $name);
        };
    }

    macro_rules! SemiMetricTest {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            PreMetricTest!($shape, $axis, $metric, $name);
            test_symmetry!($shape, $axis, $metric, $name);
        };
    }

    macro_rules! MetricTest {
        ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
            SemiMetricTest!($shape, $axis, $metric, $name);
            test_triangular_inequality!($shape, $axis, $metric, $name);
        };
    }

    MetricTest!(
        vec![10, 8, 6, 4, 2],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_10_8_6_4_2_axis_2
    );
    MetricTest!(
        vec![100,],
        Axis(0),
        Euclidean::new(),
        euclidean_shape_100_axis_0
    );
    MetricTest!(
        vec![11, 11],
        Axis(0),
        Euclidean::new(),
        euclidean_shape_11_11_axis_0
    );
    MetricTest!(
        vec![12, 12, 12],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_12_12_12_axis_2
    );
    MetricTest!(
        vec![13, 4, 6, 19],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_13_4_6_19_axis_1
    );
    MetricTest!(
        vec![14, 14, 2],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_14_14_2_axis_1
    );
    MetricTest!(
        vec![15, 4],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_15_4_axis_1
    );
    MetricTest!(
        vec![16, 5, 17],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_16_5_17_axis_2
    );
    MetricTest!(
        vec![17, 2],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_17_2_axis_1
    );
    MetricTest!(
        vec![18, 12, 6, 3],
        Axis(3),
        Euclidean::new(),
        euclidean_shape_18_12_6_3_axis_3
    );
    MetricTest!(
        vec![19, 6, 4, 14],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_19_6_4_14_axis_1
    );
    MetricTest!(
        vec![19, 6, 4, 14],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_19_6_4_14_axis_2
    );
    MetricTest!(
        vec![19, 6, 4, 14],
        Axis(3),
        Euclidean::new(),
        euclidean_shape_19_6_4_14_axis_3
    );
    MetricTest!(
        vec![20, 30, 10, 5, 3],
        Axis(4),
        Euclidean::new(),
        euclidean_shape_20_30_10_5_3_axis_4
    );
    MetricTest!(
        vec![22, 10],
        Axis(0),
        Euclidean::new(),
        euclidean_shape_22_10_axis_0
    );
    MetricTest!(
        vec![25, 15, 5],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_25_15_5_axis_2
    );
    MetricTest!(
        vec![25, 3, 7, 9],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_25_3_7_9_axis_1
    );
    MetricTest!(
        vec![4, 6, 8, 2, 9, 5],
        Axis(3),
        Euclidean::new(),
        euclidean_shape_4_6_8_2_9_5_axis_3
    );
    MetricTest!(
        vec![40, 5, 5],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_40_5_5_axis_1
    );
    MetricTest!(
        vec![5, 5, 5, 5, 5],
        Axis(4),
        Euclidean::new(),
        euclidean_shape_5_5_5_5_5_axis_4
    );
    MetricTest!(
        vec![5, 7, 3, 9, 4],
        Axis(4),
        Euclidean::new(),
        euclidean_shape_5_7_3_9_4_axis_4
    );
    MetricTest!(
        vec![50, 50, 50, 2],
        Axis(3),
        Euclidean::new(),
        euclidean_shape_50_50_50_2_axis_3
    );
    MetricTest!(
        vec![6, 4, 2, 1],
        Axis(3),
        Euclidean::new(),
        euclidean_shape_6_4_2_1_axis_3
    );
    MetricTest!(
        vec![7,],
        Axis(0),
        Euclidean::new(),
        euclidean_shape_7_axis_0
    );
    MetricTest!(
        vec![8, 15, 7],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_8_15_7_axis_2
    );
    MetricTest!(
        vec![8, 8, 8, 8],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_8_8_8_8_axis_1
    );
    MetricTest!(
        vec![9, 1],
        Axis(1),
        Euclidean::new(),
        euclidean_shape_9_1_axis_1
    );
    MetricTest!(
        vec![9, 2],
        Axis(0),
        Euclidean::new(),
        euclidean_shape_9_2_axis_0
    );
    MetricTest!(
        vec![9, 3, 7, 11],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_9_3_7_11_axis_2
    );
    MetricTest!(
        vec![9, 5, 3],
        Axis(2),
        Euclidean::new(),
        euclidean_shape_9_5_3_axis_2
    );
}
