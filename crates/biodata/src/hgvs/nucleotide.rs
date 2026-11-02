//! HGVS nucleotide changes.
use super::{Nucleotide, Nucleotides};

use crate::Position1;
use crate::outcome::Code;
use crate::parse::{Expect, Input, Result, decimal, refuse, reject, step};
use std::fmt;
use std::num::NonZeroI32;
use winnow::Parser;
use winnow::combinator::{alt, cut_err, delimited, opt, preceded, repeat, terminated};
use winnow::token::{any, one_of};

/// A nucleotide change.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct NaChange {
    /// The sequence kind.
    pub molecule: Molecule,
    /// What changed.
    pub body: NaBody,
}

crate::table::text_table!(Molecule, "The kind of sequence.", |text| text, {
    Genomic => "g.",
    Coding => "c.",
    Noncoding => "n.",
    Rna => "r.",
    Mitochondrial => "m.",
    Circular => "o.",
}, "snake_case");

/// What a nucleotide description states.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum NaBody {
    /// An edit at a location.
    Edit {
        /// The edited positions.
        location: Location,
        /// The sequence edit.
        edit: NaEdit,
    },
    /// A whole sequence outcome.
    Whole(Whole),
}

crate::table::text_table!(Whole, "A whole sequence outcome.", |text| text, {
    Unknown => "?",
    Same => "=",
    NoProduct => "0",
    Splice => "spl",
}, "snake_case");

/// Where an edit applies.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Location {
    /// One position.
    Point(Position),
    /// An exact range.
    Range(Position, Position),
    /// A range with uncertain bounds.
    Uncertain {
        /// Start bound.
        start: Bound,
        /// End bound.
        end: Bound,
    },
}

/// One bound of a location.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Bound {
    /// An exact bound.
    Exact(Position),
    /// A bound between two positions.
    Between(Position, Position),
}

/// One position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Position {
    /// The anchor.
    pub anchor: PositionAnchor,
    /// A nonzero intronic offset.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub offset: Option<NonZeroI32>,
}

/// A checked coding or noncoding transcript anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Anchor {
    /// A position before the coding start.
    Upstream {
        /// One-based distance before the coding start.
        distance: Position1,
    },
    /// A position inside the coding sequence.
    Coding {
        /// One-based coding position.
        position: Position1,
    },
    /// A position after the coding stop.
    Downstream {
        /// One-based distance after the coding stop.
        distance: Position1,
    },
    /// A position on a noncoding transcript.
    Noncoding {
        /// One-based noncoding transcript position.
        position: Position1,
    },
}

/// An HGVS grammar position, including locations outside c. and n. notation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionAnchor {
    /// A known c. or n. transcript anchor.
    Transcript(Anchor),
    /// A numbered position in another molecule notation.
    Absolute(Position1),
    /// A numbered RNA grammar position with no assigned transcript anchor.
    Rna(Position1),
    /// A written negative position outside c. and n. notation.
    Upstream(Position1),
    /// A written negative RNA grammar position.
    RnaUpstream(Position1),
    /// A written stop-relative position outside c. and n. notation.
    Downstream(Position1),
    /// A written stop-relative RNA grammar position.
    RnaDownstream(Position1),
    /// `?`, which does not identify a known anchor.
    Unknown,
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for PositionAnchor {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Transcript(anchor) => serde::Serialize::serialize(anchor, serializer),
            Self::Absolute(position) | Self::Rna(position) => {
                serializer.serialize_newtype_variant("PositionAnchor", 0, "number", &position.get())
            }
            Self::Upstream(distance) | Self::RnaUpstream(distance) => serializer
                .serialize_newtype_variant("PositionAnchor", 1, "upstream", &distance.get()),
            Self::Downstream(distance) | Self::RnaDownstream(distance) => serializer
                .serialize_newtype_variant("PositionAnchor", 2, "after_stop", &distance.get()),
            Self::Unknown => serializer.serialize_unit_variant("PositionAnchor", 3, "unknown"),
        }
    }
}

/// A nucleotide edit.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum NaEdit {
    /// One base replaced by another.
    Substitution {
        /// Reference base.
        from: Nucleotide,
        /// Replacement base.
        to: Nucleotide,
    },
    /// Deleted bases, if stated.
    Deletion(Option<Nucleotides>),
    /// Duplicated bases, if stated.
    Duplication(Option<Nucleotides>),
    /// Inserted bases.
    Insertion(Nucleotides),
    /// Replacement bases.
    Delins {
        /// Deleted bases, when stated.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        deleted: Option<Nucleotides>,
        /// Inserted replacement bases.
        inserted: Nucleotides,
    },
    /// Inverted sequence.
    Inversion,
    /// Unchanged sequence.
    Same,
    /// Unknown change.
    Unknown,
}

