//! Catalog alteration shorthand, without annotation or gene resolution.
mod coding;
mod cytogenetic;
mod deferred;
mod event;
pub use event::{Category, Effect, GeneEvent, Scope};
mod marker;
mod pharmacogenomic;

pub use cytogenetic::{CopyChange, CountKind, Cytogenetic, KaryotypeClass, Ploidy, Somy};
pub use marker::{
    Comparison, Decimal, ExpressionState, Finding, Level, Mmr, Msi, Score, ScoreBound,
    ScoringSystem, Signature,
};
pub use pharmacogenomic::{HlaSuffix, Star, StarSuffix};

use crate::GeneSymbol;
use crate::hgvs::{NaChange, ProteinChange};
use crate::parse::{Expect, Input, Result, refuse, run, step};
use crate::{Code, Parsed};
use std::fmt;
use winnow::Parser;
use winnow::ascii::Caseless;
use winnow::combinator::{alt, cut_err, opt};

/// One alteration and its stated origin.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Alteration {
    /// The subject and change.
    pub kind: Kind,
    /// The stated origin, when given.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub origin: Option<Origin>,
}
crate::table::text_table!(Origin, "The stated source of an alteration.", Caseless, {
    Germline => "germline",
    Somatic => "somatic",
}, "snake_case");
/// What an alteration describes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    /// A stated chromosome finding.
    Cytogenetic(Cytogenetic),
    /// A whole-sample signature.
    Signature(Signature),
    /// A whole gene subject and its stated change.
    Gene {
        /// The subject, without alias resolution or splitting.
        subject: GeneSymbol,
        /// The stated change.
        change: Change,
    },
}
/// A change stated for a gene.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Change {
    /// A stated coding or noncoding nucleotide change.
    Coding(NaChange),
    /// A named splice variant number.
    SpliceVariant(u8),
    /// A stated promoter methylation finding.
    PromoterMethylation(Finding),
    /// A stated expression result and optional exact score.
    Expression {
        /// The expression result.
        state: ExpressionState,
        /// An optional score, without derived thresholds.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        score: Option<Score>,
    },
    /// A stated category with an optional scope and effect.
    Event(GeneEvent),
    /// A protein change, optionally paired with a coding change.
    Protein {
        /// The protein change shared with HGVS.
        protein: ProteinChange,
        /// A stated coding change, added by the coding grammar.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        coding: Option<NaChange>,
    },
    /// One star allele or a two-allele genotype.
    StarAlleles {
        /// The first or single allele.
        first: Star,
        /// The second allele, when a genotype is stated.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        second: Option<Star>,
    },
    /// An HLA allele with up to four ordered fields.
    Hla {
        /// Numeric fields in written order; unused trailing slots are absent.
        fields: [Option<u16>; 4],
        /// The stated expression or protein group suffix.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        suffix: Option<HlaSuffix>,
    },
}
crate::table::codes! {
/// Stable refusals from alteration shorthand.
pub enum AlterationCode {
    /// A malformed cytogenetic description.
    CytogeneticInvalid => "alteration.cytogenetic.invalid",
    /// A whole karyotype belongs to ISCN.
    KaryotypeUnsupported => "alteration.karyotype.unsupported",
    /// A bare signed chromosome has no catalog alteration form.
    ChromosomeUnsupported => "alteration.chromosome.unsupported",
    /// A recognized signature with an unsupported label.
    SignatureUnsupported => "alteration.signature.unsupported",
    /// A malformed score or range.
    ScoreInvalid => "alteration.score.invalid",
    /// A malformed marker result.
    MarkerInvalid => "alteration.marker.invalid",
    /// An unknown scoring system.
    ScoringSystemUnsupported => "alteration.scoring_system.unsupported",
    /// A splice variant without an Arabic variant number.
    SpliceUnsupported => "alteration.splice.unsupported",
    /// A malformed event or incompatible scope and category.
    EventInvalid => "alteration.event.invalid",
    /// A gene inversion category without a consumer.
    InversionUnsupported => "alteration.inversion.unsupported",
    /// A wild-type finding without a consumer.
    WildTypeUnsupported => "alteration.wild_type.unsupported",
    /// An unknown status finding without a consumer.
    StatusUnknownUnsupported => "alteration.status_unknown.unsupported",
    /// An immunoglobulin segment without a consumer.
    SegmentUnsupported => "alteration.segment.unsupported",
    /// A recognized subject family without a typed grammar.
    SubjectUnsupported => "alteration.subject.unsupported",
    /// A recognized event description without a typed grammar.
    EventUnsupported => "alteration.event.unsupported",
    /// A free-text regional description without a typed grammar.
    DescriptionUnsupported => "alteration.description.unsupported",
    /// A named genotype outside the star and HLA grammars.
    GenotypeUnsupported => "alteration.genotype.unsupported",
    /// A named scored instrument.
    InstrumentUnsupported => "alteration.instrument.unsupported",
    /// A stated risk classification.
    RiskClassUnsupported => "alteration.risk_class.unsupported",
    /// A stated subtype.
    SubtypeUnsupported => "alteration.subtype.unsupported",
    /// The subject or overall shape is malformed.
    ShapeInvalid => "alteration.shape.invalid",
    /// A malformed star allele or genotype.
    StarInvalid => "alteration.star.invalid",
    /// A malformed HLA allele.
    HlaInvalid => "alteration.hla.invalid",
    /// An HLA nucleotide group suffix.
    HlaGroupUnsupported => "alteration.hla_group.unsupported",
    /// A malformed origin suffix.
    OriginInvalid => "alteration.origin.invalid",
    /// A malformed protein change.
    ProteinInvalid => "alteration.protein.invalid",
    /// A site with no stated change.
    CodonUnsupported => "alteration.codon.unsupported",
    /// A noncanonical range separator.
    RangeUnsupported => "alteration.range.unsupported",
    /// A change wrapped in parentheses.
    MultiplicityUnsupported => "alteration.multiplicity.unsupported",
    /// A formal accession-qualified description.
    HgvsUnsupported => "alteration.hgvs.unsupported",
}
}

