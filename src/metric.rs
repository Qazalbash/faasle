macro_rules! impl_metric {
    ($($metric:ident,)*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $metric;

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

#[derive(Debug, Clone, Copy)]
pub struct Minkowski {
    pub p: f64,
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
