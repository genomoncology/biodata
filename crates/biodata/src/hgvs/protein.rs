//! HGVS protein changes.
use crate::outcome::Code;
use crate::parse::{Expect, Input, Result, decimal, refuse, reject, step};
use crate::residue::residue;
use crate::{Position1, Residue};
use std::fmt;
use winnow::Parser;
use winnow::combinator::{alt, cut_err, opt, repeat};
use winnow::stream::Stream;
use winnow::token::one_of;

/// A protein change with no spelling choices retained.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ProteinChange {
    /// A replacement residue.
    Substitution {
        /// The reference site.
        site: ProteinSite,
        /// The replacement.
        to: Residue,
    },
    /// A deleted site or range.
    Deletion(ProteinRange),
    /// A duplicated site or range.
    Duplication(ProteinRange),
    /// Residues inserted between two sites.
    Insertion {
        /// The left flank.
        after: ProteinSite,
        /// The right flank.
        before: ProteinSite,
        /// The inserted sequence.
        inserted: Vec<Residue>,
    },
    /// A replacement sequence.
    Delins {
        /// The replaced range.
        range: ProteinRange,
        /// The replacement sequence.
        inserted: Vec<Residue>,
    },
    /// A shifted reading frame.
    Frameshift {
        /// The first changed site.
        site: ProteinSite,
        /// The new residue, if specified.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        to: Option<Residue>,
        /// The distance to termination, if specified.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        stop: Option<Distance>,
    },
    /// A longer protein at either end.
    Extension(Extension),
    /// No change at a site, over a range, or across the protein.
    ///
    /// HGVS writes `p.Ala2=`, `p.Ala2_Gly3=` and `p.=`. A one-site
    /// location serializes as the site itself, as it always has; a range
    /// serializes with `start` and `end`, as a deletion's range does.
    Same(
        #[cfg_attr(feature = "serde", serde(serialize_with = "same_location"))]
        Option<ProteinRange>,
    ),
    /// An unknown change, either at a site or across the protein.
    Unknown(Option<ProteinSite>),
    /// No protein product.
    NoProtein,
}
/// A numbered reference residue.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ProteinSite {
    /// The reference residue.
    pub residue: Residue,
    /// The position counted from one.
    pub position: Position1,
}
/// One site or an inclusive range.
///
/// A written last site lies after the first. [`ProteinRange::new`] checks
/// that order, and the parser refuses text that breaks it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ProteinRange {
    /// The first site.
    pub start: ProteinSite,
    /// The last site, when written.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub end: Option<ProteinSite>,
}
impl ProteinRange {
    /// Build a range from its first and last sites.
    ///
    /// The last position must lie after the first. Equal positions are
    /// refused: one site is written without a last site, through
    /// `ProteinRange::from(site)`.
    pub fn new(
        start: ProteinSite,
        end: ProteinSite,
    ) -> std::result::Result<Self, ProteinRangeError> {
        if end.position <= start.position {
            return Err(ProteinRangeError);
        }
        Ok(Self {
            start,
            end: Some(end),
        })
    }
}
impl From<ProteinSite> for ProteinRange {
    fn from(start: ProteinSite) -> Self {
        Self { start, end: None }
    }
}
/// A protein range's last position does not lie after its first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProteinRangeError;
impl ProteinRangeError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        "hgvs.protein_range.order"
    }
}
impl fmt::Display for ProteinRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for ProteinRangeError {}
/// Serialize an identity's location: a one-site range as the bare site,
/// which keeps the bytes of `p.Ala2=`, and a longer range in full.
#[cfg(feature = "serde")]
#[expect(
    clippy::ref_option,
    reason = "serde passes the variant field by reference"
)]
fn same_location<S: serde::Serializer>(
    location: &Option<ProteinRange>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    match location {
        None => serializer.serialize_none(),
        Some(ProteinRange { start, end: None }) => serializer.serialize_some(start),
        Some(range) => serializer.serialize_some(range),
    }
}
/// An extension at the start or stop.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Extension {
    /// Extension upstream of the initiation site.
    Start {
        /// The reference site.
        site: ProteinSite,
        /// The replacement, when written.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        to: Option<Residue>,
        /// The upstream distance.
        upstream: Distance,
    },
    /// Extension beyond a termination site.
    Stop {
        /// The termination site.
        site: ProteinSite,
        /// The replacement residue.
        to: Residue,
        /// The distance to the new termination.
        stop: Distance,
    },
}
/// A positive distance or an unknown distance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Distance {
    /// The stated distance.
    Known(u32),
    /// `?`.
    Unknown,
}
crate::table::codes! {
/// Refusals from the protein grammar.
pub enum ProteinCode {
    /// A malformed protein edit.
    EditInvalid => "hgvs.protein_edit.invalid",
    /// A malformed residue.
    ResidueInvalid => "hgvs.protein_residue.invalid",
    /// A malformed or zero position or distance.
    PositionInvalid => "hgvs.protein_position.invalid",
    /// Free text cannot describe a typed change.
    OpaqueUnsupported => "hgvs.protein_opaque.unsupported",
    /// A termination-only extension has no value in this representation.
    ExtensionUnsupported => "hgvs.protein_extension.unsupported",
    /// An obsolete frameshift termination marker is not supported.
    LegacyStopUnsupported => "hgvs.protein_legacy_stop.unsupported",
    /// Alternative residues have no value in this representation.
    ResidueChoiceUnsupported => "hgvs.protein_residue_choice.unsupported",
    /// Uncertain absence has no value in this representation.
    UncertainNoProductUnsupported => "hgvs.protein_uncertain_no_product.unsupported",
    /// Alleles and multiple products are not represented yet.
    AlleleUnsupported => "hgvs.protein_allele.unsupported",
    /// Repeated residues are not represented yet.
    RepeatUnsupported => "hgvs.protein_repeat.unsupported",
    /// An insertion specified by count is not represented yet.
    InsertedCountUnsupported => "hgvs.protein_inserted_count.unsupported",
}
}

