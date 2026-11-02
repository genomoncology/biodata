//! Expression, exact scores, sample signatures and named splice variants.
use super::{AlterationCode, Change, Kind};
use crate::Code;
use crate::parse::{Expect, Input, Result, decimal, refuse, reject, step};
use std::fmt;
use winnow::Parser;
use winnow::ascii::Caseless;
use winnow::combinator::{alt, cut_err, fail, not, opt, preceded};
use winnow::token::take_while;

use crate::table::text_table;
text_table!(Finding, "A stated positive or negative finding.", Caseless, {Positive => "Positive", Negative => "Negative"});
text_table!(ExpressionState, "The stated expression result.", Caseless, {
    High => "High Expression", Low => "Low Expression", Absent => "No Expression",
    Over => "Overexpression", Deficient => "Deficient Expression", Positive => "Positive",
    Negative => "Negative", Equivocal => "Equivocal", Expression => "Expression"
});
text_table!(ScoringSystem, "The written expression scoring system.", Caseless, {
    Tps => "TPS", Cps => "CPS", Ic => "IC", Tc => "TC", Expression => "Expression",
    Tap => "TAP", Tips2Plus => "TIPS2+", Ihc => "IHC"
});
text_table!(Comparison, "An exact score comparison.", Caseless, {
    LessOrEqual => "<=", GreaterOrEqual => ">=", Equal => "=", Less => "<", Greater => ">"
});
text_table!(Msi, "A stated microsatellite instability class.", Caseless, {High => "High", Low => "Low", Stable => "Stable"});
text_table!(Mmr, "A stated DNA mismatch repair finding.", Caseless, {
    Deficient => "Deficient DNA Mismatch Repair (dMMR)", Proficient => "Proficient DNA Mismatch Repair (pMMR)"
});
text_table!(Level, "A stated burden or risk level.", Caseless, {High => "High", Intermediate => "Intermediate", Medium => "Medium", Low => "Low"});

/// An exact nonnegative decimal, normalized by the parser without trailing fractional zeros.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Decimal {
    /// All significant decimal digits as an integer.
    pub digits: u64,
    /// The number of fractional places.
    pub scale: u8,
}
/// A comparison, inclusive range, or IHC grade.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ScoreBound {
    /// A comparison with one exact decimal.
    Compare(Comparison, Decimal),
    /// Inclusive bounds, written with an equals sign.
    Between(Decimal, Decimal),
    /// An IHC grade with an optional comparison.
    IhcGrade(Option<Comparison>, Decimal),
}
/// A score preserves its stated system, bound and percentage unit.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Score {
    /// The stated scoring system.
    pub system: ScoringSystem,
    /// Its exact bound.
    pub bound: ScoreBound,
    /// Whether a percent sign was stated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub percent: bool,
}
/// A whole-sample signature, without an inferred threshold or result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Signature {
    /// Microsatellite instability.
    Msi(Msi),
    /// DNA mismatch repair.
    Mmr(Mmr),
    /// Tumor mutation burden.
    Tmb(Level),
    /// Polygenic risk.
    PolygenicRisk(Level),
    /// Homologous recombination deficiency.
    Hrd(Finding),
    /// Genomic loss of heterozygosity.
    GenomicLoh(Finding),
    /// Circulating tumor DNA.
    Ctdna(Finding),
}
const SCORE: Expect = Expect {
    code: Code::Alteration(AlterationCode::ScoreInvalid),
    label: "exact score",
};
const MARKER: Expect = Expect {
    code: Code::Alteration(AlterationCode::MarkerInvalid),
    label: "marker result",
};

pub(super) fn signature(input: &mut Input<'_>) -> Result<Kind> {
    alt((known_signature, unsupported_signature)).parse_next(input)
}
fn known_signature(input: &mut Input<'_>) -> Result<Kind> {
    step(input)?;
    alt((
        preceded(
            (Caseless("MSI"), alt(('-', '_'))),
            Msi::parse.verify(|v| *v != Msi::Stable),
        )
        .map(Signature::Msi),
        Caseless("Microsatellite Stable (MSS)").value(Signature::Msi(Msi::Stable)),
        Mmr::parse.map(Signature::Mmr),
        preceded(
            Caseless("TMB-"),
            Level::parse.verify(|v| *v != Level::Medium),
        )
        .map(Signature::Tmb),
        preceded(
            Caseless("Polygenic Risk - "),
            Level::parse.verify(|v| *v != Level::Intermediate),
        )
        .map(Signature::PolygenicRisk),
        preceded(Caseless("HRD "), Finding::parse).map(Signature::Hrd),
        preceded(Caseless("Genomic LOH "), Finding::parse).map(Signature::GenomicLoh),
        preceded(Caseless("ctDNA "), Finding::parse).map(Signature::Ctdna),
    ))
    .context(MARKER)
    .map(Kind::Signature)
    .parse_next(input)
    .and_then(|kind| {
        // A signature word can also be a gene subject, such as HRD A1V.
        // Only a complete signature takes precedence over the gene grammar.
        super::end_change(input)?;
        Ok(kind)
    })
}

