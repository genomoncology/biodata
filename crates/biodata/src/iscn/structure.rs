//! Shared structural grammar for events and derivative changes.
use super::grammar::{breakpoint_group, chromosome, detailed, site};
use super::{Breakpoint, Derivative, IscnCode, Item, Neo, Part, Rearrangement, RearrangementKind};
use crate::Code;
use crate::parse::{Input, Result, refuse, step};
use winnow::Parser;
use winnow::combinator::{alt, cut_err, delimited, not, opt, peek, repeat, separated};
use winnow::token::one_of;

pub(super) fn rearrangement(input: &mut Input<'_>) -> Result<Rearrangement> {
    step(input)?;
    let uncertain = opt('?').parse_next(input)?.is_some();
    let pseudo = opt("psu ").parse_next(input)?.is_some();
    let kind = RearrangementKind::parse(input)?;
    let ring = if matches!(
        kind,
        RearrangementKind::Dicentric | RearrangementKind::Tricentric
    ) {
        opt(" r").parse_next(input)?.is_some()
    } else {
        false
    };
    cut_err(move |input: &mut Input<'_>| {
        let (parts, same_length) = parts(input, Parts::Rearrangement(&kind))?;
        opt(detailed::<()>).parse_next(input)?;
        let valid = same_length
            && valid_shape(&kind, &parts, ring)
            && (!pseudo
                || matches!(
                    kind,
                    RearrangementKind::Dicentric | RearrangementKind::Isodicentric
                ));
        // Validation stays on the parser, so a malformed shape has the construct's code.
        winnow::combinator::empty
            .value(Rearrangement {
                kind,
                pseudo,
                ring,
                uncertain,
                parts,
            })
            .verify(|_| valid)
            .parse_next(input)
    })
    .parse_next(input)
}

fn valid_shape(kind: &RearrangementKind, parts: &[Part], ring: bool) -> bool {
    use RearrangementKind::*;
    let widths: Vec<_> = parts.iter().map(|p| p.breakpoints.len()).collect();
    match kind {
        Translocation => parts.len() >= 2 && widths.iter().all(|n| *n >= 1),
        Deletion => parts.len() == 1 && widths.iter().all(|n| (1..=2).contains(n)),
        Duplication | Inversion | Triplication | Quadruplication => {
            parts.len() == 1 && widths == [2]
        }
        Pulverization => parts.len() == 1 && widths == [0],
        Dicentric if ring => parts.len() == 2 && widths == [2, 2],
        Tricentric if ring => parts.len() == 3 && widths == [2, 2, 2],
        Tricentric => parts.len() == 3 && widths.iter().all(|n| *n >= 1),
        Dicentric | Robertsonian | ChromatidExchange => parts.len() == 2 && widths == [1, 1],
        Ring => !parts.is_empty() && widths.iter().all(|n| *n == 0 || *n == 2),
        Insertion => widths == [3] || widths == [1, 2],
        Isochromosome
        | AdditionalMaterial
        | Isodicentric
        | HomogeneouslyStainingRegion
        | FragileSite
        | Fission
        | ChromatidBreak
        | ChromatidGap => parts.len() == 1 && widths == [1],
    }
}

pub(super) fn marker(input: &mut Input<'_>) -> Result<Item> {
    step(input)?;
    alt((
        "mar".value(Item::Marker),
        "dmin".value(Item::Dmin),
        (
            'r',
            not(peek(one_of(|c: char| c.is_ascii_alphabetic() || c == '('))),
        )
            .value(Item::Rearrangement(Rearrangement {
                kind: RearrangementKind::Ring,
                pseudo: false,
                ring: false,
                uncertain: false,
                parts: Vec::new(),
            })),
    ))
    .parse_next(input)
}

pub(super) fn derivative(input: &mut Input<'_>) -> Result<Derivative> {
    step(input)?;
    let isoderivative = alt(("ider".value(true), "der".value(false))).parse_next(input)?;
    cut_err(move |input: &mut Input<'_>| {
        let (parts, valid) = parts(input, Parts::Derivative)?;
        let changes = repeat(0.., rearrangement).parse_next(input)?;
        winnow::combinator::empty
            .value(Derivative {
                isoderivative,
                parts,
                changes,
            })
            .verify(|_| valid)
            .parse_next(input)
    })
    .parse_next(input)
}

pub(super) fn neo(input: &mut Input<'_>) -> Result<Neo> {
    step(input)?;
    "neo".parse_next(input)?;
    cut_err(|input: &mut Input<'_>| {
        let chromosome = delimited('(', chromosome, ')').parse_next(input)?;
        let (start, _, end) =
            delimited('(', (site, alt(("->", "→")), site), ":)").parse_next(input)?;
        Ok(Neo {
            chromosome,
            start,
            end,
        })
    })
    .parse_next(input)
}

pub(crate) enum Parts<'a> {
    Rearrangement(&'a RearrangementKind),
    Derivative,
    Segment,
}

pub(crate) fn parts(input: &mut Input<'_>, form: Parts<'_>) -> Result<(Vec<Part>, bool)> {
    let kind = match form {
        Parts::Rearrangement(kind) => Some(kind),
        _ => None,
    };
    let chromosomes: Vec<_> = delimited(
        '(',
        |input: &mut Input<'_>| match form {
            Parts::Segment => chromosome
                .map(|chromosome| vec![chromosome])
                .parse_next(input),
            _ => separated(1.., chromosome, ';').parse_next(input),
        },
        alt((
            refuse(
                Code::Iscn(IscnCode::ShortFormUnsupported),
                one_of(['p', 'q']).verify(|_| kind.is_some()),
            ),
            refuse(
                Code::Iscn(IscnCode::ShortFormUnsupported),
                (')', not(peek('('))).verify(|_| {
                    kind.is_some_and(|kind| {
                        !matches!(
                            kind,
                            RearrangementKind::Pulverization | RearrangementKind::Ring
                        )
                    })
                }),
            ),
            ')',
        )),
    )
    .parse_next(input)?;
    let groups: Option<Vec<Vec<Breakpoint>>> = match form {
        Parts::Segment => Some(
            delimited(
                '(',
                breakpoint_group
                    .verify(|points: &Vec<_>| points.len() <= 2)
                    .map(|points| vec![points]),
                ')',
            )
            .parse_next(input)?,
        ),
        Parts::Derivative => opt(delimited(
            '(',
            separated(1.., breakpoint_group, ';'),
            cut_err(')'),
        ))
        .parse_next(input)?,
        Parts::Rearrangement(kind)
            if matches!(kind, RearrangementKind::Pulverization)
                || (matches!(kind, RearrangementKind::Ring)
                    && opt(peek('(')).parse_next(input)?.is_none()) =>
        {
            None
        }
        Parts::Rearrangement(_) => Some(
            delimited(
                '(',
                cut_err(separated(1.., breakpoint_group, ';')),
                cut_err(')'),
            )
            .parse_next(input)?,
        ),
    };
    let valid = groups
        .as_ref()
        .is_none_or(|groups| groups.len() == chromosomes.len());
    let groups = groups.unwrap_or_else(|| vec![Vec::new(); chromosomes.len()]);
    Ok((
        chromosomes
            .into_iter()
            .zip(groups)
            .map(|(chromosome, breakpoints)| Part {
                chromosome,
                breakpoints,
            })
            .collect(),
        valid,
    ))
}