const SHAPE: Expect = Expect {
    code: Code::Alteration(AlterationCode::ShapeInvalid),
    label: "gene and change",
};
/// Parses one alteration description. Edge spaces are ignored.
pub fn parse(text: &str) -> Parsed<Alteration> {
    run(text, alteration)
}
fn alteration(input: &mut Input<'_>) -> Result<Alteration> {
    let mut deferred = *input;
    match alteration_value(input)
        .and_then(|value| winnow::combinator::eof.value(value).parse_next(input))
    {
        Ok(value) => Ok(value),
        Err(error) => {
            deferred.state = input.state;
            winnow::ascii::space0.parse_next(&mut deferred)?;
            deferred::unsupported(&mut deferred)?;
            Err(error)
        }
    }
}
fn alteration_value(input: &mut Input<'_>) -> Result<Alteration> {
    step(input)?;
    winnow::ascii::space0.parse_next(input)?;
    if let Some(kind) = opt(cytogenetic::kind).parse_next(input)? {
        let origin = opt(origin).parse_next(input)?;
        winnow::ascii::space0.parse_next(input)?;
        return Ok(Alteration { kind, origin });
    }
    // Formal HGVS is a distinct notation, even when it begins with a gene-like prefix.
    let hla_subject = input
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("HLA-"));
    if !hla_subject && input.contains(':') && !input.trim_end().contains(' ') {
        return refuse(
            Code::Alteration(AlterationCode::HgvsUnsupported),
            winnow::token::take_till(1.., ':'),
        )
        .parse_next(input);
    }
    if let Some(kind) = opt(marker::signature).parse_next(input)? {
        let origin = opt(origin).parse_next(input)?;
        winnow::ascii::space0.parse_next(input)?;
        return Ok(Alteration { kind, origin });
    }
    let (subject, change) = alt((event::hyphenated, gene_change)).parse_next(input)?;
    let origin = opt(origin).parse_next(input)?;
    winnow::ascii::space0.parse_next(input)?;
    Ok(Alteration {
        kind: Kind::Gene { subject, change },
        origin,
    })
}
fn gene_change(input: &mut Input<'_>) -> Result<(GeneSymbol, Change)> {
    let hla_subject = input
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("HLA-"));
    let subject =
        cut_err(|input: &mut Input<'_>| crate::value::gene_symbol_prefix(input, input.len()))
            .context(SHAPE)
            .parse_next(input)?;
    let change = if hla_subject && input.starts_with('*') {
        pharmacogenomic::hla(input)?
    } else {
        cut_err(' ').context(SHAPE).parse_next(input)?;
        gene_change_value(input)?
    };
    Ok((subject, change))
}
fn end_change(input: &mut Input<'_>) -> Result<()> {
    winnow::combinator::peek(alt((
        winnow::combinator::eof.void(),
        " (".void(),
        (winnow::ascii::space1, winnow::combinator::eof).void(),
    )))
    .parse_next(input)
}
fn gene_change_value(input: &mut Input<'_>) -> Result<Change> {
    if input.starts_with('*')
        && (input.contains('/')
            || input
                .split(' ')
                .nth(1)
                .is_some_and(|word| word.eq_ignore_ascii_case("allele"))
            || input
                .trim_start_matches('*')
                .split_ascii_whitespace()
                .next()
                .is_some_and(|number| number.bytes().all(|c| c.is_ascii_digit())))
    {
        pharmacogenomic::star_change(input)
    } else {
        alt((marker::change, event::change, coding::change)).parse_next(input)
    }
}
fn origin(input: &mut Input<'_>) -> Result<Origin> {
    " (".parse_next(input)?;
    cut_err((Origin::parse, ')'))
        .context(Expect {
            code: Code::Alteration(AlterationCode::OriginInvalid),
            label: "germline or somatic origin",
        })
        .map(|(origin, _)| origin)
        .parse_next(input)
}
impl fmt::Display for Alteration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Kind::Cytogenetic(value) => write!(f, "{value}")?,
            Kind::Gene { subject, change } => {
                let separator = if matches!(change, Change::Hla { .. }) {
                    ""
                } else {
                    " "
                };
                write!(f, "{subject}{separator}{change}")?;
            }
            Kind::Signature(signature) => write!(f, "{signature}")?,
        }
        if let Some(origin) = self.origin {
            write!(f, " ({origin})")?;
        }
        Ok(())
    }
}
impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Event(event) => write!(f, "{event}"),
            Self::Coding(coding) => write!(f, "{coding}"),
            Self::Protein { protein, coding } => {
                write!(f, "{protein:#}")?;
                if let Some(coding) = coding {
                    write!(f, " {coding}")?;
                }
                Ok(())
            }
            Self::StarAlleles { first, second } => {
                write!(f, "{first}")?;
                match second {
                    Some(second) => write!(f, "/{second}"),
                    None => f.write_str(" allele"),
                }
            }
            Self::Hla { fields, suffix } => {
                f.write_str("*")?;
                for (index, field) in fields.iter().flatten().enumerate() {
                    if index > 0 {
                        f.write_str(":")?;
                    }
                    write!(f, "{field:02}")?;
                }
                if let Some(suffix) = suffix {
                    write!(f, "{suffix}")?;
                }
                Ok(())
            }
            Self::SpliceVariant(number) => write!(f, "V{number} Splice Variant"),
            Self::PromoterMethylation(finding) => write!(f, "Promoter Methylation {finding}"),
            Self::Expression { state, score } => {
                write!(f, "{state}")?;
                if let Some(score) = score {
                    write!(f, " ({score})")?;
                }
                Ok(())
            }
        }
    }
}
