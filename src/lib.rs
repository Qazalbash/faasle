//! faasle-rs[^1] is a Rust package for evaluating distances (metrics) between
//! multidimensional arrays. It is designed to be simple, fast, and easy to use.
//!
//! ## How to cite?
//!
//! ```bibtex
//! @software{faaslers2024github,
//!     author = {{M}eesum {Q}azalbash},
//!     title = {{faasle-rs}: A Rust package for evaluating distances (metrics).},
//!     url = {https://github.com/Qazalbash/faasle-rs},
//!     version = {0.0.1},
//!     year = {2024}
//! }
//! ```
//!
//! [^1]: Faasle is an Urdu word, which [means][Faasle meaning]
//! distance in English. It is also the name of the infamous Coke Studio Season 10
//! [song][Faasle song] by Quratulain Balouch and Kaavish.
//!
//! [Faasle song]: https://www.youtube.com/watch?v=9sekgEXGm-E
//! [Faasle meaning]: https://www.rekhta.org/urdudictionary?keyword=faasle&lang=eng
mod distance;
mod generic;

/// The `Distance` trait defines the interface for evaluating distances (metrics)
/// between multidimensional arrays.
pub use distance::Distance;
/// `Chebyshev(x, y) = max(|x - y|)`
pub use generic::metric::Chebyshev;
/// `Cityblock(x, y) = sum(|x - y|)`
pub use generic::metric::Cityblock;
/// `Euclidean(x, y) = sqrt(sum((x - y).^2))`
pub use generic::metric::Euclidean;
/// `Hamming(x, y) = sum(x!=y)`
pub use generic::metric::Hamming;
/// `Minkowski(x, y) = sum(|x - y|.^p).^(1/p)`
pub use generic::metric::Minkowski;
/// `RMSDeviation(x, y) = sqrt(mean((x - y).^2))`
pub use generic::metric::RMSDeviation;
/// `TotalVariation(x, y) = sum(|x - y|) ./ 2`
pub use generic::metric::TotalVariation;
/// `GenKLDivergence(x, y) = sum(x .* ln(x ./ y) - x + y)`
pub use generic::pre_metric::GenKLDivergence;
/// `KLDivergence(x, y) = sum(x .* ln(x ./ y))`
pub use generic::pre_metric::KLDivergence;
/// `BrayCurtis(x, y) = sum(|x - y|) / sum(|x + y|)`
pub use generic::semi_metric::BrayCurtis;
/// `ChiSqDist(x, y) = sum((x - y).^2 ./ (x + y))`
pub use generic::semi_metric::ChiSqDist;
/// `JSDivergence(x, y) = KLDivergence(x, (x + y) ./ 2) + KLDivergence(y, (x + y) ./ 2)`
pub use generic::semi_metric::JSDivergence;
/// `MeanAbsDeviation(x, y) = mean(|x - y|)`
pub use generic::semi_metric::MeanAbsDeviation;
/// `MeanSqDeviation(x, y) = mean((x - y).^2)`
pub use generic::semi_metric::MeanSqDeviation;
/// `SqEuclidean(x, y) = sqrt(sum((x - y).^2))`
pub use generic::semi_metric::SqEuclidean;
