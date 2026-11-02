use crate::{Error, text_value};
use std::{fmt, num::NonZeroU32, str::FromStr};

fn split_version(value: &str) -> Option<(&str, Option<NonZeroU32>)> {
    match value.split_once('.') {
        None => Some((value, None)),
        Some((base, version)) => {
            if version.starts_with('0') || !version.bytes().all(|ch| ch.is_ascii_digit()) {
                return None;
            }
            Some((base, Some(NonZeroU32::new(version.parse().ok()?)?)))
        }
    }
}
macro_rules! versioned {
    ($(#[doc = $doc:literal])* $name:ident, $error:ident, $schema:literal, $pattern:expr, $check:expr) => {
        $(#[doc = $doc])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name { base: Box<str>, version: Option<NonZeroU32> }
        impl $name {
            /// Parse a full accession and retain a stated version separately.
            pub fn new(value: &str) -> Result<Self, Error> {
                let (base, version) = split_version(value).ok_or(Error::$error)?;
                if !($check)(base) { return Err(Error::$error); }
                Ok(Self { base: base.into(), version })
            }
            /// Borrow the accession without its version suffix.
            pub fn base(&self) -> &str { &self.base }
            /// Return the positive stated version, if any.
            pub fn version(&self) -> Option<u32> { self.version.map(NonZeroU32::get) }
        }
        impl FromStr for $name { type Err = Error; fn from_str(value: &str) -> Result<Self, Error> { Self::new(value) } }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.base())?;
                if let Some(version) = self.version { write!(f, ".{version}")?; }
                Ok(())
            }
        }
        text_value!($name, $schema, $pattern, concat!($($doc),*));
    };
}
fn ensembl(value: &str, prefix: &str) -> bool {
    value
        .strip_prefix(prefix)
        .is_some_and(|number| number.len() == 11 && number.bytes().all(|ch| ch.is_ascii_digit()))
}
versioned! {
    /// A human Ensembl gene stable identifier with a separately retained version.
    EnsemblGeneId, EnsemblGeneIdInvalid, "EnsemblGeneId",
    format!(r"^ENSG[0-9]{{11}}(\.{})?$", crate::schema_pattern::decimal(u32::MAX.into())), |value| ensembl(value, "ENSG")
}