crate::table::codes! {
/// Refusals of nucleotide forms.
pub enum NaCode {
    /// An edit is malformed.
    EditInvalid => "hgvs.edit.invalid",
    /// A ranged identity with stated bases has unresolved compatibility.
    RangeIdentityUnsupported => "hgvs.range_identity.unsupported",
    /// A position is malformed.
    PositionInvalid => "hgvs.position.invalid",
    /// A base is malformed.
    BaseInvalid => "hgvs.base.invalid",
    /// An allele is not supported.
    AlleleUnsupported => "hgvs.allele.unsupported",
    /// A mosaic is not supported.
    MosaicUnsupported => "hgvs.mosaic.unsupported",
    /// A chimera is not supported.
    ChimericUnsupported => "hgvs.chimeric.unsupported",
    /// A fusion is not supported.
    FusionUnsupported => "hgvs.fusion.unsupported",
    /// A conversion is not supported.
    ConversionUnsupported => "hgvs.conversion.unsupported",
    /// A repeat is not supported.
    RepeatUnsupported => "hgvs.repeat.unsupported",
    /// An inserted length is not supported.
    InsertedLengthUnsupported => "hgvs.inserted_length.unsupported",
    /// An uncertain no-product RNA outcome is not supported.
    RnaOutcomeUnsupported => "hgvs.rna_outcome.unsupported",
    /// Fully unknown inversion bounds have unresolved grammar.
    UnknownInversionUnsupported => "hgvs.unknown_inversion.unsupported",
    /// An inserted reference is not supported.
    InsertedReferenceUnsupported => "hgvs.inserted_reference.unsupported",
}
}

const EDIT: Expect = Expect {
    code: Code::Nucleotide(NaCode::EditInvalid),
    label: "edit",
};
const POSITION: Expect = Expect {
    code: Code::Nucleotide(NaCode::PositionInvalid),
    label: "position",
};
const BASE: Expect = Expect {
    code: Code::Nucleotide(NaCode::BaseInvalid),
    label: "base",
};

