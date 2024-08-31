# `faasle`

`faasle`[^1] is a Rust package for evaluating distances (metrics) between multidimensional arrays. It is designed to be simple, fast, and easy to use.

## Usage

```rust
use faasle::{Distance, Euclidean};
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

## Hierarchy of Types

Mathematically a distance metric is a function $d:\mathcal{X}\times\mathcal{X}\rightarrow\mathbb{R}$, where $\mathcal{X}$ is a set, such that they satisfy the following properties:

### Positivity

1. $d(x, y) \geq 0$ for all $x, y \in \mathcal{X}$,
2. $d(x, y) = 0$ if and only if $x = y$,

### Symmetry

3. $d(x, y) = d(y, x)$ for all $x, y \in \mathcal{X}$,

### Triangle Inequality

4. $d(x, z) \leq d(x, y) + d(y, z)$ for all $x, y, z \in \mathcal{X}$.

The hierarchy of types and their properties are as follows:

|                     | `PreMetric` | `SemiMetric` | `Metric` |
|---------------------|-------------|--------------|----------|
| Positivity          |      ✅     |      ✅      |    ✅    |
| Symmetry            |      ❌     |      ✅      |    ✅    |
| Triangle Inequality |      ❌     |      ❌      |    ✅    |

## How to cite?

```bibtex
@software{faaslers2024github,
    author = {{M}eesum {Q}azalbash},
    title = {{faasle}: Rust crate for evaluating distances (metrics).},
    url = {https://github.com/Qazalbash/faasle},
    version = {0.0.1},
    year = {2024}
}
```

[^1]: Faasle is an Urdu word, which [means][Faasle meaning]
distance in English. It is also the name of the infamous Coke Studio Season 10
[song][Faasle song] by Quratulain Balouch and Kaavish.

[Faasle song]: https://www.youtube.com/watch?v=9sekgEXGm-E

[Faasle meaning]: https://www.rekhta.org/urdudictionary?keyword=faasle&lang=eng