pub(super) const EDIT: Expect = Expect {
    code: Code::Protein(ProteinCode::EditInvalid),
    label: "protein edit",
};
const RESIDUE: Expect = Expect {
    code: Code::Protein(ProteinCode::ResidueInvalid),
    label: "residue",
};
const POSITION: Expect = Expect {
    code: Code::Protein(ProteinCode::PositionInvalid),
    label: "positive position or distance",
};

fn positive(input: &mut Input<'_>) -> Result<Position1> {
    step(input)?;
    let start = input.checkpoint();
    let value = cut_err(decimal).context(POSITION).parse_next(input)?;
    match Position1::new(value) {
        Ok(position) => Ok(position),
        Err(_) => {
            input.reset(&start);
            reject(input, Some(POSITION))
        }
    }
}
fn site(input: &mut Input<'_>) -> Result<ProteinSite> {
    step(input)?;
    let residue = residue.context(RESIDUE).parse_next(input)?;
    let position = positive(input)?;
    Ok(ProteinSite { residue, position })
}
fn distance(input: &mut Input<'_>) -> Result<Distance> {
    step(input)?;
    alt((
        '?'.value(Distance::Unknown),
        positive.map(|position| Distance::Known(position.get())),
    ))
    .parse_next(input)
}
fn stop(input: &mut Input<'_>) -> Result<Distance> {
    alt(("Ter", "*")).parse_next(input)?;
    cut_err(distance).parse_next(input)
}
fn sequence(input: &mut Input<'_>) -> Result<Vec<Residue>> {
    step(input)?;
    let count = refuse(
        Code::Protein(ProteinCode::InsertedCountUnsupported),
        alt(("X[".void(), winnow::ascii::digit1.void())),
    );
    let values = cut_err(alt((
        count,
        repeat(1.., residue).fold(Vec::new, |mut values: Vec<Residue>, value| {
            values.push(value);
            values
        }),
    )))
    .context(RESIDUE)
    .parse_next(input)?;
    if values
        .iter()
        .rev()
        .skip(1)
        .any(|value| *value == Residue::Ter)
    {
        return reject(input, Some(EDIT));
    }
    Ok(values)
}
#[derive(Clone, Copy)]
enum Marker {
    Deletion,
    Duplication,
    Insertion,
    Delins,
    Same,
    Unknown,
}
pub(crate) fn body(input: &mut Input<'_>) -> Result<ProteinChange> {
    let value = edit(input)?;
    if input.starts_with("(;)") {
        return refuse(Code::Protein(ProteinCode::AlleleUnsupported), "(;)").parse_next(input);
    }
    Ok(value)
}
#[expect(
    clippy::too_many_lines,
    reason = "preserve the reviewed parser layout and hot-path behavior"
)]
fn edit(input: &mut Input<'_>) -> Result<ProteinChange> {
    step(input)?;
    if input.chars().next().is_some_and(|c| c.is_ascii_digit()) && input.contains('[') {
        return refuse(
            Code::Protein(ProteinCode::RepeatUnsupported),
            winnow::ascii::digit1,
        )
        .parse_next(input);
    }
    let text: &str = input;
    let text = text.strip_suffix(')').unwrap_or(text);
    if text.len() >= 2
        && text.bytes().all(|b| b.is_ascii_lowercase())
        && !["fs", "ext", "del", "dup", "ins", "con", "inv"]
            .iter()
            .any(|marker| text.contains(marker))
    {
        return refuse(
            Code::Protein(ProteinCode::OpaqueUnsupported),
            winnow::ascii::alpha1,
        )
        .parse_next(input);
    }
    if let Some(whole) = opt(one_of(['=', '?', '0'])).parse_next(input)? {
        if whole == '0' && input.starts_with('?') {
            return refuse(
                Code::Protein(ProteinCode::UncertainNoProductUnsupported),
                '?',
            )
            .parse_next(input);
        }
        return Ok(match whole {
            '=' => ProteinChange::Same(None),
            '?' => ProteinChange::Unknown(None),
            _ => ProteinChange::NoProtein,
        });
    }
    if input.starts_with('[') {
        return refuse(Code::Protein(ProteinCode::AlleleUnsupported), '[').parse_next(input);
    }
    let start = site(input)?;
    let end = opt(('_', cut_err(site)).map(|(_, site)| site)).parse_next(input)?;
    let range = match end {
        None => ProteinRange::from(start),
        Some(end) => match ProteinRange::new(start, end) {
            Ok(range) => range,
            Err(ProteinRangeError) => return reject(input, Some(EDIT)),
        },
    };
    let repeat = refuse(Code::Protein(ProteinCode::RepeatUnsupported), '[');
    let marker = alt((
        repeat,
        "delins".value(Marker::Delins),
        "del".value(Marker::Deletion),
        "dup".value(Marker::Duplication),
        "ins".value(Marker::Insertion),
        "=".value(Marker::Same),
        "?".value(Marker::Unknown),
    ));
    if let Some(marker) = opt(marker).parse_next(input)? {
        if stop_misplaced(marker, &range) {
            return reject(input, Some(EDIT));
        }
        return Ok(match marker {
            Marker::Deletion => ProteinChange::Deletion(range),
            Marker::Duplication => ProteinChange::Duplication(range),
            Marker::Delins => ProteinChange::Delins {
                range,
                inserted: sequence(input)?,
            },
            Marker::Insertion => {
                let before = required(range.end, input)?;
                if range.start.position.checked_add(1).ok() != Some(before.position) {
                    return reject(input, Some(EDIT));
                }
                ProteinChange::Insertion {
                    after: range.start,
                    before,
                    inserted: sequence(input)?,
                }
            }
            Marker::Same => ProteinChange::Same(Some(range)),
            Marker::Unknown => {
                if range.end.is_some() {
                    return reject(input, Some(EDIT));
                }
                ProteinChange::Unknown(Some(range.start))
            }
        });
    }
    if range.end.is_some() {
        return reject(input, Some(EDIT));
    }
    let site = range.start;
    if input.starts_with('(') && input.contains('^') {
        return refuse(Code::Protein(ProteinCode::ResidueChoiceUnsupported), '(').parse_next(input);
    }
    let to = opt(residue).parse_next(input)?;
    if opt("fs").parse_next(input)?.is_some() {
        if input.starts_with("X?") {
            return refuse(Code::Protein(ProteinCode::LegacyStopUnsupported), "X?")
                .parse_next(input);
        }
        let stop = opt(stop).parse_next(input)?;
        if site.residue == Residue::Ter
            || to == Some(Residue::Ter)
            || stop == Some(Distance::Known(1))
        {
            return reject(input, Some(EDIT));
        }
        return Ok(ProteinChange::Frameshift { site, to, stop });
    }
    if input.starts_with("ext") && !input.starts_with("ext-") && site.residue != Residue::Ter {
        return refuse(Code::Protein(ProteinCode::ExtensionUnsupported), "ext").parse_next(input);
    }
    if opt("ext").parse_next(input)?.is_some() {
        if opt('-').parse_next(input)?.is_some() {
            if site.residue != Residue::Met || site.position.get() != 1 {
                return reject(input, Some(EDIT));
            }
            let upstream = distance(input)?;
            return Ok(ProteinChange::Extension(Extension::Start {
                site,
                to,
                upstream,
            }));
        }
        let to = required(to, input)?;
        let stop = stop(input)?;
        return Ok(ProteinChange::Extension(Extension::Stop { site, to, stop }));
    }
    // A plain substitution must have a replacement residue.
    let to = required(to, input)?;
    if site.residue == Residue::Ter || site.residue == to {
        return reject(input, Some(EDIT));
    }
    Ok(ProteinChange::Substitution { site, to })
}

