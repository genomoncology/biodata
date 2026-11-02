//! Source-stated gene locations retain HGNC spelling and parsed cytobands.
use crate::{Cytoband, SourceText};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// HGNC location text with a parsed interpretation when its syntax is known.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "String", into = "String"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "String"))]
pub struct GeneLocation {
    text: SourceText,
    kind: GeneLocationKind,
}

/// Syntactic forms found in HGNC's location field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeneLocationKind {
    /// One band followed by any additional stated bands.
    Cytobands {
        /// First stated band.
        first: Cytoband,
        /// Remaining stated bands, in source order.
        additional: Box<[Cytoband]>,
    },
    /// Stated range endpoints, without ordering or biological inference.
    Range {
        /// First stated endpoint.
        start: Cytoband,
        /// Last stated endpoint with its chromosome expanded.
        end: Cytoband,
    },
    /// HGNC's `mitochondria` value.
    Mitochondrial,
    /// Source text outside the recognized syntax.
    Unparsed,
}
impl GeneLocation {
    /// Borrow the unchanged publisher text.
    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
    /// Borrow the interpretation assigned when the text entered the model.
    pub fn kind(&self) -> &GeneLocationKind {
        &self.kind
    }
}
impl From<String> for GeneLocation {
    fn from(value: String) -> Self {
        let kind = parse(&value).unwrap_or(GeneLocationKind::Unparsed);
        Self {
            text: value.into(),
            kind,
        }
    }
}
impl From<&str> for GeneLocation {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}
impl From<GeneLocation> for String {
    fn from(value: GeneLocation) -> Self {
        value.as_str().into()
    }
}
impl std::fmt::Display for GeneLocation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
fn parse(value: &str) -> Option<GeneLocationKind> {
    if value == "mitochondria" {
        return Some(GeneLocationKind::Mitochondrial);
    }
    if let Some((start, end)) = value.split_once('-') {
        let start = Cytoband::new(start).ok()?;
        let chromosome = start.as_str().split_once(['p', 'q'])?.0;
        let end = if end.starts_with(['p', 'q']) {
            Cytoband::new(&format!("{chromosome}{end}")).ok()?
        } else {
            Cytoband::new(end).ok()?
        };
        return Some(GeneLocationKind::Range { start, end });
    }
    let mut bands = value.split(" and ");
    let first = Cytoband::new(bands.next()?).ok()?;
    let additional = bands
        .map(Cytoband::new)
        .collect::<Result<Box<[_]>, _>>()
        .ok()?;
    Some(GeneLocationKind::Cytobands { first, additional })
}
