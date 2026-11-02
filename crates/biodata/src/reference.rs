//! Reference assembly, primary chromosome and direction values.
use crate::Accession;
use std::{fmt, str::FromStr};
/// A reference value fails its checked construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceError {
    /// The build name is not supported.
    BuildUnknown,
    /// The primary chromosome spelling or accession is not valid.
    ChromosomeInvalid,
    /// The source contig name is not valid.
    ContigInvalid,
}
impl ReferenceError {
    /// Stable machine-readable refusal.
    pub const fn code(self) -> &'static str {
        match self {
            Self::BuildUnknown => "coord.build.unknown",
            Self::ChromosomeInvalid => "coord.chromosome.invalid",
            Self::ContigInvalid => "coord.contig.invalid",
        }
    }
}
impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for ReferenceError {}
/// A build selected by the caller.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Build {
    /// Genome Reference Consortium human build 37.
    GRCh37,
    /// Genome Reference Consortium human build 38.
    GRCh38,
}

/// Transcript direction on the reference chromosome.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i8)]
pub enum Strand {
    /// Transcript bases follow the reference.
    #[cfg_attr(feature = "serde", serde(rename = "+"))]
    Plus = 1,
    /// Transcript bases follow its reverse complement.
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Minus = -1,
}
const VERSIONS: [[u16; 24]; 2] = [
    [
        10, 11, 11, 11, 9, 11, 13, 10, 11, 10, 9, 11, 10, 8, 9, 9, 10, 9, 9, 10, 8, 10, 10, 9,
    ],
    [
        11, 12, 12, 12, 10, 12, 14, 11, 12, 11, 10, 12, 11, 9, 10, 10, 11, 10, 10, 11, 9, 11, 11,
        10,
    ],
];
/// A primary chromosome in the caller's reference build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(u8);
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Chromosome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.name())
    }
}

impl Chromosome {
    /// Construct a primary chromosome from its numeric identifier (1–25).
    pub fn new(number: u8) -> Option<Self> {
        (1..=25).contains(&number).then_some(Self(number))
    }

    /// Return its chromosome accession in this build.
    pub fn accession(self, build: Build) -> Accession {
        let (number, version) = if self.0 == 25 {
            (12920, 1)
        } else {
            let row = if build == Build::GRCh37 {
                VERSIONS.first()
            } else {
                VERSIONS.last()
            };
            (
                u32::from(self.0),
                row.and_then(|v| v.get(usize::from(self.0 - 1)))
                    .copied()
                    .unwrap_or(0),
            )
        };
        Accession::chromosome(number, version)
    }
    /// Chromosome name without the optional `chr` prefix.
    pub fn name(self) -> String {
        match self.0 {
            23 => "X".into(),
            24 => "Y".into(),
            25 => "MT".into(),
            n => n.to_string(),
        }
    }
    /// Read a canonical chromosome spelling.
    pub fn parse(text: &str) -> Result<Self, ReferenceError> {
        let n = match text {
            "X" => 23,
            "Y" => 24,
            "MT" => 25,
            _ => text
                .parse()
                .map_err(|_| ReferenceError::ChromosomeInvalid)?,
        };
        let value = Self::new(n).ok_or(ReferenceError::ChromosomeInvalid)?;
        if value.name() == text {
            Ok(value)
        } else {
            Err(ReferenceError::ChromosomeInvalid)
        }
    }
    /// Numeric primary chromosome identifier (1–25).
    pub const fn number(self) -> u8 {
        self.0
    }
    /// Resolve an exact primary chromosome accession on the selected build.
    pub fn from_accession(ac: Accession, build: Build) -> Result<Self, ReferenceError> {
        (1..=25)
            .map(Self)
            .find(|c| c.accession(build) == ac)
            .ok_or(ReferenceError::ChromosomeInvalid)
    }
}

impl FromStr for Build {
    type Err = ReferenceError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "GRCh37" => Ok(Self::GRCh37),
            "GRCh38" => Ok(Self::GRCh38),
            _ => Err(ReferenceError::BuildUnknown),
        }
    }
}
impl fmt::Display for Build {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl FromStr for Chromosome {
    type Err = ReferenceError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

impl Ord for Strand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        matches!(self, Self::Minus).cmp(&matches!(other, Self::Minus))
    }
}
impl PartialOrd for Strand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Strand {
    /// Read the source direction sign.
    pub fn from_sign(sign: i8) -> Option<Self> {
        match sign {
            1 => Some(Self::Plus),
            -1 => Some(Self::Minus),
            _ => None,
        }
    }
}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}
/// Read provider chromosome spellings at an adapter boundary.
pub fn parse_chromosome(text: &str) -> Result<Chromosome, ReferenceError> {
    let text = without_chr_prefix(text);
    let number = match text {
        "X" => 23,
        "Y" => 24,
        "M" | "MT" => 25,
        _ => text
            .parse()
            .map_err(|_| ReferenceError::ChromosomeInvalid)?,
    };
    Chromosome::new(number).ok_or(ReferenceError::ChromosomeInvalid)
}
/// Remove only a written `chr` prefix when an adapter echoes source text.
/// This does not validate or canonicalize the remaining source name.
pub fn without_chr_prefix(text: &str) -> &str {
    text.strip_prefix("chr").unwrap_or(text)
}
/// A source chromosome or scaffold name without spelling normalization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Contig(Box<str>);
impl Contig {
    /// Maximum number of bytes in a source contig name.
    pub const MAX_LEN: usize = 255;
    /// Check a nonempty printable ASCII source name.
    pub fn new(text: &str) -> Result<Self, ReferenceError> {
        if text.is_empty()
            || text.len() > Self::MAX_LEN
            || !text.bytes().all(|ch| ch.is_ascii_graphic())
        {
            return Err(ReferenceError::ContigInvalid);
        }
        Ok(Self(text.into()))
    }
    /// Source spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for Contig {
    type Err = ReferenceError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::new(text)
    }
}
impl fmt::Display for Contig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Contig {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Contig {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(&text).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Chromosome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
