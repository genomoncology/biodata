//! Cytogenetic catalog forms reuse ISCN chromosomes, bands and events.
use super::{AlterationCode, Kind};
use crate::Code;
use crate::iscn::{self, Arm, Chromosome};
use crate::parse::{Expect, Input, Result, decimal, refuse, step};
use std::fmt;
use winnow::Parser;
use winnow::ascii::Caseless;
use winnow::combinator::{alt, cut_err, delimited, opt, peek, preceded};
use winnow::token::{one_of, take_until};

/// A cytogenetic alteration without interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Cytogenetic {
    /// A stated ploidy class.
    Ploidy(Ploidy),
    /// A stated whole-chromosome copy class.
    Somy {
        /// The copy class.
        somy: Somy,
        /// The chromosome.
        chromosome: Chromosome,
    },
    /// A stated karyotype class.
    Karyotype(KaryotypeClass),
    /// An inclusive aberration count.
    Count {
        /// The count's subject.
        kind: CountKind,
        /// The lower bound.
        low: u16,
        /// The upper bound, equal to low for an exact count.
        high: u16,
    },
    /// A standalone ISCN event.
    Event(iscn::Event),
    /// An arm short form such as del(5q).
    ArmDeletion {
        /// The chromosome.
        chromosome: Chromosome,
        /// The arm.
        arm: Arm,
    },
    /// A stated arm gain or loss.
    ArmCopy {
        /// The copy change.
        change: CopyChange,
        /// The chromosome.
        chromosome: Chromosome,
        /// The arm.
        arm: Arm,
    },
    /// A stated segment gain or loss.
    Segment {
        /// The copy change.
        change: CopyChange,
        /// The chromosome and breakpoints, shared with ISCN.
        part: iscn::Part,
    },
}

// The text table is shared by parsing and canonical rendering.
use crate::table::text_table;
text_table!(Ploidy, "A stated ploidy class.", Caseless, {
    NearDiploidy => "Near diploidy", NearTriploidy => "Near triploidy",
    NearTetraploidy => "Near tetraploidy", Hypodiploidy => "Hypodiploidy",
    Hyperdiploidy => "Hyperdiploidy", Diploidy => "Diploidy",
    Triploidy => "Triploidy", Tetraploidy => "Tetraploidy",
});
text_table!(Somy, "A stated chromosome copy class.", Caseless, {
    Nullisomy => "Nullisomy", Monosomy => "Monosomy", Trisomy => "Trisomy",
    Tetrasomy => "Tetrasomy", Pentasomy => "Pentasomy", Polysomy => "Polysomy",
});
text_table!(KaryotypeClass, "A stated karyotype class.", Caseless, {
    VeryComplex => "Very Complex karyotype", Complex => "Complex karyotype",
    Monosomal => "Monosomal karyotype", Normal => "Normal karyotype",
});
text_table!(CountKind, "The subject of an aberration count.", Caseless, {
    Chromosome => "ChrCount", Numerical => "NumVarCount",
    Structural => "StrucVarCount", Total => "TotalVarCount",
});
text_table!(CopyChange, "A stated copy change.", Caseless, { Gain => "Gain", Loss => "Loss"});

const CYTO: Expect = Expect {
    code: Code::Alteration(AlterationCode::CytogeneticInvalid),
    label: "cytogenetic alteration",
};

pub(super) fn kind(input: &mut Input<'_>) -> Result<Kind> {
    step(input)?;
    alt((
        Ploidy::parse.map(Cytogenetic::Ploidy),
        KaryotypeClass::parse.map(Cytogenetic::Karyotype),
        somy,
        count,
        segment,
        arm_copy,
        arm_event,
        standalone,
    ))
    .map(Kind::Cytogenetic)
    .parse_next(input)
}

fn somy(input: &mut Input<'_>) -> Result<Cytogenetic> {
    step(input)?;
    if opt(Caseless("Loss of Y")).parse_next(input)?.is_some() {
        return Ok(Cytogenetic::Somy {
            somy: Somy::Nullisomy,
            chromosome: Chromosome::Y,
        });
    }
    opt(refuse::<_, ()>(
        Code::Alteration(AlterationCode::CytogeneticInvalid),
        Caseless("Loss of "),
    ))
    .parse_next(input)?;
    let somy = Somy::parse(input)?;
    cut_err(preceded(' ', iscn::chromosome))
        .context(CYTO)
        .map(|chromosome| Cytogenetic::Somy { somy, chromosome })
        .parse_next(input)
}

