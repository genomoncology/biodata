//! Checked source text, reference-qualified notation, and partial dates.
use crate::{Accession, GeneSymbol};
use std::{fmt, str::FromStr};

/// Empty source text is not a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextError;

impl TextError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        "text.value.empty"
    }
}

impl fmt::Display for TextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for TextError {}

/// A source notation's reference and body failed validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceTextError {
    /// A source notation does not match its declared form.
    NotationInvalid,
    /// Text is empty after trimming.
    Empty,
    /// No HGVS reference separator is present.
    ReferenceMissing,
    /// The outer reference is invalid.
    ReferenceInvalid,
    /// The inner reference or gene is invalid.
    InnerReferenceInvalid,
    /// The body has no supported HGVS type marker.
    SyntaxUnknown,
    /// No body follows the HGVS type marker.
    BodyEmpty,
    /// The body contains a nonprintable character.
    BodyUnprintable,
    /// A partial date has invalid shape or calendar values.
    DateInvalid,
}

impl SourceTextError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::NotationInvalid => "notation.invalid",
            Self::Empty => "hgvs.text.empty",
            Self::ReferenceMissing => "hgvs.reference.missing",
            Self::ReferenceInvalid => "hgvs.reference.invalid",
            Self::InnerReferenceInvalid => "hgvs.inner_reference.invalid",
            Self::SyntaxUnknown => "hgvs.syntax.unknown",
            Self::BodyEmpty => "hgvs.body.empty",
            Self::BodyUnprintable => "hgvs.body.unprintable",
            Self::DateInvalid => "date.partial.invalid",
        }
    }
}

impl fmt::Display for SourceTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for SourceTextError {}

/// Reference-qualified HGVS text retained without molecular reinterpretation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HgvsText(Box<str>);

impl HgvsText {
    /// Borrow the unchanged source notation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Read the checked outer reference accession.
    pub fn accession(&self) -> Result<Accession, SourceTextError> {
        let outer = self.0.split_once(':').map_or("", |(outer, _)| outer);
        let outer = outer.split_once('(').map_or(outer, |(outer, _)| outer);
        outer.parse().map_err(|_| SourceTextError::ReferenceInvalid)
    }
}

impl FromStr for HgvsText {
    type Err = SourceTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(SourceTextError::Empty);
        }
        let (reference, body) = value
            .split_once(':')
            .ok_or(SourceTextError::ReferenceMissing)?;
        let outer = if let Some((outer, inner)) = reference.split_once('(') {
            let inner = inner
                .strip_suffix(')')
                .ok_or(SourceTextError::InnerReferenceInvalid)?;
            if inner.parse::<Accession>().is_err() && inner.parse::<GeneSymbol>().is_err() {
                return Err(SourceTextError::InnerReferenceInvalid);
            }
            outer
        } else {
            reference
        };
        let accession = outer
            .parse::<Accession>()
            .map_err(|_| SourceTextError::ReferenceInvalid)?;
        let inner = reference
            .split_once('(')
            .and_then(|(_, value)| value.strip_suffix(')'))
            .and_then(|value| value.parse::<Accession>().ok());
        if !body.starts_with("p.")
            && (protein_reference(accession) || inner.is_some_and(protein_reference))
        {
            return Err(SourceTextError::ReferenceInvalid);
        }
        check_hgvs_body(body)?;
        if !matches!(
            body.as_bytes().first(),
            Some(b'g' | b'c' | b'n' | b'r' | b'p' | b'm' | b'o')
        ) || body.as_bytes().get(1) != Some(&b'.')
        {
            return Err(SourceTextError::SyntaxUnknown);
        }
        if body.len() < 3 {
            return Err(SourceTextError::BodyEmpty);
        }
        if !body
            .bytes()
            .all(|byte| byte.is_ascii_graphic() || byte == b' ')
        {
            return Err(SourceTextError::BodyUnprintable);
        }
        Ok(Self(value.into()))
    }
}

impl fmt::Display for HgvsText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for HgvsText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HgvsText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

/// A Gregorian date preserving year, month or day precision. The parser additionally validates the calendar day, including leap years.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PartialDate {
    year: u16,
    month: Option<u8>,
    day: Option<u8>,
}

impl PartialDate {
    /// Check components without filling missing precision.
    pub fn new(year: u16, month: Option<u8>, day: Option<u8>) -> Result<Self, crate::Error> {
        if !(1..=9999).contains(&year) || (day.is_some() && month.is_none()) {
            return Err(crate::Error::DateInvalid);
        }
        if let Some(month) = month {
            let days = match month {
                4 | 6 | 9 | 11 => 30,
                2 if year.is_multiple_of(400)
                    || (year.is_multiple_of(4) && !year.is_multiple_of(100)) =>
                {
                    29
                }
                2 => 28,
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                _ => return Err(crate::Error::DateInvalid),
            };
            if day.is_some_and(|day| !(1..=days).contains(&day)) {
                return Err(crate::Error::DateInvalid);
            }
        }
        Ok(Self { year, month, day })
    }

