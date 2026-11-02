//! Checked identifiers for genes, variants, alleles and ClinVar attribution.
use std::{
    fmt,
    num::{NonZeroU32, NonZeroU64},
    str::FromStr,
};

/// Text does not have the canonical form of its identifier namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdError;
impl IdError {
    /// Stable refusal code for an invalid identifier spelling or bound.
    pub const fn code(self) -> &'static str {
        "id.value.invalid"
    }
}
impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid identifier")
    }
}
impl std::error::Error for IdError {}

pub(crate) const OMIM_MIN: u32 = 100_000;
pub(crate) const OMIM_MAX: u32 = 699_999;

macro_rules! text_serde {
    ($ty:ty) => {
        #[cfg(feature = "serde")]
        #[cfg(feature = "serde")]
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }
        #[cfg(feature = "serde")]
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let text: String = serde::Deserialize::deserialize(deserializer)?;
                text.parse().map_err(serde::de::Error::custom)
            }
        }
    };
}

macro_rules! numeric_id {
    ($(#[$doc:meta])* $name:ident, $number:ty, $primitive:ty, $prefix:literal $(, $minimum:expr, $maximum:expr)?) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($number);
        impl $name {
            /// Construct from a number in this known namespace.
            pub fn new(number: $primitive) -> Result<Self, IdError> {
                $(if !($minimum..=$maximum).contains(&number) { return Err(IdError); })?
                <$number>::new(number).map(Self).ok_or(IdError)
            }
            /// Return the positive numeric identifier.
            pub fn get(self) -> $primitive { self.0.get() }
        }
        impl FromStr for $name {
            type Err = IdError;
            fn from_str(text: &str) -> Result<Self, IdError> {
                let digits = text.strip_prefix($prefix).ok_or(IdError)?;
                let number = digits.parse().map_err(|_| IdError)?;
                let id = Self::new(number)?;
                (id.to_string() == text).then_some(id).ok_or(IdError)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", $prefix, self.get())
            }
        }
        text_serde!($name);
    };
}
numeric_id!(/// An HGNC gene identity.
    HgncId, NonZeroU32, u32, "HGNC:");
numeric_id!(/// An NCBI Gene identity.
    NcbiGeneId, NonZeroU32, u32, "");
numeric_id!(/// An OMIM identity.
    OmimId, NonZeroU32, u32, "", OMIM_MIN, OMIM_MAX);
numeric_id!(/// A dbSNP site, possibly containing several alleles; never an allele join key.
    RsId, NonZeroU64, u64, "rs");
numeric_id!(/// A ClinGen protein allele.
    PaId, NonZeroU64, u64, "PA");
numeric_id!(/// A ClinVar variation record; it can identify an allele, haplotype or compound.
    ClinVarVariationId, NonZeroU32, u32, "clinvar:");
numeric_id!(/// An allele inside a ClinVar record, distinct from its variation identity.
    ClinVarAlleleId, NonZeroU32, u32, "");

/// A ClinGen nucleotide allele. Registry text uses at least six digits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaId(NonZeroU64);
impl CaId {
    /// Construct from a known ClinGen allele number.
    pub fn new(number: u64) -> Result<Self, IdError> {
        NonZeroU64::new(number).map(Self).ok_or(IdError)
    }
    /// Return the registry number.
    pub fn get(self) -> u64 {
        self.0.get()
    }
}
impl FromStr for CaId {
    type Err = IdError;
    fn from_str(text: &str) -> Result<Self, IdError> {
        let number = text
            .strip_prefix("CA")
            .ok_or(IdError)?
            .parse()
            .map_err(|_| IdError)?;
        let id = Self::new(number)?;
        (id.to_string() == text).then_some(id).ok_or(IdError)
    }
}
impl fmt::Display for CaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CA{:06}", self.get())
    }
}
text_serde!(CaId);

