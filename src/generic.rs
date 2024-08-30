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


pub mod pre_metric {
    impl_metric! {
        KLDivergence,
        GenKLDivergence,
        NormRMSDeviation,
    }
}
pub mod semi_metric {
    impl_metric! {
        SqEuclidean,
        BrayCurtis,
        ChiSqDist,
        JSDivergence,
        SpanNormDist,
        MeanAbsDeviation,
        MeanSqDeviation,
    }
}
pub mod metric {
    impl_metric! {
        Chebyshev,
        Cityblock,
        CosineDist,
        Euclidean,
        Hamming,
        Jaccard,
        RMSDeviation,
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
}