    /// Return the stated year.
    pub fn year(self) -> u16 {
        self.year
    }
    /// Return the stated month, if present.
    pub fn month(self) -> Option<u8> {
        self.month
    }
    /// Return the stated day, if present.
    pub fn day(self) -> Option<u8> {
        self.day
    }
}

impl FromStr for PartialDate {
    type Err = SourceTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split('-');
        let year = parts.next().ok_or(SourceTextError::DateInvalid)?;
        let month = parts.next();
        let day = parts.next();
        if parts.next().is_some() {
            return Err(SourceTextError::DateInvalid);
        }
        let parse = |part: &str, width| {
            if part.len() != width || !part.bytes().all(|ch| ch.is_ascii_digit()) {
                return Err(SourceTextError::DateInvalid);
            }
            part.parse::<u16>()
                .map_err(|_| SourceTextError::DateInvalid)
        };
        let component =
            |part: &str| u8::try_from(parse(part, 2)?).map_err(|_| SourceTextError::DateInvalid);
        Self::new(
            parse(year, 4)?,
            month.map(component).transpose()?,
            day.map(component).transpose()?,
        )
        .map_err(|_| SourceTextError::DateInvalid)
    }
}

impl fmt::Display for PartialDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:04}", self.year)?;
        if let Some(month) = self.month {
            write!(formatter, "-{month:02}")?;
        }
        if let Some(day) = self.day {
            write!(formatter, "-{day:02}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for PartialDate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PartialDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
use crate::{Error, text_control::unsafe_control};

/// Bounded nonempty text with surrounding Unicode whitespace removed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Text(Box<str>);

impl Text {
    /// Maximum Unicode scalar count after trimming or source escaping.
    pub const MAX_CHARS: usize = 1_048_576;

    /// Trim whitespace and reject empty, oversized or unsafe control text.
    /// C0, DEL, C1 and Unicode bidi controls must use visible source escapes.
    pub fn new(value: &str) -> Result<Self, Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(Error::TextEmpty)
        } else if value.chars().any(unsafe_control) {
            Err(Error::TextControl)
        } else if trimmed.chars().count() > Self::MAX_CHARS {
            Err(Error::TextTooLong)
        } else {
            Ok(Self(trimmed.into()))
        }
    }

    /// Admit source text without dropping a record for unsafe controls.
    /// Controls become literal `\\u{xxxx}` escapes before constructor checks.
    pub fn from_source(value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::TextEmpty);
        }
        let mut escaped = String::new();
        for ch in value
            .trim_matches(|ch: char| ch.is_whitespace() && !unsafe_control(ch))
            .chars()
        {
            if unsafe_control(ch) {
                use std::fmt::Write;
                write!(&mut escaped, "\\u{{{:04x}}}", u32::from(ch))
                    .map_err(|_| Error::TextControl)?;
            } else {
                escaped.push(ch);
            }
            if escaped.len() > Self::MAX_CHARS * 4 {
                return Err(Error::TextTooLong);
            }
        }
        Self::new(&escaped)
    }

    /// Borrow the checked text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Text {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for Text {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

pub(crate) fn check_hgvs_body(body: &str) -> Result<(), SourceTextError> {
    if !matches!(
        body.as_bytes().first(),
        Some(b'g' | b'c' | b'n' | b'r' | b'p' | b'm' | b'o')
    ) || body.as_bytes().get(1) != Some(&b'.')
    {
        return Err(SourceTextError::SyntaxUnknown);
    }
    if body.len() < 3 {
        return Err(SourceTextError::BodyEmpty);
    }
    if !body.bytes().all(|b| b.is_ascii() && !b.is_ascii_control()) {
        return Err(SourceTextError::BodyUnprintable);
    }
    if !body.bytes().all(|b| b.is_ascii_graphic()) {
        return Err(SourceTextError::NotationInvalid);
    }
    if !body[2..].bytes().any(|b| b.is_ascii_digit()) && !positionless(body) {
        return Err(SourceTextError::NotationInvalid);
    }
    Ok(())
}
// HGVS 21.1 defines these positionless forms:
// https://hgvs-nomenclature.org/stable/recommendations/uncertain/
// https://hgvs-nomenclature.org/stable/recommendations/RNA/splicing/
fn positionless(body: &str) -> bool {
    matches!(
        body,
        "p.?"
            | "p.(?)"
            | "p.="
            | "p.(=)"
            | "r.?"
            | "r.(?)"
            | "r.="
            | "r.(=)"
            | "r.spl"
            | "r.spl?"
            | "r.(spl?)"
    )
}
fn protein_reference(accession: Accession) -> bool {
    matches!(
        accession.refseq_parts(),
        Some((crate::RefSeqPrefix::Np | crate::RefSeqPrefix::Xp, _, _))
    ) || matches!(
        accession.ensembl_parts(),
        Some((crate::EnsemblFeature::Protein, _, _))
    ) || accession
        .to_string()
        .strip_prefix("LRG_")
        .is_some_and(|value| value.contains('p'))
}

impl From<crate::TextLiteral> for Text {
    fn from(literal: crate::TextLiteral) -> Self {
        Self(literal.source().trim().into())
    }
}
