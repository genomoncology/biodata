//! The result every parse returns: the source text and one outcome.

use crate::hgvs::{HgvsCode, NaCode, ProteinCode};
use std::fmt;

/// The text a writer wrote and what parsing it gave.
pub struct Parsed<T> {
    /// The text exactly as given.
    pub source: Box<str>,
    /// The parsed value or the refusal.
    pub outcome: Outcome<T>,
}

/// A parsed value, or why the text was refused.
#[derive(Debug)]
pub enum Outcome<T> {
    /// The text follows the notation, and this is its value.
    Parsed(T),
    /// The text follows the notation but uses a construct this crate does not parse yet.
    Unsupported(Refusal),
    /// The text breaks the notation.
    Invalid(Refusal),
    /// The text exceeds the length or step budget.
    LimitExceeded(Refusal),
}

/// Why a text was refused, and where.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The stable code. Its last word fixes the outcome.
    pub code: Code,
    /// The bytes the refusal points at.
    pub span: Option<Span>,
    /// What the grammar expected at the span.
    pub expected: Option<&'static str>,
}

/// A half-open range of byte offsets into the source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// The first byte.
    pub start: u32,
    /// The byte after the last.
    pub end: u32,
}

/// A stable refusal code, named `notation.construct.outcome`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    /// Raised before or around any grammar.
    Input(InputCode),
    /// Raised by checked reference accession text.
    Accession(crate::value::AccessionCode),
    /// Raised by the HGVS reference and molecule grammar.
    Hgvs(HgvsCode),
    /// Raised by the HGVS nucleotide grammar.
    Nucleotide(NaCode),
    /// Raised by the HGVS protein grammar.
    Protein(ProteinCode),
    /// Raised by the ISCN grammar.
    Iscn(crate::iscn::IscnCode),
    /// Raised by alteration shorthand.
    Alteration(crate::alteration::AlterationCode),
}

crate::table::codes! {
/// Refusals of the input as a whole.
pub enum InputCode {
    /// The canonical spelling exceeds the input budget.
    RenderLengthLimit => "input.render_length.limit",
    /// The text is longer than [`crate::MAX_LEN`] bytes.
    LengthLimit => "input.length.limit",
    /// Parsing took more steps than the budget allows.
    StepLimit => "input.step.limit",
    /// The bytes are not UTF-8.
    Utf8Invalid => "input.utf8.invalid",
    /// Text remains after a complete value.
    TrailingInvalid => "input.trailing.invalid",
}
}

impl Code {
    pub(crate) fn is_limit(self) -> bool {
        self.name().ends_with(".limit")
    }

    /// The dotted name, such as `hgvs.base.invalid`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Input(code) => code.name(),
            Self::Accession(code) => code.name(),
            Self::Hgvs(code) => code.name(),
            Self::Nucleotide(code) => code.name(),
            Self::Protein(code) => code.name(),
            Self::Iscn(code) => code.name(),
            Self::Alteration(code) => code.name(),
        }
    }
}

impl<T> Outcome<T> {
    /// The outcome the refusal's code names.
    pub(crate) fn refused(refusal: Refusal) -> Self {
        if refusal.code.name().ends_with(".unsupported") {
            Self::Unsupported(refusal)
        } else if refusal.code.is_limit() {
            Self::LimitExceeded(refusal)
        } else {
            Self::Invalid(refusal)
        }
    }
}

/// Prints the source's length, never its text.
impl<T: fmt::Debug> fmt::Debug for Parsed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parsed")
            .field("source_len", &self.source.len())
            .field("outcome", &self.outcome)
            .finish()
    }
}

/// One flat object: `source`, `outcome`, then `value` or `code`, `span` and `expected`.
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Parsed<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde")]
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("source", &*self.source)?;
        let (outcome, refusal) = match &self.outcome {
            Outcome::Parsed(value) => {
                map.serialize_entry("outcome", "parsed")?;
                map.serialize_entry("value", value)?;
                return map.end();
            }
            Outcome::Unsupported(refusal) => ("unsupported", refusal),
            Outcome::Invalid(refusal) => ("invalid", refusal),
            Outcome::LimitExceeded(refusal) => ("limit_exceeded", refusal),
        };
        map.serialize_entry("outcome", outcome)?;
        map.serialize_entry("code", refusal.code.name())?;
        if let Some(span) = refusal.span {
            map.serialize_entry("span", &[span.start, span.end])?;
        }
        if let Some(expected) = refusal.expected {
            map.serialize_entry("expected", expected)?;
        }
        map.end()
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code.name())
    }
}
impl std::error::Error for Refusal {}
