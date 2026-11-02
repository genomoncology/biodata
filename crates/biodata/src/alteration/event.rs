//! Gene-event shorthand uses the shared cursor and budgets.
use super::{AlterationCode, Change};
use crate::Code;
use crate::parse::{Expect, Input, Result, decimal, refuse, step};
use std::fmt;
use winnow::Parser;
use winnow::ascii::Caseless;
use winnow::combinator::{alt, cut_err, opt};

const EVENT: Expect = Expect {
    code: Code::Alteration(AlterationCode::EventInvalid),
    label: "gene event category and compatible scope",
};

pub(super) fn change(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    for (text, code) in [
        ("Inversion", AlterationCode::InversionUnsupported),
        ("Wild Type", AlterationCode::WildTypeUnsupported),
        ("Status Unknown", AlterationCode::StatusUnknownUnsupported),
    ] {
        if opt(winnow::combinator::peek(word(text)))
            .parse_next(input)?
            .is_some()
        {
            return refuse(Code::Alteration(code), word(text)).parse_next(input);
        }
    }
    if opt(winnow::combinator::peek((
        '(',
        winnow::token::take_while(1.., |c: char| c.is_ascii_alphabetic()),
        ')',
        ' ',
    )))
    .parse_next(input)?
    .is_some()
    {
        return refuse(
            Code::Alteration(AlterationCode::SegmentUnsupported),
            ('(', winnow::token::take_till(1.., ')'), ')'),
        )
        .parse_next(input);
    }
    opt(refuse::<_, ()>(
        Code::Alteration(AlterationCode::EventInvalid),
        word("Del"),
    ))
    .parse_next(input)?;
    // Exon Spanning is a category, while Exon Deletion has an unnumbered scope.
    let spanning = opt(winnow::combinator::peek(word("Exon-Spanning")))
        .parse_next(input)?
        .is_some();
    let scope = if spanning {
        None
    } else {
        opt(scope).context(EVENT).parse_next(input)?
    };
    let effect = opt(effect).parse_next(input)?;
    let category =
        if scope == Some(Scope::Tkd) && opt(super::end_change).parse_next(input)?.is_some() {
            None
        } else if scope.is_some() || effect.is_some() {
            Some(cut_err(Category::parse).context(EVENT).parse_next(input)?)
        } else {
            Some(Category::parse(input)?)
        };
    let compatible = match scope {
        Some(Scope::Exons { last: Some(_), .. }) => matches!(
            category,
            Some(Category::Skipping | Category::Amplification | Category::Loss)
        ),
        Some(Scope::Codon(_)) => category == Some(Category::Missense),
        Some(Scope::Tkd) => matches!(
            category,
            None | Some(Category::Mutation | Category::Deletion | Category::Insertion)
        ),
        _ => true,
    };
    if !compatible {
        return refuse(
            Code::Alteration(AlterationCode::EventInvalid),
            winnow::combinator::empty,
        )
        .parse_next(input);
    }
    Ok(Change::Event(GeneEvent {
        scope,
        category,
        effect,
        detail: match category {
            Some(category) => super::coding::detail(input, category)?,
            None => None,
        },
    }))
}

fn effect(input: &mut Input<'_>) -> Result<Effect> {
    step(input)?;
    let effect = Effect::parse(input)?;
    cut_err(' ').context(EVENT).parse_next(input)?;
    Ok(effect)
}

fn scope(input: &mut Input<'_>) -> Result<Scope> {
    step(input)?;
    if opt(word("TKD")).parse_next(input)?.is_some() {
        if !input.starts_with(" (") {
            opt(' ').parse_next(input)?;
        }
        return Ok(Scope::Tkd);
    }
    if opt(word("Codon")).parse_next(input)?.is_some() {
        let (_, number, _) = cut_err((' ', decimal::<u32>.verify(|n| *n > 0), ' '))
            .context(EVENT)
            .parse_next(input)?;
        return Ok(Scope::Codon(number));
    }
    let plural = alt((word("Exons").value(true), word("Exon").value(false))).parse_next(input)?;
    cut_err(' ').context(EVENT).parse_next(input)?;
    let numbered = input.starts_with(|c: char| c.is_ascii_digit());
    if !plural && !numbered {
        return Ok(Scope::UnnumberedExon);
    }
    let first = cut_err(decimal::<u16>.verify(|n| *n > 0))
        .context(EVENT)
        .parse_next(input)?;
    let last = opt(('-', cut_err(decimal::<u16>.verify(|n| *n > first))))
        .context(EVENT)
        .parse_next(input)?
        .map(|(_, n)| n);
    if plural && last.is_none() {
        return refuse(
            Code::Alteration(AlterationCode::EventInvalid),
            winnow::combinator::empty,
        )
        .parse_next(input);
    }
    cut_err(' ').context(EVENT).parse_next(input)?;
    Ok(Scope::Exons { first, last })
}

