#[macro_export]
macro_rules! test_zero_distance {
    ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
        paste::paste! {
            #[test]
            fn [<zero_distance_$name>]() {
                let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                let distance = $metric.evaluate(&x, &x, $axis).unwrap();
                assert!(distance.iter().all(|&d| d == 0.));
            }
        }
    };
}

#[macro_export]
macro_rules! test_positivity {
    ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
        paste::paste! {
            #[test]
            fn [<positivity_$name>]() {
                let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                let y = Array::random($shape, Uniform::new(-10.0, 10.0));
                let distance = $metric.evaluate(&x, &y, $axis).unwrap();
                assert!(distance.iter().all(|&d| d >= 0.));
            }
        }
    };
}

#[macro_export]
macro_rules! test_symmetry {
    ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
        paste::paste! {
            #[test]
            fn [<symmetry_$name>]() {
                let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                let y = Array::random($shape, Uniform::new(-10.0, 10.0));
                let distance_x_y = $metric.evaluate(&x, &y, $axis).unwrap();
                let distance_y_x = $metric.evaluate(&y, &x, $axis).unwrap();
                assert_eq!(distance_x_y, distance_y_x);
            }
        }
    };
}

#[macro_export]
macro_rules! test_triangular_inequality {
    ($shape:expr, $axis:expr, $metric:expr, $name:ident) => {
        paste::paste! {
            #[test]
            fn [<triangular_inequality_$name>]() {
                let x = Array::random($shape, Uniform::new(-10.0, 10.0));
                let y = Array::random($shape, Uniform::new(-10.0, 10.0));
                let z = Array::random($shape, Uniform::new(-10.0, 10.0));
                let distance_x_y = $metric.evaluate(&x, &y, $axis).unwrap();
                let distance_x_z = $metric.evaluate(&x, &z, $axis).unwrap();
                let distance_y_z = $metric.evaluate(&y, &z, $axis).unwrap();
                const ERR_MARGIN: f64 = 1e-6;
                assert!(distance_x_y
                    .iter()
                    .zip(distance_x_z.iter().zip(distance_y_z.iter()))
                    .all(|(dxy, (dxz, dyz))| (*dxy + ERR_MARGIN <= dxz + dyz) || (*dxy - ERR_MARGIN<= dxz + dyz)));
                assert!(distance_x_z
                    .iter()
                    .zip(distance_x_y.iter().zip(distance_y_z.iter()))
                    .all(|(dxz, (dxy, dyz))| (*dxz + ERR_MARGIN <= dxy + dyz) || (*dxz - ERR_MARGIN<= dxy + dyz)));
                assert!(distance_y_z
                    .iter()
                    .zip(distance_x_y.iter().zip(distance_x_z.iter()))
                    .all(|(dyz, (dxy, dxz))| (*dyz + ERR_MARGIN <= dxy + dxz) || (*dyz - ERR_MARGIN<= dxy + dxz)));
                }
            }
        };
    }

#[macro_export]
macro_rules! pre_metric_test {
    ($($name:ident: ($shape:expr, $axis:expr, $metric:expr),)*) => {
        $(
            test_zero_distance!($shape, $axis, $metric, $name);
            test_positivity!($shape, $axis, $metric, $name);
        )*
    };
}

#[macro_export]
macro_rules! semi_metric_test {
    ($($name:ident: ($shape:expr, $axis:expr, $metric:expr),)*) => {
        pre_metric_test! {
            $(
                $name: ($shape, $axis, $metric),
            )*
        }
        $(
            test_symmetry!($shape, $axis, $metric, $name);
        )*
    };
}

#[macro_export]
macro_rules! metric_test {
    ($($name:ident: ($shape:expr, $axis:expr, $metric:expr),)*) => {
        semi_metric_test!{
            $(
                $name: ($shape, $axis, $metric),
            )*
        }
        $(
            test_triangular_inequality!($shape, $axis, $metric, $name);
        )*
    };
}
