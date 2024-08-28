#[derive(Debug, Clone, Copy)]
pub struct BrayCurtis;

#[derive(Debug, Clone, Copy)]
pub struct Chebyshev;

#[derive(Debug, Clone, Copy)]
pub struct ChiSqDist;

#[derive(Debug, Clone, Copy)]
pub struct Cityblock;

#[derive(Debug, Clone, Copy)]
pub struct Euclidean;

#[derive(Debug, Clone, Copy)]
pub struct GenKLDivergence;

#[derive(Debug, Clone, Copy)]
pub struct Hamming;

#[derive(Debug, Clone, Copy)]
pub struct JSDivergence;

#[derive(Debug, Clone, Copy)]
pub struct KLDivergence;

#[derive(Debug, Clone, Copy)]
pub struct MeanAbsDeviation;

#[derive(Debug, Clone, Copy)]
pub struct MeanSqDeviation;

#[derive(Debug, Clone, Copy)]
pub struct Minkowski {
    pub p: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct RMSDeviation;

#[derive(Debug, Clone, Copy)]
pub struct SqEuclidean;

#[derive(Debug, Clone, Copy)]
pub struct TotalVariation;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct BhattacharyyaDist;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Bregman;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct CorrDist;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct CosineDist;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Haversine;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct HellingerDist;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Jaccard;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Mahalanobis;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct NormRMSDeviation;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct PeriodicEuclidean;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct RenyiDivergence;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct RogersTanimoto;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct SpanNormDist;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct SphericalAngle;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct SqMahalanobis;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct WeightedCityblock;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct WeightedEuclidean;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct WeightedHamming;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct WeightedMinkowski;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct WeightedSqEuclidean;

macro_rules! impl_metric {
    ($($metric:ident,)*) => {
        $(
            impl $metric {
                pub fn new() -> Self {
                    Self {}
                }
            }

            impl Default for $metric {
                fn default() -> Self {
                    Self::new()
                }
            }
        )*
    };
}

impl_metric! {
    BrayCurtis,
    Chebyshev,
    ChiSqDist,
    Cityblock,
    CosineDist,
    Euclidean,
    GenKLDivergence,
    Hamming,
    Jaccard,
    JSDivergence,
    KLDivergence,
    MeanAbsDeviation,
    MeanSqDeviation,
    NormRMSDeviation,
    RMSDeviation,
    SpanNormDist,
    SqEuclidean,
    TotalVariation,
}

impl Minkowski {
    pub fn new(p: f64) -> Self {
        Self { p }
    }
}

impl Default for Minkowski {
    fn default() -> Self {
        Self::new(1.0)
    }
}
