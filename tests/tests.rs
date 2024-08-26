#[cfg(test)]
mod test {
    use faasle_rs::distance::Distance;
    use faasle_rs::metric::Euclidean;
    use ndarray::{Array, Axis};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[rstest]
    #[case(vec![2, 3], Axis(1))]
    #[case(vec![2, 1, 4], Axis(0))]
    fn test_positivity(#[case] shape: Vec<usize>, #[case] axis: Axis) {
        let x = Array::random(shape, Uniform::new(0., 10.));
        let euclidean = Euclidean::new();
        let distance = euclidean.distance(&x, &x, axis);
        assert!(distance.iter().all(|&d| d >= 0.));
    }
}
