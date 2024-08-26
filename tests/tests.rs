#[cfg(test)]
mod test {
    use faasle_rs::generic::Distance;
    use faasle_rs::metric::Euclidean;
    use ndarray::{ArrayD, Axis};

    #[test]
    fn test_positivity() {
        let x = ArrayD::<f64>::from_shape_vec(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let euclidean = Euclidean::new();
        let result = euclidean.distance(&x, &x, Axis(1));
        assert_eq!(
            result,
            ArrayD::<f64>::from_shape_vec(vec![2], vec![0.0, 0.0]).unwrap()
        );
    }
}
