//! Winnow grammar for conventional karyotypes.
use super::{
    Alternative, Arm, Band, Breakpoint, Cells, Chromosome, Clone, CloneBase, Count, Event,
    IscnCode, Item, Karyotype, Mixture, RearrangementKind, SexChromosome, Sign, Site, SiteKind,
    observation, structure,
};
use crate::Code;
use crate::parse::{Expect, Input, Result, decimal, refuse, reject, step};
use structure::{derivative, marker, neo, rearrangement};
use winnow::Parser;
use winnow::ascii::digit1;
use winnow::combinator::{alt, cut_err, delimited, not, opt, peek, preceded, repeat};
use winnow::token::{one_of, take_while};

fn expect(code: IscnCode, label: &'static str) -> Expect {
    Expect {
        code: Code::Iscn(code),
        label,
    }
}

pub(super) fn iscn(input: &mut Input<'_>) -> Result<Karyotype> {
    step(input)?;
    alt((
        observation::unsupported,
        refuse(Code::Iscn(IscnCode::MixtureUnsupported), "end "),
        refuse(
            Code::Iscn(IscnCode::EventUnsupported),
            alt(((RearrangementKind::parse, '(').void(), unsupported_event)),
        ),
        karyotype,
    ))
    .context(expect(IscnCode::KaryotypeInvalid, "karyotype"))
    .parse_next(input)
}

fn karyotype(input: &mut Input<'_>) -> Result<Karyotype> {
    step(input)?;
    let mixture = opt(alt((
        "mos ".value(Mixture::Mosaic),
        "chi ".value(Mixture::Chimera),
    )))
    .parse_next(input)?;
    let explicit_chimera = matches!(mixture, Some(Mixture::Chimera));
    let leading = !explicit_chimera && opt("//").parse_next(input)?.is_some();
    let mut clones = vec![clone.parse_next(input)?];
    let mut donor_start = leading.then_some(0);
    loop {
        if opt("//").parse_next(input)?.is_some() {
            if !explicit_chimera {
                if donor_start.is_some() {
                    reject::<()>(input, None)?;
                }
                donor_start = Some(
                    winnow::combinator::empty
                        .verify_map(|_| u8::try_from(clones.len()).ok())
                        .parse_next(input)?,
                );
            }
            if opt(peek(winnow::combinator::eof))
                .parse_next(input)?
                .is_some()
                && !explicit_chimera
            {
                break;
            }
            clones.push(cut_err(clone).parse_next(input)?);
        } else if !explicit_chimera && opt('/').parse_next(input)?.is_some() {
            clones.push(cut_err(clone).parse_next(input)?);
        } else {
            break;
        }
    }
    opt(preceded(peek('.'), observation::unsupported::<()>)).parse_next(input)?;
    Ok(Karyotype {
        mixture,
        donor_start,
        clones,
    })
}

fn clone(input: &mut Input<'_>) -> Result<Clone> {
    step(input)?;
    let count = count
        .verify(|value| !matches!(value, Count::About(_)))
        .context(expect(IscnCode::CountInvalid, "chromosome count"))
        .parse_next(input)?;
    let ploidy = opt(delimited(
        '<',
        cut_err(decimal.verify(|n: &u8| *n > 0)),
        cut_err("n>"),
    ))
    .parse_next(input)?;
    ','.parse_next(input)?;
    opt(refuse::<_, ()>(
        Code::Iscn(IscnCode::CompositeUnsupported),
        digit1,
    ))
    .parse_next(input)?;
    let (sex, base) = cut_err(alt((
        clone_base.map(|base| (Vec::new(), Some(base))),
        repeat(
            1..,
            alt(('X'.value(SexChromosome::X), 'Y'.value(SexChromosome::Y))),
        )
        .map(|sex| (sex, None)),
    )))
    .parse_next(input)?;
    let sex_constitutional = if base.is_none() {
        opt('c').parse_next(input)?.is_some()
    } else {
        false
    };
    let base_times = if base.is_some() {
        opt(preceded(
            alt(('x', '×')),
            cut_err(decimal.verify(|n: &u8| *n > 0)),
        ))
        .parse_next(input)?
    } else {
        None
    };
    let events = repeat(0.., preceded(',', cut_err(event))).parse_next(input)?;
    let cells = opt(cells).parse_next(input)?;
    if cells.is_some() {
        opt(refuse::<_, ()>(
            Code::Iscn(IscnCode::CompositeUnsupported),
            alt((',', '[')),
        ))
        .parse_next(input)?;
    }
    Ok(Clone {
        count,
        ploidy,
        sex,
        sex_constitutional,
        base,
        base_times,
        events,
        cells,
    })
}