macro_rules! clinvar_accession {
    ($(#[$doc:meta])* $name:ident, $prefix:literal) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name { number: NonZeroU32, version: Option<NonZeroU32> }
        impl $name {
            /// Construct from a known record number and optional source version.
            pub fn new(number: u32, version: Option<u32>) -> Result<Self, IdError> {
                if number > 999_999_999 { return Err(IdError); }
                Ok(Self {
                    number: NonZeroU32::new(number).ok_or(IdError)?,
                    version: version.map(|v| NonZeroU32::new(v).ok_or(IdError)).transpose()?,
                })
            }
            /// Return the record number.
            pub fn get(self) -> u32 { self.number.get() }
            /// Compare the source accession without its supplied version.
            pub fn same_accession(self, other: Self) -> bool { self.number == other.number }
            /// Return the supplied version, if any.
            pub fn version(self) -> Option<u32> { self.version.map(NonZeroU32::get) }
        }
        impl FromStr for $name {
            type Err = IdError;
            fn from_str(text: &str) -> Result<Self, IdError> {
                let tail = text.strip_prefix($prefix).ok_or(IdError)?;
                let (number, version) = match tail.split_once('.') {
                    Some((number, version)) => (number, Some(version.parse().map_err(|_| IdError)?)),
                    None => (tail, None),
                };
                let id = Self::new(number.parse().map_err(|_| IdError)?, version)?;
                (id.to_string() == text).then_some(id).ok_or(IdError)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{:09}", $prefix, self.get())?;
                if let Some(version) = self.version() { write!(f, ".{version}")?; }
                Ok(())
            }
        }
        text_serde!($name);
    };
}
clinvar_accession!(/// A ClinVar variation accession, which can name an allele, haplotype or compound.
    VcvId, "VCV");
clinvar_accession!(/// A ClinVar variant-condition assertion aggregate; attribution, not variant identity.
    RcvId, "RCV");
clinvar_accession!(/// A ClinVar submission; attribution, not variant identity.
    ScvId, "SCV");

/// Which kind of COSMIC variant a source identified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CosmicKind {
    /// Genomic variant.
    Cosv,
    /// Legacy transcript mutation.
    Cosm,
    /// Noncoding variant.
    Cosn,
}
/// A COSMIC identity whose kind is part of the key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CosmicId {
    kind: CosmicKind,
    number: NonZeroU64,
}
impl CosmicId {
    /// Construct from a known COSMIC kind and number.
    pub fn new(kind: CosmicKind, number: u64) -> Result<Self, IdError> {
        Ok(Self {
            kind,
            number: NonZeroU64::new(number).ok_or(IdError)?,
        })
    }
    /// Return the COSMIC kind.
    pub fn kind(self) -> CosmicKind {
        self.kind
    }
    /// Return the numeric part.
    pub fn get(self) -> u64 {
        self.number.get()
    }
}
impl FromStr for CosmicId {
    type Err = IdError;
    fn from_str(text: &str) -> Result<Self, IdError> {
        let (kind, tail) = if let Some(v) = text.strip_prefix("COSV") {
            (CosmicKind::Cosv, v)
        } else if let Some(v) = text.strip_prefix("COSM") {
            (CosmicKind::Cosm, v)
        } else if let Some(v) = text.strip_prefix("COSN") {
            (CosmicKind::Cosn, v)
        } else {
            return Err(IdError);
        };
        let id = Self::new(kind, tail.parse().map_err(|_| IdError)?)?;
        (id.to_string() == text).then_some(id).ok_or(IdError)
    }
}
impl fmt::Display for CosmicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.kind {
            CosmicKind::Cosv => "COSV",
            CosmicKind::Cosm => "COSM",
            CosmicKind::Cosn => "COSN",
        };
        write!(f, "{prefix}{}", self.get())
    }
}
text_serde!(CosmicId);

/// A source-given GA4GH VRS allele identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VrsId(Box<str>);
impl FromStr for VrsId {
    type Err = IdError;
    fn from_str(text: &str) -> Result<Self, IdError> {
        let digest = text.strip_prefix("ga4gh:VA.").ok_or(IdError)?;
        if digest.len() != 32
            || !digest
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err(IdError);
        }
        Ok(Self(text.into()))
    }
}
impl fmt::Display for VrsId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_serde!(VrsId);

/// Variant-level identities only. RCV and SCV identify statements, and an rsID names a site.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum VariantId {
    /// A dbSNP site, never an allele join key.
    Rs(RsId),
    /// One nucleotide allele.
    Ca(CaId),
    /// One protein allele.
    Pa(PaId),
    /// A ClinVar variation record, possibly a haplotype or compound.
    Vcv(VcvId),
    /// The same ClinVar variation level as VCV.
    ClinVarVariation(ClinVarVariationId),
    /// A COSMIC genomic, transcript or noncoding variant according to its kind.
    Cosmic(CosmicId),
    /// One source-supplied GA4GH VRS allele digest.
    Vrs(VrsId),
}
impl FromStr for VariantId {
    type Err = IdError;
    fn from_str(text: &str) -> Result<Self, IdError> {
        if text.starts_with("rs") {
            text.parse().map(Self::Rs)
        } else if text.starts_with("CA") {
            text.parse().map(Self::Ca)
        } else if text.starts_with("PA") {
            text.parse().map(Self::Pa)
        } else if text.starts_with("VCV") {
            text.parse().map(Self::Vcv)
        } else if text.starts_with("clinvar:") {
            text.parse().map(Self::ClinVarVariation)
        } else if text.starts_with("COS") {
            text.parse().map(Self::Cosmic)
        } else if text.starts_with("ga4gh:VA.") {
            text.parse().map(Self::Vrs)
        } else {
            Err(IdError)
        }
    }
}
impl fmt::Display for VariantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rs(v) => v.fmt(f),
            Self::Ca(v) => v.fmt(f),
            Self::Pa(v) => v.fmt(f),
            Self::Vcv(v) => v.fmt(f),
            Self::ClinVarVariation(v) => v.fmt(f),
            Self::Cosmic(v) => v.fmt(f),
            Self::Vrs(v) => v.fmt(f),
        }
    }
}
text_serde!(VariantId);
