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
mod metric;

/// The `Distance` trait defines the interface for evaluating distances (metrics)
/// between multidimensional arrays.
pub use distance::Distance;
pub use metric::BrayCurtis;
pub use metric::Chebyshev;
pub use metric::ChiSqDist;
pub use metric::Cityblock;
pub use metric::Euclidean;
pub use metric::GenKLDivergence;
pub use metric::Hamming;
pub use metric::JSDivergence;
pub use metric::KLDivergence;
pub use metric::MeanAbsDeviation;
pub use metric::MeanSqDeviation;
pub use metric::Minkowski;
pub use metric::RMSDeviation;
pub use metric::SqEuclidean;
pub use metric::TotalVariation;