fn count(input: &mut Input<'_>) -> Result<Count> {
    step(input)?;
    let about = opt('~').parse_next(input)?.is_some();
    let low: u8 = decimal.verify(|n: &u8| *n > 0).parse_next(input)?;
    let high = opt(preceded(
        alt(('~', '∼', '-')),
        cut_err(decimal.verify(move |n: &u8| *n > low)),
    ))
    .parse_next(input)?;
    if about && high.is_some() {
        winnow::combinator::fail::<_, (), _>.parse_next(input)?;
    }
    Ok(if about {
        Count::About(low)
    } else {
        high.map_or(Count::Exact(low), |high| Count::Range(low, high))
    })
}

fn clone_base(input: &mut Input<'_>) -> Result<CloneBase> {
    step(input)?;
    alt((
        "idem".value(CloneBase::Idem),
        preceded("sdl", opt(decimal.verify(|n: &u8| *n > 0))).map(CloneBase::Sdl),
        preceded("sl", opt(decimal.verify(|n: &u8| *n > 0))).map(CloneBase::Sl),
    ))
    .parse_next(input)
}

fn cells(input: &mut Input<'_>) -> Result<Cells> {
    step(input)?;
    preceded(
        '[',
        cut_err((opt("cp"), decimal.verify(|n: &u32| *n > 0), ']')),
    )
    .map(|(composite, count, _)| Cells {
        count,
        composite: composite.is_some(),
    })
    .context(expect(IscnCode::CellsInvalid, "cell count"))
    .parse_next(input)
}

/// Recognize an isolated event with the same markers used by the event grammar.
pub(crate) fn event_start(input: &mut Input<'_>) -> Result<()> {
    step(input)?;
    peek((
        opt('?'),
        opt(Sign::parse),
        opt('?'),
        alt((
            (opt(count), marker).void(),
            (
                opt("psu "),
                alt((
                    alt(("ider", "der", "neo")).void(),
                    (RearrangementKind::parse, opt(" r")).void(),
                )),
                '(',
            )
                .void(),
        )),
    ))
    .void()
    .parse_next(input)
}

pub(crate) fn event(input: &mut Input<'_>) -> Result<Event> {
    step(input)?;
    event_body
        .context(expect(IscnCode::EventInvalid, "event"))
        .parse_next(input)
}