fn count(input: &mut Input<'_>) -> Result<Cytogenetic> {
    step(input)?;
    let kind = CountKind::parse(input)?;
    cut_err(|input: &mut Input<'_>| {
        ' '.parse_next(input)?;
        let low: u16 = decimal(input)?;
        let high = opt(preceded(
            '-',
            cut_err(decimal.verify(move |n: &u16| *n > low)),
        ))
        .parse_next(input)?
        .unwrap_or(low);
        super::end_change.parse_next(input)?;
        Ok(Cytogenetic::Count { kind, low, high })
    })
    .context(CYTO)
    .parse_next(input)
}

fn arm_copy(input: &mut Input<'_>) -> Result<Cytogenetic> {
    step(input)?;
    peek((
        alt((winnow::ascii::digit1, alt(("X", "Y", "?")))),
        one_of(['p', 'q']),
        ' ',
    ))
    .parse_next(input)?;
    let (chromosome, arm, _) = (iscn::chromosome, Arm::parse, ' ').parse_next(input)?;
    cut_err(CopyChange::parse)
        .context(CYTO)
        .map(|change| Cytogenetic::ArmCopy {
            change,
            chromosome: chromosome.clone(),
            arm,
        })
        .parse_next(input)
}

fn arm_event(input: &mut Input<'_>) -> Result<Cytogenetic> {
    step(input)?;
    let (chromosome, arm) =
        preceded("del", delimited('(', (iscn::chromosome, Arm::parse), ')')).parse_next(input)?;
    Ok(Cytogenetic::ArmDeletion { chromosome, arm })
}

fn segment(input: &mut Input<'_>) -> Result<Cytogenetic> {
    step(input)?;
    let change = alt((
        Caseless("StrucGain").value(CopyChange::Gain),
        Caseless("StrucLoss").value(CopyChange::Loss),
    ))
    .parse_next(input)?;
    cut_err(|input: &mut Input<'_>| {
        let (mut parts, _) = iscn::parts(input, iscn::Parts::Segment)?;
        let part = parts.remove(0);
        Ok(Cytogenetic::Segment { change, part })
    })
    .context(CYTO)
    .parse_next(input)
}

fn standalone(input: &mut Input<'_>) -> Result<Cytogenetic> {
    step(input)?;
    // Numeric prefixes with a comma name a whole karyotype, outside alteration shorthand.
    opt(refuse::<_, ()>(
        Code::Alteration(AlterationCode::KaryotypeUnsupported),
        (winnow::ascii::digit1, ','),
    ))
    .parse_next(input)?;
    opt(refuse::<_, ()>(
        Code::Alteration(AlterationCode::ChromosomeUnsupported),
        (
            one_of(['+', '-', '−']),
            peek((iscn::chromosome, super::end_change)),
        ),
    ))
    .parse_next(input)?;
    iscn::event_start(input)?;
    // The wildcard refusal must not conceal a malformed band in the chromosome list.
    opt(refuse::<_, ()>(
        Code::Alteration(AlterationCode::CytogeneticInvalid),
        take_until(1.., ")").verify(|list: &str| {
            list.split_once('(')
                .is_some_and(|(_, chromosomes)| chromosomes.contains(['p', 'q']))
        }),
    ))
    .parse_next(input)?;
    cut_err(iscn::event)
        .context(CYTO)
        .map(Cytogenetic::Event)
        .parse_next(input)
}

impl fmt::Display for Cytogenetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ploidy(value) => write!(f, "{value}"),
            Self::Karyotype(value) => write!(f, "{value}"),
            Self::Somy {
                somy: Somy::Nullisomy,
                chromosome: Chromosome::Y,
            } => f.write_str("Loss of Y"),
            Self::Somy { somy, chromosome } => write!(f, "{somy} {chromosome}"),
            Self::Count { kind, low, high } => {
                write!(f, "{kind} {low}")?;
                if high != low {
                    write!(f, "-{high}")?;
                }
                Ok(())
            }
            Self::Event(event) => write!(f, "{event}"),
            Self::ArmDeletion { chromosome, arm } => write!(f, "del({chromosome}{arm})"),
            Self::ArmCopy {
                change,
                chromosome,
                arm,
            } => write!(f, "{chromosome}{arm} {change}"),
            Self::Segment { change, part } => {
                write!(f, "Struc{change}")?;
                iscn::render_parts(f, std::slice::from_ref(part))
            }
        }
    }
}