fn unsupported_signature(input: &mut Input<'_>) -> Result<Kind> {
    let prefix = alt((
        Caseless("MSI-"),
        Caseless("MSI_"),
        Caseless("TMB-"),
        Caseless("TMB "),
        Caseless("HRD "),
        Caseless("Genomic LOH "),
        Caseless("ctDNA "),
        Caseless("Polygenic Risk - "),
    ))
    .parse_next(input)?;
    // A signature subject can also carry an ordinary gene marker.
    if prefix.ends_with(' ') {
        not((
            winnow::combinator::backtrack_err(super::gene_change_value),
            super::end_change,
        ))
        .parse_next(input)?;
    }
    refuse(
        Code::Alteration(AlterationCode::SignatureUnsupported),
        (
            take_while(1.., |c: char| c.is_ascii_alphabetic()),
            super::end_change,
        )
            .take(),
    )
    .parse_next(input)
}

pub(super) fn change(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    alt((
        preceded(
            Caseless("Promoter Methylation"),
            cut_err(preceded(' ', Finding::parse)),
        )
        .context(MARKER)
        .map(Change::PromoterMethylation),
        splice,
        expression,
    ))
    .parse_next(input)
}
fn splice(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    if input
        .get(..14)
        .is_some_and(|word| word.eq_ignore_ascii_case("splice variant"))
    {
        return refuse(
            Code::Alteration(AlterationCode::SpliceUnsupported),
            Caseless("Splice Variant"),
        )
        .parse_next(input);
    }
    let has_splice = input
        .as_bytes()
        .windows(15)
        .any(|word| word.eq_ignore_ascii_case(b" splice variant"));
    // A V followed by Roman letters is never converted to a variant number.
    if has_splice && !input.chars().nth(1).is_some_and(|c| c.is_ascii_digit()) {
        return refuse(
            Code::Alteration(AlterationCode::SpliceUnsupported),
            take_while(1.., |c: char| c != ' '),
        )
        .parse_next(input);
    }
    let mut numbered = (
        preceded(Caseless("V"), decimal::<u8>),
        Caseless(" Splice Variant"),
    );
    let (number, _) = if has_splice {
        cut_err(numbered).context(MARKER).parse_next(input)?
    } else {
        numbered.parse_next(input)?
    };
    if number == 0 {
        return reject(input, Some(MARKER));
    }
    Ok(Change::SpliceVariant(number))
}
fn expression(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    let state = alt((
        Caseless("Expression Positive").value(ExpressionState::Positive),
        Caseless("Expression Negative").value(ExpressionState::Negative),
        ExpressionState::parse,
    ))
    .parse_next(input)?;
    let score = if opt((
        not(winnow::combinator::peek(winnow::combinator::backtrack_err(
            super::origin,
        ))),
        winnow::combinator::peek(" ("),
    ))
    .parse_next(input)?
    .is_some()
    {
        Some(score(input)?)
    } else {
        None
    };
    Ok(Change::Expression { state, score })
}
fn score(input: &mut Input<'_>) -> Result<Score> {
    step(input)?;
    " (".parse_next(input)?;
    cut_err(score_body).context(SCORE).parse_next(input)
}
fn score_body(input: &mut Input<'_>) -> Result<Score> {
    step(input)?;
    let Some(system) = opt(ScoringSystem::parse).parse_next(input)? else {
        return refuse(
            Code::Alteration(AlterationCode::ScoringSystemUnsupported),
            take_while(1.., |c: char| c != ' ' && c != ')'),
        )
        .parse_next(input);
    };
    ' '.parse_next(input)?;
    let comparison = opt(alt((
        Caseless("≥").value(Comparison::GreaterOrEqual),
        Caseless("≤").value(Comparison::LessOrEqual),
        Comparison::parse,
    )))
    .parse_next(input)?;
    if comparison.is_some() {
        ' '.parse_next(input)?;
    }
    let first = exact(input)?;
    let bound = if system == ScoringSystem::Ihc {
        if first.digits != 0 {
            '+'.parse_next(input)?;
        }
        if first.greater_than(Decimal {
            digits: 3,
            scale: 0,
        }) {
            return fail.parse_next(input);
        }
        ScoreBound::IhcGrade(comparison, first)
    } else {
        let Some(comparison) = comparison else {
            return fail.parse_next(input);
        };
        match opt(preceded('-', exact)).parse_next(input)? {
            Some(last) => {
                if comparison != Comparison::Equal || !last.greater_than(first) {
                    return fail.parse_next(input);
                }
                ScoreBound::Between(first, last)
            }
            None => ScoreBound::Compare(comparison, first),
        }
    };
    let percent = opt('%').parse_next(input)?.is_some();
    if system == ScoringSystem::Ihc && percent {
        return fail.parse_next(input);
    }
    ')'.parse_next(input)?;
    Ok(Score {
        system,
        bound,
        percent,
    })
}
fn exact(input: &mut Input<'_>) -> Result<Decimal> {
    step(input)?;
    (
        winnow::ascii::digit1,
        opt(preceded('.', cut_err(winnow::ascii::digit1))),
    )
        .verify_map(|(whole, fraction): (&str, Option<&str>)| {
            let fraction = fraction.unwrap_or("").trim_end_matches('0');
            let digits = whole
                .bytes()
                .chain(fraction.bytes())
                .try_fold(0_u64, |value, digit| {
                    value.checked_mul(10)?.checked_add(u64::from(digit - b'0'))
                })?;
            let scale = if digits == 0 {
                0
            } else {
                u8::try_from(fraction.len()).ok()?
            };
            Some(Decimal { digits, scale })
        })
        .parse_next(input)
}
impl Decimal {
    fn greater_than(self, other: Self) -> bool {
        if self.digits == 0 || other.digits == 0 {
            return self.digits > other.digits;
        }
        let gap = self.scale.abs_diff(other.scale);
        if gap > 19 {
            return self.scale < other.scale;
        }
        let factor = 10_u128.pow(u32::from(gap));
        if self.scale < other.scale {
            u128::from(self.digits) * factor > u128::from(other.digits)
        } else {
            u128::from(self.digits) > u128::from(other.digits) * factor
        }
    }
}
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scale == 0 {
            return write!(f, "{}", self.digits);
        }
        let scale = u32::from(self.scale);
        if let Some(factor) = 10_u64.checked_pow(scale) {
            write!(
                f,
                "{}.{:0width$}",
                self.digits / factor,
                self.digits % factor,
                width = usize::from(self.scale)
            )
        } else {
            f.write_str("0.")?;
            for _ in 0..scale.saturating_sub(self.digits.max(1).ilog10() + 1) {
                f.write_str("0")?;
            }
            write!(f, "{}", self.digits)
        }
    }
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.system)?;
        match self.bound {
            ScoreBound::Compare(comparison, value) => write!(f, "{comparison} {value}")?,
            ScoreBound::Between(first, last) => write!(f, "= {first}-{last}")?,
            ScoreBound::IhcGrade(comparison, value) => {
                if let Some(comparison) = comparison {
                    write!(f, "{comparison} ")?;
                }
                write!(f, "{value}")?;
                if value.digits != 0 {
                    f.write_str("+")?;
                }
            }
        }
        if self.percent {
            f.write_str("%")?;
        }
        Ok(())
    }
}
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Msi(Msi::Stable) => f.write_str("Microsatellite Stable (MSS)"),
            Self::Msi(value) => write!(f, "MSI-{value}"),
            Self::Mmr(value) => write!(f, "{value}"),
            Self::Tmb(value) => write!(f, "TMB-{value}"),
            Self::PolygenicRisk(value) => write!(f, "Polygenic Risk - {value}"),
            Self::Hrd(value) => write!(f, "HRD {value}"),
            Self::GenomicLoh(value) => write!(f, "Genomic LOH {value}"),
            Self::Ctdna(value) => write!(f, "ctDNA {value}"),
        }
    }
}
