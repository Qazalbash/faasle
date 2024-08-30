# faasle-rs

faasle-rs[^1] is a Rust package for evaluating distances (metrics) between
multidimensional arrays. It is designed to be simple, fast, and easy to use.

## Usage

```rust
use faasle_rs::{Distance, Euclidean};
use ndarray::{ArrayD, Axis};

let x =
ArrayD::from_shape_vec(vec![2, 4], vec![0.0, 3.0, 3.0, 5.0, 11.0, 2.0, 0.0, 9.0]).unwrap();
let y =
ArrayD::from_shape_vec(vec![2, 4], vec![9.0, 2.0, 2.0, 1.0, 9.0, 5.0, 4.0, 7.0]).unwrap();
let metric = Euclidean::new();
let distance = metric.evaluate( & x, & y, Axis(1)).unwrap();
assert!(distance.abs_diff_eq(
    &ArrayD::from_shape_vec(vec![2], vec![9.9498743710662, 5.744562646538029]).unwrap(),
    1e-6
));
```

## How to cite?

```bibtex
@software{faaslers2024github,
    author = {{M}eesum {Q}azalbash},
    title = {{faasle-rs}: A Rust package for evaluating distances (metrics).},
    url = {https://github.com/Qazalbash/faasle-rs},
    version = {0.0.1},
    year = {2024}
}
```

[^1]: Faasle is an Urdu word, which [means][Faasle meaning]
distance in English. It is also the name of the infamous Coke Studio Season 10
[song][Faasle song] by Quratulain Balouch and Kaavish.

[Faasle song]: https://www.youtube.com/watch?v=9sekgEXGm-E

[Faasle meaning]: https://www.rekhta.org/urdudictionary?keyword=faasle&lang=eng