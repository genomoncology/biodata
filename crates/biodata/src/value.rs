//! Shared biomedical value types and the grammar for their text.

use crate::outcome::Code;
use crate::parse::{Expect, Fail, Input, Result, decimal, reject, step};
use std::fmt;
use std::str::FromStr;
use winnow::Parser;
use winnow::ascii::digit1;
use winnow::combinator::{alt, cut_err, opt, preceded};
use winnow::error::ErrMode;

/// One nucleotide: the four DNA bases and the IUPAC ambiguity codes.
/// It renders as its upper-case letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
#[expect(missing_docs, reason = "each variant is its IUPAC letter")]
pub enum Base {
    A = b'A',
    C = b'C',
    G = b'G',
    T = b'T',
    N = b'N',
    R = b'R',
    Y = b'Y',
    S = b'S',
    W = b'W',
    K = b'K',
    M = b'M',
    B = b'B',
    D = b'D',
    H = b'H',
    V = b'V',
}

/// A reference sequence accession: RefSeq or Ensembl with its version, or LRG.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Accession(AccessionKind);

crate::table::codes! {
/// Refusals of a reference accession spelling.
pub enum AccessionCode {
    /// The prefix names no supported accession family.
    PrefixUnknown => "accession.prefix.unknown",
    /// The numeric identifier has the wrong width.
    DigitsWidth => "accession.digits.width",
    /// A versioned accession has no version.
    VersionMissing => "accession.version.missing",
    /// A version begins with zero.
    VersionLeadingZero => "accession.version.leading_zero",
    /// A version exceeds the supported integer range.
    VersionOverflow => "accession.version.overflow",
    /// An unversioned accession states a version.
    VersionPresent => "accession.version.present",
    /// An LRG identifier has invalid syntax.
    LrgInvalid => "accession.lrg.invalid",
}
}
const PREFIX: Expect = Expect {
    code: Code::Accession(AccessionCode::PrefixUnknown),
    label: "accession prefix",
};
const WIDTH: Expect = Expect {
    code: Code::Accession(AccessionCode::DigitsWidth),
    label: "accession digit width",
};
const VERSION_MISSING: Expect = Expect {
    code: Code::Accession(AccessionCode::VersionMissing),
    label: "accession version",
};
const VERSION_LEADING_ZERO: Expect = Expect {
    code: Code::Accession(AccessionCode::VersionLeadingZero),
    label: "version without leading zero",
};
const VERSION_OVERFLOW: Expect = Expect {
    code: Code::Accession(AccessionCode::VersionOverflow),
    label: "supported accession version",
};
const VERSION_PRESENT: Expect = Expect {
    code: Code::Accession(AccessionCode::VersionPresent),
    label: "unversioned accession",
};
const LRG_INVALID: Expect = Expect {
    code: Code::Accession(AccessionCode::LrgInvalid),
    label: "LRG accession",
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum AccessionKind {
    /// A RefSeq accession such as `NC_000007.14`.
    RefSeq {
        /// The two letters and underscore.
        prefix: RefSeqPrefix,
        /// The number, written in six digits, or nine from 1000000 up.
        number: u32,
        /// The version after the dot.
        version: u16,
    },
    /// A human Ensembl accession such as `ENST00000288602.11`.
    Ensembl {
        /// The letter after `ENS`.
        feature: EnsemblFeature,
        /// The number, written in eleven digits.
        number: u64,
        /// The version after the dot.
        version: u16,
    },
    /// A Locus Reference Genomic record such as `LRG_199`, `LRG_199t1` or `LRG_199p1`.
    Lrg {
        /// The record number.
        number: u32,
        /// A transcript or protein of the record.
        part: Option<LrgPart>,
    },
}

crate::table::text_table!(frozen RefSeqPrefix, "The RefSeq prefixes a variant description uses.", |text| text, {
    Nc => "NC_",
    Ng => "NG_",
    Nm => "NM_",
    Np => "NP_",
    Nr => "NR_",
    Nt => "NT_",
    Nw => "NW_",
    Xm => "XM_",
    Xp => "XP_",
    Xr => "XR_",
}, "snake_case");

crate::table::text_table!(frozen EnsemblFeature, "What an Ensembl accession names.", |text| text, {
    Gene => "ENSG",
    Transcript => "ENST",
    Protein => "ENSP",
}, "snake_case");

/// A transcript or protein of an LRG record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LrgPart {
    /// `t1`
    Transcript(u16),
    /// `p1`
    Protein(u16),
}

