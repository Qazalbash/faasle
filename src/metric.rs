#[derive(Debug, Clone, Copy)]
pub struct BhattacharyyaDist;

#[derive(Debug, Clone, Copy)]
pub struct BrayCurtis;

#[derive(Debug, Clone, Copy)]
pub struct Bregman;

#[derive(Debug, Clone, Copy)]
pub struct Chebyshev;

#[derive(Debug, Clone, Copy)]
pub struct ChiSqDist;

#[derive(Debug, Clone, Copy)]
pub struct Cityblock;

#[derive(Debug, Clone, Copy)]
pub struct CorrDist;

#[derive(Debug, Clone, Copy)]
pub struct CosineDist;

#[derive(Debug, Clone, Copy)]
pub struct Euclidean;

#[derive(Debug, Clone, Copy)]
pub struct GenKLDivergence;

#[derive(Debug, Clone, Copy)]
pub struct Hamming;

#[derive(Debug, Clone, Copy)]
pub struct Haversine;

#[derive(Debug, Clone, Copy)]
pub struct HellingerDist;

#[derive(Debug, Clone, Copy)]
pub struct Jaccard;

#[derive(Debug, Clone, Copy)]
pub struct JSDivergence;

#[derive(Debug, Clone, Copy)]
pub struct KLDivergence;

#[derive(Debug, Clone, Copy)]
pub struct Mahalanobis;

#[derive(Debug, Clone, Copy)]
pub struct MeanAbsDeviation;

#[derive(Debug, Clone, Copy)]
pub struct MeanSqDeviation;

#[derive(Debug, Clone, Copy)]
pub struct Minkowski {
    pub p: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct NormRMSDeviation;

#[derive(Debug, Clone, Copy)]
pub struct PeriodicEuclidean;

#[derive(Debug, Clone, Copy)]
pub struct RenyiDivergence;

#[derive(Debug, Clone, Copy)]
pub struct RMSDeviation;

#[derive(Debug, Clone, Copy)]
pub struct RogersTanimoto;

#[derive(Debug, Clone, Copy)]
pub struct SpanNormDist;

#[derive(Debug, Clone, Copy)]
pub struct SphericalAngle;

#[derive(Debug, Clone, Copy)]
pub struct SqEuclidean;

#[derive(Debug, Clone, Copy)]
pub struct SqMahalanobis;

#[derive(Debug, Clone, Copy)]
pub struct TotalVariation;

#[derive(Debug, Clone, Copy)]
pub struct WeightedCityblock;

#[derive(Debug, Clone, Copy)]
pub struct WeightedEuclidean;

#[derive(Debug, Clone, Copy)]
pub struct WeightedHamming;

#[derive(Debug, Clone, Copy)]
pub struct WeightedMinkowski;

#[derive(Debug, Clone, Copy)]
pub struct WeightedSqEuclidean;

pub enum PreMetric {
    GenKLDivergence(GenKLDivergence),
    KLDivergence(KLDivergence),
    NormRMSDeviation(NormRMSDeviation),
}

pub enum SemiMetric {
    BrayCurtis(BrayCurtis),
    ChiSqDist(ChiSqDist),
    CosineDist(CosineDist),
    JSDivergence(JSDivergence),
    MeanSqDeviation(MeanSqDeviation),
    SpanNormDist(SpanNormDist),
    SqEuclidean(SqEuclidean),
    WeightedSqEuclidean(WeightedSqEuclidean),
}

pub enum Metric {
    Chebyshev(Chebyshev),
    Cityblock(Cityblock),
    Euclidean(Euclidean),
    Hamming(Hamming),
    Jaccard(Jaccard),
    MeanAbsDeviation(MeanAbsDeviation),
    Minkowski(Minkowski),
    RMSDeviation(RMSDeviation),
    TotalVariation(TotalVariation),
}

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