/// Recognizes the category suffix without splitting a fusion subject.
pub(super) fn hyphenated(input: &mut Input<'_>) -> Result<(crate::GeneSymbol, Change)> {
    step(input)?;
    let text: &str = input;
    let token = text.split(' ').next().unwrap_or(text);
    let Some((subject, suffix)) = token.rsplit_once('-') else {
        return winnow::combinator::fail.parse_next(input);
    };
    let category = match suffix {
        s if s.eq_ignore_ascii_case("ITD") => Category::Itd,
        s if s.eq_ignore_ascii_case("PTD") => Category::Ptd,
        s if s.eq_ignore_ascii_case("KDD") => Category::Kdd,
        _ => return winnow::combinator::fail.parse_next(input),
    };
    // The suffix belongs to a category only when no change follows it.
    if !(text.len() == token.len()
        || opt(winnow::combinator::peek(|input: &mut Input<'_>| {
            winnow::token::take(token.chars().count()).parse_next(input)?;
            super::end_change(input)
        }))
        .parse_next(input)?
        .is_some())
    {
        return winnow::combinator::fail.parse_next(input);
    }
    let symbol = crate::value::gene_symbol_prefix(input, subject.len())?;
    ('-', Caseless(suffix)).parse_next(input)?;
    Ok((
        symbol,
        Change::Event(GeneEvent {
            scope: None,
            category: Some(category),
            effect: None,
            detail: super::coding::detail(input, category)?,
        }),
    ))
}

use crate::table::text_table;
text_table!(Category, "A stated gene-event category.", word, {
   Mutation => "Mutation",
   Missense => "Missense",
   Nonsense => "Nonsense",
   Frameshift => "Frameshift",
   Deletion => "Deletion",
   Insertion => "Insertion",
   Substitution => "Substitution",
   SpliceSite => "Splice Site",
   StopLoss => "Stop Loss",
   Skipping => "Skipping",
   BiallelicMutation => "Biallelic Mutation",
   SomaticHypermutation => "Somatic Hypermutation",
   Promoter => "Promoter",
   Fusion => "Fusion",
   Itd => "ITD",
   Ptd => "PTD",
   Kdd => "KDD",
   Amplification => "Amplification",
   Loss => "Loss",
   Synonymous => "(=)",
   NonExonic => "Non-Exonic",
   SpliceRegion => "Splice Region",
   ExonSpanning => "Exon-Spanning",
   Rearrangement => "Rearrangement",
   CnvEquivocal => "CNV Equivocal",
});

pub(super) fn word<'a>(
    text: &'static str,
) -> impl Parser<Input<'a>, (), winnow::error::ErrMode<crate::parse::Fail>> {
    move |input: &mut Input<'a>| {
        if let Some((first, second)) = text.split_once([' ', '-']) {
            Caseless(first).parse_next(input)?;
            opt(winnow::token::one_of([' ', '-', '_'])).parse_next(input)?;
            Caseless(second).parse_next(input)?;
        } else {
            Caseless(text).parse_next(input)?;
        }
        // A keyword cannot consume the prefix of another token.
        winnow::combinator::peek(winnow::combinator::alt((
            winnow::ascii::space1.void(),
            winnow::combinator::eof.void(),
        )))
        .parse_next(input)?;
        Ok(())
    }
}

/// A stated category with an optional region and effect.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GeneEvent {
    /// The stated region.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scope: Option<Scope>,
    /// The stated category.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub category: Option<super::Category>,
    /// The stated activating or inactivating effect.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub effect: Option<Effect>,
    /// A stated molecular detail.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub detail: Option<crate::hgvs::Change>,
}
/// The region to which an event applies.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Scope {
    /// An inclusive coding interval.
    CodingSpan {
        /// First coding position.
        first: u32,
        /// Last coding position.
        last: u32,
    },
    /// One exon or an inclusive range.
    Exons {
        /// The first exon.
        first: u16,
        /// The last exon in a range.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        last: Option<u16>,
    },
    /// An exon without a stated number.
    UnnumberedExon,
    /// A stated codon number.
    Codon(u32),
    /// The tyrosine kinase domain.
    Tkd,
}
crate::table::text_table!(Effect, "A stated effect without inferring it from sequence.", word, {
    Activating => "Activating",
    Inactivating => "Inactivating",
}, "snake_case");
impl fmt::Display for GeneEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(scope) = &self.scope {
            write!(f, "{scope}")?;
            if self.category.is_some() || self.effect.is_some() {
                f.write_str(" ")?;
            }
        }
        if let Some(effect) = self.effect {
            write!(f, "{effect} ")?;
        }
        if let Some(category) = self.category {
            write!(f, "{category}")?;
        }
        if let Some(detail) = &self.detail {
            write!(f, " ({detail:#})")?;
        }
        Ok(())
    }
}
impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodingSpan { first, last } => write!(f, "c.{first}-c.{last}"),
            Self::Exons {
                first,
                last: Some(last),
            } => write!(f, "Exons {first}-{last}"),
            Self::Exons { first, last: None } => write!(f, "Exon {first}"),
            Self::UnnumberedExon => f.write_str("Exon"),
            Self::Codon(number) => write!(f, "Codon {number}"),
            Self::Tkd => f.write_str("TKD"),
        }
    }
}
