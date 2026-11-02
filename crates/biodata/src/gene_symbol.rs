//! A checked source gene symbol.
use crate::{Error, text_value};
use std::hash::{Hash, Hasher};
use std::{fmt, str::FromStr};

/// Source spelling of a gene symbol, with ASCII case-insensitive identity.
#[derive(Clone, Debug)]
pub struct GeneSymbol(Box<str>);
impl GeneSymbol {
    /// Maximum accepted symbol length in bytes.
    pub const MAX_LEN: usize = 255;
    /// Check the approved-symbol alphabet without inferring HGNC membership.
    pub fn new(value: &str) -> Result<Self, Error> {
        if value.is_empty()
            || value.len() > Self::MAX_LEN
            || !value
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
            || !value
                .bytes()
                .all(|ch| ch.is_ascii_alphanumeric() || b"-._/@".contains(&ch))
        {
            return Err(Error::GeneSymbolInvalid);
        }
        Ok(Self(value.into()))
    }
    /// Borrow the preserved source spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl PartialEq for GeneSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}
impl Eq for GeneSymbol {}
impl Hash for GeneSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for ch in self.0.bytes() {
            ch.to_ascii_lowercase().hash(state);
        }
    }
}
impl FromStr for GeneSymbol {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::new(value)
    }
}
impl fmt::Display for GeneSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    GeneSymbol,
    "GeneSymbol",
    format!(r"^[A-Za-z][A-Za-z0-9._/@-]{{0,{}}}$", Self::MAX_LEN - 1),
    "Source spelling of a gene symbol, with ASCII case-insensitive identity."
);
use crate::parse::{Input, Result as ParseResult, step};
use winnow::{
    Parser,
    token::{any, take_while},
};

/// Reads at most the stated symbol length, leaving a notation suffix on the cursor.
pub(crate) fn gene_symbol_prefix(input: &mut Input<'_>, length: usize) -> ParseResult<GeneSymbol> {
    step(input)?;
    let length = length.min(GeneSymbol::MAX_LEN);
    let symbol = (
        any.verify(char::is_ascii_alphabetic),
        take_while(0..=length.saturating_sub(1), |c: char| {
            c.is_ascii_alphanumeric() || c == '-'
        }),
    )
        .take()
        .parse_next(input)?;
    Ok(GeneSymbol(symbol.into()))
}