impl Base {
    /// Parse one upper-case IUPAC DNA letter.
    pub fn from_letter(letter: char) -> Option<Self> {
        use Base::*;
        Some(match letter {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            'N' => N,
            'R' => R,
            'Y' => Y,
            'S' => S,
            'W' => W,
            'K' => K,
            'M' => M,
            'B' => B,
            'D' => D,
            'H' => H,
            'V' => V,
            _ => return None,
        })
    }

    /// Return the IUPAC DNA complement of this base.
    pub const fn complement(self) -> Self {
        use Base::*;
        match self {
            A => T,
            T => A,
            C => G,
            G => C,
            R => Y,
            Y => R,
            K => M,
            M => K,
            B => V,
            V => B,
            D => H,
            H => D,
            N => N,
            S => S,
            W => W,
        }
    }
}

/// An accession. Its prefix commits to the rest.
pub(crate) fn accession(input: &mut Input<'_>) -> Result<Accession> {
    step(input)?;
    alt((refseq, ensembl, lrg))
        .context(PREFIX)
        .parse_next(input)
}

fn refseq(input: &mut Input<'_>) -> Result<Accession> {
    let prefix = RefSeqPrefix::parse(input)?;
    let (number, version) =
        cut_err((digits(refseq_width).context(WIDTH), version)).parse_next(input)?;
    Ok(Accession(AccessionKind::RefSeq {
        prefix,
        number,
        version,
    }))
}

fn ensembl(input: &mut Input<'_>) -> Result<Accession> {
    let feature = EnsemblFeature::parse(input)?;
    let (number, version) = cut_err((digits(|_| 11).context(WIDTH), version)).parse_next(input)?;
    Ok(Accession(AccessionKind::Ensembl {
        feature,
        number,
        version,
    }))
}

fn lrg(input: &mut Input<'_>) -> Result<Accession> {
    let number = preceded("LRG_", cut_err(decimal.context(LRG_INVALID))).parse_next(input)?;
    let part = opt(alt((
        preceded('t', cut_err(decimal.context(LRG_INVALID))).map(LrgPart::Transcript),
        preceded('p', cut_err(decimal.context(LRG_INVALID))).map(LrgPart::Protein),
    )))
    .parse_next(input)?;
    if input.len() > 0
        && !input.starts_with(':')
        && !input.starts_with('(')
        && !input.starts_with(')')
        && !input.starts_with('.')
    {
        return reject(input, Some(LRG_INVALID));
    }
    Ok(Accession(AccessionKind::Lrg { number, part }))
}

fn version(input: &mut Input<'_>) -> Result<u16> {
    '.'.context(VERSION_MISSING).parse_next(input)?;
    if input.starts_with('0') {
        return reject(input, Some(VERSION_LEADING_ZERO));
    }
    let text: &str = digit1.context(VERSION_MISSING).parse_next(input)?;
    match text.parse::<u16>() {
        Ok(version) => Ok(version),
        Err(_) => reject(input, Some(VERSION_OVERFLOW)),
    }
}

/// Digits read as a number, written in exactly the width the number renders at.
pub(crate) fn digits<'a, T: FromStr + Copy>(
    width: impl Fn(T) -> usize,
) -> impl Parser<Input<'a>, T, ErrMode<Fail>> {
    digit1.verify_map(move |text: &str| text.parse().ok().filter(|n| text.len() == width(*n)))
}

fn refseq_width(number: u32) -> usize {
    if number < 1_000_000 { 6 } else { 9 }
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self as u8))
    }
}

impl fmt::Display for Accession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            AccessionKind::RefSeq {
                prefix,
                number,
                version,
            } => {
                let width = refseq_width(number);
                write!(f, "{prefix}{number:0width$}.{version}")
            }
            AccessionKind::Ensembl {
                feature,
                number,
                version,
            } => {
                write!(f, "{feature}{number:011}.{version}")
            }
            AccessionKind::Lrg { number, part } => {
                write!(f, "LRG_{number}")?;
                match part {
                    Some(LrgPart::Transcript(n)) => write!(f, "t{n}"),
                    Some(LrgPart::Protein(n)) => write!(f, "p{n}"),
                    None => Ok(()),
                }
            }
        }
    }
}

