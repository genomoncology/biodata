//! Additive operations retained by values whose storage has a separate grammar.
macro_rules! canonical_extras {
    ($ty:ty, $error:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        impl $ty {
            /// Every declared value, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
        }
        impl From<$ty> for &'static str {
            fn from(value: $ty) -> Self { match value { $(<$ty>::$variant => $text),+ } }
        }
        impl TryFrom<String> for $ty {
            type Error = crate::Error;
            fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
                value.parse().map_err(|_| crate::Error::$error)
            }
        }
    };
}
pub(crate) use canonical_extras;

macro_rules! canonical_parse {
    ($ty:ty, $error:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        impl std::str::FromStr for $ty {
            type Err = crate::Error;
            fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
                match value { $($text => Ok(Self::$variant),)+ _ => Err(crate::Error::$error) }
            }
        }
    };
}
pub(crate) use canonical_parse;

macro_rules! canonical_display {
    ($ty:ty) => {
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str((*self).into())
            }
        }
    };
}

macro_rules! canonical_value {
    ($ty:ty, $error:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        canonical_extras!($ty, $error { $($variant => $text),+ });
        canonical_parse!($ty, $error { $($variant => $text),+ });
        canonical_display!($ty);
    };
}
canonical_extras!(crate::Build, BuildUnknown { GRCh37 => "GRCh37", GRCh38 => "GRCh38" });
canonical_value!(crate::Strand, StrandUnknown { Plus => "+", Minus => "-" });
canonical_value!(crate::ManeStatus, ManeStatusUnknown { Select => "select", PlusClinical => "plus_clinical" });
canonical_value!(crate::ExpressionSyntax, ExpressionSyntaxUnknown {
    HgvsG => "hgvs.g", HgvsC => "hgvs.c", HgvsN => "hgvs.n", HgvsR => "hgvs.r",
    HgvsP => "hgvs.p", HgvsM => "hgvs.m", HgvsO => "hgvs.o", Spdi => "spdi",
    Gnomad => "gnomad", Vcf => "vcf", Iscn => "iscn",
});
