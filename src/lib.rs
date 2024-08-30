#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/Qazalbash/faasle-rs/issues/")]
mod distance;
mod generic;

/// The `Distance` trait defines the interface for evaluating distances (metrics)
/// between multidimensional arrays.
#[doc(inline)]
pub use distance::Distance;
/// `Chebyshev(x, y) = max(|x - y|)`
#[doc(inline)]
pub use generic::metric::Chebyshev;
/// `Cityblock(x, y) = sum(|x - y|)`
#[doc(inline)]
pub use generic::metric::Cityblock;
/// `Euclidean(x, y) = sqrt(sum((x - y).^2))`
#[doc(inline)]
pub use generic::metric::Euclidean;
/// `Hamming(x, y) = sum(x!=y)`
#[doc(inline)]
pub use generic::metric::Hamming;
/// `Minkowski(x, y) = sum(|x - y|.^p).^(1/p)`
#[doc(inline)]
pub use generic::metric::Minkowski;
/// `RMSDeviation(x, y) = sqrt(mean((x - y).^2))`
#[doc(inline)]
pub use generic::metric::RMSDeviation;
/// `TotalVariation(x, y) = sum(|x - y|) ./ 2`
#[doc(inline)]
pub use generic::metric::TotalVariation;
/// `GenKLDivergence(x, y) = sum(x .* ln(x ./ y) - x + y)`
#[doc(inline)]
pub use generic::pre_metric::GenKLDivergence;
/// `KLDivergence(x, y) = sum(x .* ln(x ./ y))`
#[doc(inline)]
pub use generic::pre_metric::KLDivergence;
/// `BrayCurtis(x, y) = sum(|x - y|) / sum(|x + y|)`
#[doc(inline)]
pub use generic::semi_metric::BrayCurtis;
/// `ChiSqDist(x, y) = sum((x - y).^2 ./ (x + y))`
#[doc(inline)]
pub use generic::semi_metric::ChiSqDist;
/// `JSDivergence(x, y) = KLDivergence(x, (x + y) ./ 2) + KLDivergence(y, (x + y) ./ 2)`
#[doc(inline)]
pub use generic::semi_metric::JSDivergence;
/// `MeanAbsDeviation(x, y) = mean(|x - y|)`
#[doc(inline)]
pub use generic::semi_metric::MeanAbsDeviation;
/// `MeanSqDeviation(x, y) = mean((x - y).^2)`
#[doc(inline)]
pub use generic::semi_metric::MeanSqDeviation;
/// `SqEuclidean(x, y) = sqrt(sum((x - y).^2))`
#[doc(inline)]
pub use generic::semi_metric::SqEuclidean;