/// These serialize as their canonical text.
#[cfg(feature = "serde")]
macro_rules! serialize_as_text {
    ($($name:ty),*) => {$(
        #[cfg(feature = "serde")]
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }
    )*};
}

impl FromStr for Accession {
    type Err = crate::Refusal;
    fn from_str(text: &str) -> std::result::Result<Self, Self::Err> {
        match crate::parse::value(text, accession) {
            crate::Outcome::Parsed(value) => Ok(value),
            crate::Outcome::Invalid(error)
            | crate::Outcome::Unsupported(error)
            | crate::Outcome::LimitExceeded(error) => Err(error),
        }
    }
}
#[cfg(feature = "serde")]
macro_rules! deserialize_text {
    ($($name:ty),*) => {$(impl<'de> serde::Deserialize<'de> for $name {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self,D::Error> {
            let text = <String as serde::Deserialize>::deserialize(deserializer)?;
            text.parse().map_err(|_| serde::de::Error::custom("invalid reference text"))
        }
    })*};
}

impl PartialOrd for Accession {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn accession_key(value: Accession, versioned: bool) -> (u8, u8, u64, u16, u16) {
    match value.0 {
        AccessionKind::RefSeq {
            prefix,
            number,
            version,
        } => (
            0,
            prefix as u8,
            u64::from(number),
            if versioned { version } else { 0 },
            0,
        ),
        AccessionKind::Ensembl {
            feature,
            number,
            version,
        } => (
            1,
            feature as u8,
            number,
            if versioned { version } else { 0 },
            0,
        ),
        AccessionKind::Lrg { number, part } => {
            let (kind, index) = match part {
                None => (0, 0),
                Some(LrgPart::Transcript(index)) => (1, index),
                Some(LrgPart::Protein(index)) => (2, index),
            };
            (2, kind, u64::from(number), index, 0)
        }
    }
}
impl Ord for Accession {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        accession_key(*self, true).cmp(&accession_key(*other, true))
    }
}

impl TryFrom<&str> for Accession {
    type Error = crate::Refusal;
    fn try_from(text: &str) -> std::result::Result<Self, Self::Error> {
        text.parse()
    }
}
impl TryFrom<&String> for Accession {
    type Error = crate::Refusal;
    fn try_from(text: &String) -> std::result::Result<Self, Self::Error> {
        text.parse()
    }
}
impl From<&Accession> for Accession {
    fn from(value: &Accession) -> Self {
        *value
    }
}

impl Accession {
    /// Whether this parsed accession names a transcript rather than a gene,
    /// genomic record, or protein.
    pub fn is_transcript(self) -> bool {
        matches!(
            self.0,
            AccessionKind::RefSeq {
                prefix: RefSeqPrefix::Nm | RefSeqPrefix::Nr | RefSeqPrefix::Xm | RefSeqPrefix::Xr,
                ..
            } | AccessionKind::Ensembl {
                feature: EnsemblFeature::Transcript,
                ..
            } | AccessionKind::Lrg {
                part: Some(LrgPart::Transcript(_)),
                ..
            }
        )
    }
    pub(crate) fn chromosome(number: u32, version: u16) -> Self {
        Self(AccessionKind::RefSeq {
            prefix: RefSeqPrefix::Nc,
            number,
            version,
        })
    }
    /// Checked RefSeq accession components.
    pub fn refseq_parts(self) -> Option<(RefSeqPrefix, u32, u16)> {
        match self.0 {
            AccessionKind::RefSeq {
                prefix,
                number,
                version,
            } => Some((prefix, number, version)),
            _ => None,
        }
    }
    /// Checked Ensembl accession components.
    pub fn ensembl_parts(self) -> Option<(EnsemblFeature, u64, u16)> {
        match self.0 {
            AccessionKind::Ensembl {
                feature,
                number,
                version,
            } => Some((feature, number, version)),
            _ => None,
        }
    }
    /// Stated version, absent for LRG records.
    pub fn version(self) -> Option<u16> {
        match self.0 {
            AccessionKind::RefSeq { version, .. } | AccessionKind::Ensembl { version, .. } => {
                Some(version)
            }
            AccessionKind::Lrg { .. } => None,
        }
    }
    /// Check RefSeq numeric components against the canonical grammar.
    pub fn refseq(
        prefix: RefSeqPrefix,
        number: u32,
        version: u16,
    ) -> std::result::Result<Self, crate::Refusal> {
        if number >= 1_000_000_000 || version == 0 {
            let width = refseq_width(number);
            return format!("{prefix}{number:0width$}.{version}").parse();
        }
        Ok(Self(AccessionKind::RefSeq {
            prefix,
            number,
            version,
        }))
    }
    /// Check Ensembl numeric components against the canonical grammar.
    pub fn ensembl(
        feature: EnsemblFeature,
        number: u64,
        version: u16,
    ) -> std::result::Result<Self, crate::Refusal> {
        if number >= 100_000_000_000 || version == 0 {
            return format!("{feature}{number:011}.{version}").parse();
        }
        Ok(Self(AccessionKind::Ensembl {
            feature,
            number,
            version,
        }))
    }
}