/// Whether a stop sits where the edit cannot use it. Deletions,
/// duplications, deletion-insertions, insertions and range identities
/// refuse a range starting at a stop. Only deletions, duplications and
/// deletion-insertions also refuse one ending at a stop. A one-site
/// identity or unknown change may name a stop.
fn stop_misplaced(marker: Marker, range: &ProteinRange) -> bool {
    let (start, end) = match marker {
        Marker::Deletion | Marker::Duplication | Marker::Delins => (true, true),
        Marker::Insertion => (true, false),
        Marker::Same => (range.end.is_some(), false),
        Marker::Unknown => (false, false),
    };
    (start && range.start.residue == Residue::Ter)
        || (end
            && range
                .end
                .as_ref()
                .is_some_and(|end| end.residue == Residue::Ter))
}

fn required<T>(value: Option<T>, input: &mut Input<'_>) -> Result<T> {
    match value {
        Some(value) => Ok(value),
        None => reject(input, Some(EDIT)),
    }
}
impl fmt::Display for ProteinSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.residue, f)?;
        write!(f, "{}", self.position)
    }
}
impl fmt::Display for ProteinRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.start, f)?;
        if let Some(end) = &self.end {
            f.write_str("_")?;
            fmt::Display::fmt(end, f)?;
        }
        Ok(())
    }
}
impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Known(n) => write!(f, "{n}"),
            Self::Unknown => f.write_str("?"),
        }
    }
}
fn residues(f: &mut fmt::Formatter<'_>, inserted: &[Residue]) -> fmt::Result {
    for residue in inserted {
        fmt::Display::fmt(residue, f)?;
    }
    Ok(())
}
impl fmt::Display for ProteinChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Substitution { site, to } => {
                fmt::Display::fmt(site, f)?;
                fmt::Display::fmt(to, f)
            }
            Self::Deletion(range) | Self::Duplication(range) => {
                fmt::Display::fmt(range, f)?;
                f.write_str(if matches!(self, Self::Deletion(_)) {
                    "del"
                } else {
                    "dup"
                })
            }
            Self::Insertion {
                after,
                before,
                inserted,
            } => {
                fmt::Display::fmt(after, f)?;
                f.write_str("_")?;
                fmt::Display::fmt(before, f)?;
                f.write_str("ins")?;
                residues(f, inserted)
            }
            Self::Delins { range, inserted } => {
                fmt::Display::fmt(range, f)?;
                f.write_str("delins")?;
                residues(f, inserted)
            }
            Self::Frameshift { site, to, stop } => {
                fmt::Display::fmt(site, f)?;
                if let Some(to) = to {
                    fmt::Display::fmt(to, f)?;
                }
                f.write_str("fs")?;
                if let Some(stop) = stop {
                    fmt::Display::fmt(&Residue::Ter, f)?;
                    write!(f, "{stop}")?;
                }
                Ok(())
            }
            Self::Extension(extension) => match extension {
                Extension::Start { site, to, upstream } => {
                    fmt::Display::fmt(site, f)?;
                    if let Some(to) = to {
                        fmt::Display::fmt(to, f)?;
                    }
                    write!(f, "ext-{upstream}")
                }
                Extension::Stop { site, to, stop } => {
                    fmt::Display::fmt(site, f)?;
                    fmt::Display::fmt(to, f)?;
                    f.write_str("ext")?;
                    fmt::Display::fmt(&Residue::Ter, f)?;
                    write!(f, "{stop}")
                }
            },
            Self::Same(range) => {
                if let Some(range) = range {
                    fmt::Display::fmt(range, f)?;
                }
                f.write_str("=")
            }
            Self::Unknown(site) => {
                if let Some(site) = site {
                    fmt::Display::fmt(site, f)?;
                }
                f.write_str("?")
            }
            Self::NoProtein => f.write_str("0"),
        }
    }
}
