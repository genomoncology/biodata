//! Text tables for keyword value types.
macro_rules! text_table {
    (frozen $name:ident, $doc:literal, $parser:expr, {$($variant:ident => $text:literal),+ $(,)?}, $case:literal) => {
        crate::table::text_table!(@define [Hash, PartialOrd, Ord] $name, $doc, $parser, {$($variant => $text),+}, $case);
        crate::frozen_api::canonical_extras!($name, AccessionInvalid { $($variant => $text),+ });
        crate::frozen_api::canonical_parse!($name, AccessionInvalid { $($variant => $text),+ });
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                // Deserialize the existing wire spelling. Canonical prefix text
                // belongs to FromStr, so this adds no compatibility aliases.
                #[derive(serde::Deserialize)]
                #[serde(rename_all = $case)]
                enum Stored { $($variant),+ }
                let stored = <Stored as serde::Deserialize>::deserialize(
                    serde::de::value::StrDeserializer::<D::Error>::new(&value))
                    .map_err(|_| serde::de::Error::custom(crate::Error::AccessionInvalid))?;
                Ok(match stored { $(Stored::$variant => Self::$variant),+ })
            }
        }
    };

    ($name:ident, $doc:literal, $parser:expr, {$($variant:ident => $text:literal $(| $alias:literal)*),+ $(,)?}) => {
        crate::table::text_table!($name, $doc, $parser, {$($variant => $text $(| $alias)*),+}, "snake_case");
    };
    ($name:ident, $doc:literal, $parser:expr, {$($variant:ident => $text:literal $(| $alias:literal)*),+ $(,)?}, $case:literal) => {
        crate::table::text_table!(@define [] $name, $doc, $parser, {$($variant => $text $(| $alias)*),+}, $case);
    };
    (@define [$($extra:path),*] $name:ident, $doc:literal, $parser:expr, {$($variant:ident => $text:literal $(| $alias:literal)*),+ $(,)?}, $case:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq $(, $extra)*)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = $case))]
        pub enum $name { $(#[doc = $text] $variant),+ }
        impl $name {
            pub(crate) fn parse(input: &mut crate::parse::Input<'_>) -> crate::parse::Result<Self> {
                crate::parse::step(input)?;
                $(if winnow::Parser::parse_next(&mut winnow::combinator::opt(($parser)($text)), input)?.is_some() {
                    return Ok(Self::$variant);
                }
                $(if winnow::Parser::parse_next(&mut winnow::combinator::opt(($parser)($alias)), input)?.is_some() {
                    return Ok(Self::$variant);
                })* )+
                winnow::Parser::parse_next(&mut winnow::combinator::fail, input)
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self { $(Self::$variant => $text),+ })
            }
        }
    };
}
pub(crate) use text_table;

macro_rules! codes {
    ($(#[$meta:meta])* pub enum $name:ident { $($(#[$doc:meta])* $variant:ident => $text:literal),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name { $(#[doc = $text] $(#[$doc])* $variant),* }
        impl $name {
            pub(crate) fn name(self) -> &'static str {
                match self { $(Self::$variant => $text),* }
            }
        }
    };
}
pub(crate) use codes;