/// An accession identity without a source version.
#[derive(Clone, Copy, Debug, Eq)]
pub struct AccessionBase(Accession);
fn accession_base(input: &mut Input<'_>) -> Result<AccessionBase> {
    step(input)?;
    let value = alt((
        |i: &mut Input<'_>| {
            let prefix = RefSeqPrefix::parse(i)?;
            let number = cut_err(digits(refseq_width).context(WIDTH)).parse_next(i)?;
            Ok(AccessionBase(Accession(AccessionKind::RefSeq {
                prefix,
                number,
                version: 0,
            })))
        },
        |i: &mut Input<'_>| {
            let feature = EnsemblFeature::parse(i)?;
            let number = cut_err(digits(|_| 11).context(WIDTH)).parse_next(i)?;
            Ok(AccessionBase(Accession(AccessionKind::Ensembl {
                feature,
                number,
                version: 0,
            })))
        },
        lrg.map(AccessionBase),
    ))
    .context(PREFIX)
    .parse_next(input)?;
    if input.starts_with('.') {
        return reject(input, Some(VERSION_PRESENT));
    }
    Ok(value)
}
impl FromStr for AccessionBase {
    type Err = crate::Refusal;
    fn from_str(text: &str) -> std::result::Result<Self, Self::Err> {
        match crate::parse::value(text, accession_base) {
            crate::Outcome::Parsed(value) => Ok(value),
            crate::Outcome::Invalid(error)
            | crate::Outcome::Unsupported(error)
            | crate::Outcome::LimitExceeded(error) => Err(error),
        }
    }
}
impl AccessionBase {
    /// Ensembl feature namespace when this checked base names an Ensembl record.
    pub fn ensembl_feature(self) -> Option<EnsemblFeature> {
        self.0.ensembl_parts().map(|(feature, _, _)| feature)
    }
    /// Check RefSeq numeric components through the canonical grammar.
    pub fn refseq(prefix: RefSeqPrefix, number: u32) -> std::result::Result<Self, crate::Refusal> {
        let width = refseq_width(number);
        format!("{prefix}{number:0width$}").parse()
    }
    /// Check Ensembl numeric components through the canonical grammar.
    pub fn ensembl(
        feature: EnsemblFeature,
        number: u64,
    ) -> std::result::Result<Self, crate::Refusal> {
        format!("{feature}{number:011}").parse()
    }
}
impl Accession {
    /// Remove the version for identity matching without fabricating a versioned accession.
    pub fn base(self) -> AccessionBase {
        AccessionBase(self)
    }
}
impl std::hash::Hash for AccessionBase {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&accession_key(self.0, false), state);
    }
}
impl PartialEq for AccessionBase {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl PartialOrd for AccessionBase {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AccessionBase {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        accession_key(self.0, false).cmp(&accession_key(other.0, false))
    }
}

impl fmt::Display for AccessionBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.0 {
            AccessionKind::RefSeq { prefix, number, .. } => {
                let width = refseq_width(number);
                write!(f, "{prefix}{number:0width$}")
            }
            AccessionKind::Ensembl {
                feature, number, ..
            } => write!(f, "{feature}{number:011}"),
            AccessionKind::Lrg { .. } => self.0.fmt(f),
        }
    }
}
#[cfg(feature = "serde")]
serialize_as_text!(AccessionBase);
#[cfg(feature = "serde")]
deserialize_text!(AccessionBase);
pub(crate) use crate::GeneSymbol;
pub(crate) use crate::gene_symbol::gene_symbol_prefix;

#[cfg(feature = "serde")]
serialize_as_text!(Base, Accession, crate::Residue);
#[cfg(feature = "serde")]
deserialize_text!(Accession);