pub(crate) fn change(input: &mut Input<'_>) -> Result<(NaChange, bool)> {
    step(input)?;
    let molecule = alt((
        Molecule::parse,
        refuse(Code::Nucleotide(NaCode::AlleleUnsupported), "["),
    ))
    .parse_next(input)?;
    let (body, predicted) = cut_err(|i: &mut Input<'_>| predicted_body(i, molecule))
        .context(EDIT)
        .parse_next(input)?;
    Ok((NaChange { molecule, body }, predicted))
}

fn predicted_body(input: &mut Input<'_>, molecule: Molecule) -> Result<(NaBody, bool)> {
    let remaining: &str = input;
    // Only the matching close belongs to this opening. A caller's closing
    // parenthesis must not turn an uncertain location into a predicted body.
    let mut depth = 0;
    let mut predicted = false;
    if remaining.starts_with('(') {
        // Retain the malformed-prediction refusal when its outer close is missing.
        predicted = remaining.ends_with(')');
        for (index, byte) in remaining.bytes().enumerate() {
            match byte {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                predicted = remaining
                    .get(index + 1..)
                    .is_some_and(|tail| tail.is_empty() || tail.starts_with([')', ' ']));
                break;
            }
        }
    }
    if predicted {
        delimited(
            '(',
            |i: &mut Input<'_>| body(i, molecule, true),
            cut_err(')'),
        )
        .parse_next(input)
        .map(|body| (body, true))
    } else {
        body(input, molecule, false).map(|body| (body, false))
    }
}

fn unsupported(input: &mut Input<'_>) -> Result<()> {
    alt((
        refuse(Code::Nucleotide(NaCode::AlleleUnsupported), "(;)"),
        refuse(Code::Nucleotide(NaCode::ChimericUnsupported), "//"),
        refuse(Code::Nucleotide(NaCode::MosaicUnsupported), "/"),
        refuse(Code::Nucleotide(NaCode::FusionUnsupported), "::"),
        refuse(Code::Nucleotide(NaCode::ConversionUnsupported), "con"),
        ().value(()),
    ))
    .parse_next(input)
}

#[expect(
    clippy::too_many_lines,
    reason = "preserve the reviewed parser layout and hot-path behavior"
)]
fn body(input: &mut Input<'_>, molecule: Molecule, predicted: bool) -> Result<NaBody> {
    step(input)?;
    opt(refuse::<_, ()>(
        Code::Nucleotide(NaCode::AlleleUnsupported),
        "[",
    ))
    .parse_next(input)?;
    unsupported(input)?;
    if molecule == Molecule::Rna {
        opt(refuse::<_, ()>(
            Code::Nucleotide(NaCode::RnaOutcomeUnsupported),
            "0?",
        ))
        .parse_next(input)?;
    }
    opt(preceded(
        alt(("pter", "qter")),
        preceded(
            winnow::token::take_until(0.., "::"),
            refuse::<_, ()>(Code::Nucleotide(NaCode::FusionUnsupported), "::"),
        ),
    ))
    .parse_next(input)?;
    let whole = Whole::parse.verify(|whole| {
        molecule == Molecule::Rna || !matches!(whole, Whole::NoProduct | Whole::Splice)
    });
    // A whole outcome must end the body, before a prediction's closing parenthesis.
    let whole = opt(terminated(
        whole,
        winnow::combinator::peek(alt((winnow::combinator::eof.void(), ")".void()))),
    ))
    .parse_next(input)?;
    if let Some(whole) = whole {
        return Ok(NaBody::Whole(whole));
    }
    let location = (|i: &mut Input<'_>| location(i, molecule))
        .context(POSITION)
        .parse_next(input)?;
    unsupported(input)?;
    if matches!(location, Location::Range(_, _)) {
        let remaining: &str = input;
        let asserted = remaining.strip_suffix('=').or_else(|| {
            if predicted {
                remaining.strip_suffix("=)")
            } else {
                None
            }
        });
        if asserted.is_some_and(|bases| {
            bases.len() > 1
                && bases
                    .chars()
                    .all(|base| Nucleotide::from_letter(base).is_some())
        }) {
            return refuse(
                Code::Nucleotide(NaCode::RangeIdentityUnsupported),
                winnow::ascii::alpha1,
            )
            .parse_next(input);
        }
    }
    let edit = edit(input, molecule)?;
    unsupported(input)?;
    if matches!(
        location,
        Location::Point(Position {
            anchor: PositionAnchor::Unknown,
            ..
        })
    ) && matches!(edit, NaEdit::Unknown)
    {
        return reject(input, Some(EDIT));
    }
    if matches!(edit, NaEdit::Inversion) {
        if matches!(&location, Location::Point(_)) {
            return reject(input, Some(EDIT));
        }
        // Known equal endpoints already fail location parsing. Equal unknown
        // tokens do not establish a single coordinate or a valid inversion.
        if matches!(&location, Location::Range(a, b) if a == b)
            || matches!(&location, Location::Uncertain { start, end }
                if start == end && matches!(start, Bound::Between(a, b) if a == b))
        {
            return refuse(
                Code::Nucleotide(NaCode::UnknownInversionUnsupported),
                winnow::combinator::empty,
            )
            .parse_next(input);
        }
    }
    let insertion = match &location {
        Location::Range(a, b) => distance(a, b).is_some_and(|d| d.adjacent),
        Location::Uncertain { start, end } if start == end => {
            matches!(start, Bound::Between(a, b) if distance(a, b).is_some())
        }
        _ => false,
    };
    if matches!(edit, NaEdit::Insertion(_)) && !insertion {
        return reject(input, Some(EDIT));
    }
    let stated = match &edit {
        NaEdit::Deletion(bases) | NaEdit::Duplication(bases) => bases.as_ref(),
        NaEdit::Delins { deleted, .. } => deleted.as_ref(),
        _ => None,
    };
    let length = match &location {
        Location::Point(p) => distance(p, p).and_then(|d| d.steps),
        Location::Range(a, b) => distance(a, b).and_then(|d| d.steps),
        Location::Uncertain { .. } => None,
    }
    .map(|steps| steps + 1);
    if stated.is_some_and(|bases| length.is_some_and(|n| n != bases.as_slice().len() as i64)) {
        return reject(input, Some(EDIT));
    }
    Ok(NaBody::Edit { location, edit })
}

fn position(input: &mut Input<'_>, molecule: Molecule) -> Result<Position> {
    step(input)?;
    let positive = || decimal::<u32>.verify_map(|n| Position1::new(n).ok());
    let anchor = alt((
        preceded('-', cut_err(positive())).map(|distance| {
            if molecule == Molecule::Coding {
                PositionAnchor::Transcript(Anchor::Upstream { distance })
            } else if molecule == Molecule::Rna {
                PositionAnchor::RnaUpstream(distance)
            } else {
                PositionAnchor::Upstream(distance)
            }
        }),
        preceded('*', cut_err(positive()))
            .verify(|_| matches!(molecule, Molecule::Coding | Molecule::Rna))
            .map(|distance| {
                if molecule == Molecule::Coding {
                    PositionAnchor::Transcript(Anchor::Downstream { distance })
                } else {
                    PositionAnchor::RnaDownstream(distance)
                }
            }),
        '?'.value(PositionAnchor::Unknown),
        positive().map(|position| match molecule {
            Molecule::Coding => PositionAnchor::Transcript(Anchor::Coding { position }),
            Molecule::Noncoding => PositionAnchor::Transcript(Anchor::Noncoding { position }),
            Molecule::Rna => PositionAnchor::Rna(position),
            _ => PositionAnchor::Absolute(position),
        }),
    ))
    .parse_next(input)?;
    let offset = opt((
        one_of(['+', '-']),
        cut_err(decimal::<i32>.verify(|n| *n > 0)).context(POSITION),
    )
        .map(|(sign, n)| NonZeroI32::new(if sign == '-' { -n } else { n })))
    .verify(|offset| {
        offset.is_none()
            || matches!(
                molecule,
                Molecule::Coding | Molecule::Noncoding | Molecule::Rna
            )
    })
    .parse_next(input)?
    .flatten();
    Ok(Position { anchor, offset })
}

fn bound(input: &mut Input<'_>, molecule: Molecule) -> Result<Bound> {
    step(input)?;
    alt((
        delimited(
            '(',
            cut_err((
                |i: &mut Input<'_>| position(i, molecule),
                '_',
                |i: &mut Input<'_>| position(i, molecule),
            )),
            cut_err(')'),
        )
        .verify(|(a, _, b)| distance(a, b).is_none_or(|d| d.order.is_lt()))
        .map(|(a, _, b)| Bound::Between(a, b)),
        (|i: &mut Input<'_>| position(i, molecule)).map(Bound::Exact),
    ))
    .parse_next(input)
}

fn location(input: &mut Input<'_>, molecule: Molecule) -> Result<Location> {
    step(input)?;
    let start = bound(input, molecule)?;
    let end = opt(preceded(
        '_',
        cut_err(|i: &mut Input<'_>| bound(i, molecule)),
    ))
    .parse_next(input)?;
    // Refuse disjoint reversed bounds without guessing within overlapping uncertainty.
    let earliest = match &start {
        Bound::Exact(position) | Bound::Between(position, _) => position,
    };
    let latest = end.as_ref().map(|bound| match bound {
        Bound::Exact(position) | Bound::Between(_, position) => position,
    });
    if latest.is_some_and(|latest| distance(earliest, latest).is_some_and(|d| !d.order.is_lt())) {
        return reject(input, Some(POSITION));
    }
    Ok(match (start, end) {
        (Bound::Exact(a), None) => Location::Point(a),
        (Bound::Exact(a), Some(Bound::Exact(b))) => Location::Range(a, b),
        (start, end) => Location::Uncertain {
            start: start.clone(),
            end: end.unwrap_or(start),
        },
    })
}

struct PositionDistance {
    order: std::cmp::Ordering,
    steps: Option<i64>,
    adjacent: bool,
}

// Ordering is known across coding anchors even when sequence context is needed
// for a distance. Offsets order positions only within their own anchor.
fn distance(a: &Position, b: &Position) -> Option<PositionDistance> {
    fn anchor(anchor: PositionAnchor) -> Option<(u8, i64)> {
        match anchor {
            PositionAnchor::Transcript(Anchor::Upstream { distance })
            | PositionAnchor::Upstream(distance)
            | PositionAnchor::RnaUpstream(distance) => Some((0, -i64::from(distance.get()))),
            PositionAnchor::Transcript(
                Anchor::Coding { position } | Anchor::Noncoding { position },
            )
            | PositionAnchor::Absolute(position)
            | PositionAnchor::Rna(position) => Some((1, i64::from(position.get()))),
            PositionAnchor::Transcript(Anchor::Downstream { distance })
            | PositionAnchor::Downstream(distance)
            | PositionAnchor::RnaDownstream(distance) => Some((2, i64::from(distance.get()))),
            PositionAnchor::Unknown => None,
        }
    }
    let left = anchor(a.anchor)?;
    let right = anchor(b.anchor)?;
    let offset = |p: &Position| i64::from(p.offset.map_or(0, NonZeroI32::get));
    let order = (left, offset(a)).cmp(&(right, offset(b)));
    let steps = if a.anchor == b.anchor {
        Some(offset(b) - offset(a))
    } else if a.offset.is_none() && b.offset.is_none() {
        match (left.0, right.0) {
            (x, y) if x == y => Some(right.1 - left.1),
            (0, 1) => Some(right.1 - left.1 - 1),
            (1, 0) => Some(right.1 - left.1 + 1),
            _ => None,
        }
    } else {
        None
    };
    let adjacent = steps.map_or(
        a.offset.is_some()
            || b.offset.is_some()
            || matches!(
                (a.anchor, b.anchor),
                (PositionAnchor::Transcript(Anchor::Coding { .. }), PositionAnchor::Transcript(Anchor::Downstream { distance }))
                    | (PositionAnchor::Rna(_), PositionAnchor::RnaDownstream(distance))
                    if distance.get() == 1
            ),
        |steps| steps == 1,
    );
    Some(PositionDistance {
        order,
        steps,
        adjacent,
    })
}

fn nucleotide(input: &mut Input<'_>, molecule: Molecule) -> Result<Nucleotide> {
    step(input)?;
    if molecule != Molecule::Rna {
        return any
            .verify_map(Nucleotide::from_letter)
            .context(BASE)
            .parse_next(input);
    }
    any.verify_map(|c: char| {
        if !c.is_ascii_lowercase() || c == 't' {
            return None;
        }
        Nucleotide::from_rna_letter(c.to_ascii_uppercase())
    })
    .context(BASE)
    .parse_next(input)
}

fn bases(input: &mut Input<'_>, molecule: Molecule) -> Result<Nucleotides> {
    repeat(1.., |i: &mut Input<'_>| nucleotide(i, molecule))
        .fold(Vec::new, |mut values, base| {
            values.push(base);
            values
        })
        .verify_map(|values: Vec<Nucleotide>| Nucleotides::from_nucleotides(values))
        .parse_next(input)
}

fn inserted(input: &mut Input<'_>, molecule: Molecule) -> Result<Nucleotides> {
    alt((
        refuse(Code::Nucleotide(NaCode::InsertedLengthUnsupported), "("),
        refuse(Code::Nucleotide(NaCode::InsertedReferenceUnsupported), "["),
        |i: &mut Input<'_>| bases(i, molecule),
    ))
    .parse_next(input)
    .and_then(|value| {
        opt(refuse::<_, ()>(
            Code::Nucleotide(NaCode::InsertedLengthUnsupported),
            "[",
        ))
        .parse_next(input)?;
        Ok(value)
    })
}

fn edit(input: &mut Input<'_>, molecule: Molecule) -> Result<NaEdit> {
    step(input)?;
    let edit = alt((
        refuse(Code::Nucleotide(NaCode::RepeatUnsupported), "["),
        preceded("del", |i: &mut Input<'_>| {
            let deleted = opt(|i: &mut Input<'_>| bases(i, molecule)).parse_next(i)?;
            if opt("ins").parse_next(i)?.is_some() {
                let inserted = cut_err(|i: &mut Input<'_>| inserted(i, molecule)).parse_next(i)?;
                Ok(NaEdit::Delins { deleted, inserted })
            } else {
                Ok(NaEdit::Deletion(deleted))
            }
        }),
        preceded("dup", opt(|i: &mut Input<'_>| bases(i, molecule))).map(NaEdit::Duplication),
        preceded("ins", cut_err(|i: &mut Input<'_>| inserted(i, molecule))).map(NaEdit::Insertion),
        "inv".value(NaEdit::Inversion),
        "=".value(NaEdit::Same),
        "?".value(NaEdit::Unknown),
        |i: &mut Input<'_>| {
            let from = nucleotide(i, molecule)?;
            opt(terminated(
                winnow::ascii::alpha1,
                winnow::combinator::peek("["),
            ))
            .parse_next(i)?;
            opt(refuse::<_, ()>(
                Code::Nucleotide(NaCode::RepeatUnsupported),
                "[",
            ))
            .parse_next(i)?;
            if opt('=').parse_next(i)?.is_some() {
                return Ok(NaEdit::Same);
            }
            cut_err('>').context(EDIT).parse_next(i)?;
            let to = cut_err(|i: &mut Input<'_>| nucleotide(i, molecule)).parse_next(i)?;
            if from == to {
                return reject(i, Some(EDIT));
            }
            Ok(NaEdit::Substitution { from, to })
        },
    ))
    .context(EDIT)
    .parse_next(input)?;
    Ok(edit)
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.anchor {
            PositionAnchor::Transcript(
                Anchor::Coding { position } | Anchor::Noncoding { position },
            )
            | PositionAnchor::Absolute(position)
            | PositionAnchor::Rna(position) => write!(f, "{}", position.get()),
            PositionAnchor::Transcript(Anchor::Upstream { distance })
            | PositionAnchor::Upstream(distance)
            | PositionAnchor::RnaUpstream(distance) => write!(f, "-{}", distance.get()),
            PositionAnchor::Transcript(Anchor::Downstream { distance })
            | PositionAnchor::Downstream(distance)
            | PositionAnchor::RnaDownstream(distance) => write!(f, "*{}", distance.get()),
            PositionAnchor::Unknown => f.write_str("?"),
        }?;
        if let Some(offset) = self.offset {
            write!(f, "{offset:+}")?;
        }
        Ok(())
    }
}
impl fmt::Display for Bound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(p) => write!(f, "{p}"),
            Self::Between(a, b) => write!(f, "({a}_{b})"),
        }
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Point(p) => write!(f, "{p}"),
            Self::Range(a, b) => write!(f, "{a}_{b}"),
            Self::Uncertain { start, end } if start == end => write!(f, "{start}"),
            Self::Uncertain { start, end } => write!(f, "{start}_{end}"),
        }
    }
}

impl NaChange {
    pub(super) fn render(
        &self,
        f: &mut fmt::Formatter<'_>,
        enclosing: (&str, &str),
    ) -> fmt::Result {
        write!(f, "{}", self.molecule)?;
        f.write_str(enclosing.0)?;
        match &self.body {
            NaBody::Whole(whole) => write!(f, "{whole}")?,
            NaBody::Edit { location, edit } => {
                write!(f, "{location}")?;
                match edit {
                    NaEdit::Substitution { from, to } => {
                        self.render_base(f, *from)?;
                        f.write_str(">")?;
                        self.render_base(f, *to)?;
                    }
                    NaEdit::Deletion(bases) | NaEdit::Duplication(bases) => {
                        f.write_str(if matches!(edit, NaEdit::Deletion(_)) {
                            "del"
                        } else {
                            "dup"
                        })?;
                        if let Some(bases) = bases {
                            self.render_bases(f, bases)?;
                        }
                    }
                    NaEdit::Insertion(bases) => {
                        f.write_str("ins")?;
                        self.render_bases(f, bases)?;
                    }
                    NaEdit::Delins { deleted, inserted } => {
                        f.write_str("del")?;
                        if let Some(deleted) = deleted {
                            self.render_bases(f, deleted)?;
                        }
                        f.write_str("ins")?;
                        self.render_bases(f, inserted)?;
                    }
                    NaEdit::Inversion => f.write_str("inv")?,
                    NaEdit::Same => f.write_str("=")?,
                    NaEdit::Unknown => f.write_str("?")?,
                }
            }
        }
        f.write_str(enclosing.1)?;
        Ok(())
    }
    fn render_base(&self, f: &mut fmt::Formatter<'_>, base: Nucleotide) -> fmt::Result {
        let letter = base.to_string().chars().next().ok_or(fmt::Error)?;
        write!(
            f,
            "{}",
            if self.molecule == Molecule::Rna {
                letter.to_ascii_lowercase()
            } else {
                letter
            }
        )
    }
    fn render_bases(&self, f: &mut fmt::Formatter<'_>, bases: &Nucleotides) -> fmt::Result {
        for base in bases.as_slice() {
            self.render_base(f, *base)?;
        }
        Ok(())
    }
}
impl fmt::Display for NaChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f, ("", ""))
    }
}