fn event_body(input: &mut Input<'_>) -> Result<Event> {
    let uncertain = opt('?').parse_next(input)?.is_some();
    let sign = opt(Sign::parse).parse_next(input)?;
    let uncertain = opt((
        '?',
        peek(alt((chromosome.void(), event_start, unsupported_event))),
    ))
    .parse_next(input)?
    .is_some()
        || uncertain;
    let (item, prefix_count) = alt((
        (count, marker).map(|(count, item)| (item, Some(count))),
        (|input: &mut Input<'_>| item(input, sign.as_ref())).map(|item| (item, None)),
    ))
    .parse_next(input)?;
    if sign.is_none() && matches!(item, Item::Whole(_)) {
        reject::<()>(input, None)?;
    }
    let constitutional = opt('c').parse_next(input)?.is_some();
    let copies = opt(preceded(
        alt(('x', '×')),
        cut_err(decimal.verify_map(std::num::NonZeroU8::new)),
    ))
    .parse_next(input)?;
    Ok(Event {
        sign,
        uncertain,
        constitutional,
        prefix_count,
        copies,
        item,
    })
}

fn item(input: &mut Input<'_>, sign: Option<&Sign>) -> Result<Item> {
    step(input)?;
    if sign.is_some()
        && let Some(chromosome) = opt(chromosome).parse_next(input)?
    {
        return Ok(Item::Whole(chromosome));
    }
    let item = alt((
        observation::unsupported,
        detailed,
        derivative.map(Item::Derivative),
        neo.map(Item::Neo),
        marker,
        refuse(Code::Iscn(IscnCode::EventUnsupported), unsupported_event),
        rearrangement.map(Item::Rearrangement),
        chromosome.map(Item::Whole),
    ))
    .parse_next(input)?;
    Ok(item)
}

fn unsupported_event(input: &mut Input<'_>) -> Result<()> {
    step(input)?;
    preceded(
        take_while(0.., |c: char| c.is_ascii_digit() || "?~∼-".contains(c)),
        take_while(1.., |c: char| c.is_ascii_lowercase()).verify(|marker: &str| {
            [
                "inc", "ace", "chre", "chrb", "chrg", "cht", "chtdel", "pcc", "pcd", "sce", "rec",
                "tas", "tr", "cx", "fem", "qr", "dim", "min",
            ]
            .contains(&marker)
        }),
    )
    .void()
    .parse_next(input)
}

pub(super) fn detailed<T>(input: &mut Input<'_>) -> Result<T> {
    refuse(
        Code::Iscn(IscnCode::DetailedUnsupported),
        alt(("::", "→", "->")),
    )
    .parse_next(input)
}

pub(crate) fn chromosome(input: &mut Input<'_>) -> Result<Chromosome> {
    step(input)?;
    alt((
        'X'.value(Chromosome::X),
        'Y'.value(Chromosome::Y),
        '?'.value(Chromosome::Unknown),
        refuse(Code::Iscn(IscnCode::ChromosomeUnsupported), 'v'),
        decimal
            .verify(|n: &u8| (1..=22).contains(n))
            .map(Chromosome::Autosome),
    ))
    .context(expect(
        IscnCode::ChromosomeInvalid,
        "chromosome 1 through 22, X, Y or ?",
    ))
    .parse_next(input)
}

pub(crate) fn breakpoint_group(input: &mut Input<'_>) -> Result<Vec<Breakpoint>> {
    step(input)?;
    repeat(1.., breakpoint)
        .context(expect(IscnCode::BreakpointInvalid, "breakpoint"))
        .parse_next(input)
}

fn breakpoint(input: &mut Input<'_>) -> Result<Breakpoint> {
    step(input)?;
    alt((detailed, breakpoint_body))
        .context(expect(
            IscnCode::BreakpointInvalid,
            "band, terminal or centromere",
        ))
        .parse_next(input)
}

fn breakpoint_body(input: &mut Input<'_>) -> Result<Breakpoint> {
    let point = site(input)?;
    let arm = match &point.location {
        SiteKind::Band(arm, _) | SiteKind::Arm(arm) | SiteKind::Terminal(arm) => Some(*arm),
        _ => None,
    };
    let alternative = opt(alt((
        preceded(" or ", cut_err(site)).map(Alternative::Or),
        preceded(
            alt(('~', '∼')),
            cut_err(|input: &mut Input<'_>| {
                (|input: &mut Input<'_>| site_with_arm(input, arm))
                    .verify(|end: &Site| point.location != end.location)
                    .parse_next(input)
            }),
        )
        .map(Alternative::Range),
    )))
    .parse_next(input)?;
    Ok(Breakpoint {
        site: point,
        alternative,
    })
}

pub(super) fn site(input: &mut Input<'_>) -> Result<Site> {
    site_with_arm(input, None)
}

fn site_with_arm(input: &mut Input<'_>, inherited: Option<Arm>) -> Result<Site> {
    step(input)?;
    let prefix = opt('?').parse_next(input)?.is_some();
    let mut location = alt((
        "cen".value((SiteKind::Centromere, false)),
        |input: &mut Input<'_>| {
            let arm = Arm::parse(input)?;
            arm_site(input, arm)
        },
        |input: &mut Input<'_>| {
            let Some(arm) = inherited else {
                return winnow::combinator::fail.parse_next(input);
            };
            band_site(input, arm)
        },
    ));
    let location = if prefix {
        opt(location).parse_next(input)?
    } else {
        Some(location.parse_next(input)?)
    };
    let Some((location, inside)) = location else {
        return winnow::combinator::empty
            .verify(|_| prefix)
            .value(Site {
                location: SiteKind::Unknown,
                uncertain: false,
            })
            .parse_next(input);
    };
    let suffix = opt('?').parse_next(input)?.is_some();
    Ok(Site {
        location,
        uncertain: prefix || inside || suffix,
    })
}

fn arm_site(input: &mut Input<'_>, arm: Arm) -> Result<(SiteKind, bool)> {
    alt((
        |input: &mut Input<'_>| band_site(input, arm),
        "ter".value((SiteKind::Terminal(arm), false)),
        preceded('?', |input: &mut Input<'_>| band_site(input, arm)).map(|(site, _)| (site, true)),
        '?'.value((SiteKind::Arm(arm), false)),
    ))
    .parse_next(input)
}

fn band_site(input: &mut Input<'_>, arm: Arm) -> Result<(SiteKind, bool)> {
    alt((
        (one_of('0'..='9'), '?', one_of('0'..='9'), subband)
            .verify_map(|(region, _, digit, sub)| {
                Band::new(region as u8 - b'0', Some(digit as u8 - b'0'), sub)
            })
            .map(|band| (SiteKind::Band(arm, band), true)),
        band.map(|band| (SiteKind::Band(arm, band), false)),
    ))
    .parse_next(input)
}

fn band(input: &mut Input<'_>) -> Result<Band> {
    step(input)?;
    (
        one_of('0'..='9'),
        one_of(|c: char| c.is_ascii_digit() || c == '?'),
        not(one_of('0'..='9')),
        subband,
    )
        .verify_map(|(region, digit, _, sub)| {
            Band::new(
                region as u8 - b'0',
                (digit != '?').then_some(digit as u8 - b'0'),
                sub,
            )
        })
        .parse_next(input)
}

fn subband(input: &mut Input<'_>) -> Result<Option<u8>> {
    opt(preceded(
        '.',
        cut_err(digit1.verify_map(|s: &str| {
            if s.starts_with('0') || s.len() > 2 {
                None
            } else {
                s.parse().ok()
            }
        })),
    ))
    .parse_next(input)
}
