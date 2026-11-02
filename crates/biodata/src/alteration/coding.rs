//! Molecular shorthand shares the HGVS nucleotide and protein grammars.
use super::{AlterationCode, Category, Change, GeneEvent, Scope};
use crate::Code;
use crate::hgvs::NaChange;
use crate::parse::{Expect, Input, Result, decimal, refuse, step};
use winnow::Parser;
use winnow::combinator::{alt, cut_err, not, opt, peek};

const CODING: Expect = Expect {
    code: Code::Alteration(AlterationCode::EventInvalid),
    label: "coding interval or molecular detail",
};

fn nucleotide(input: &mut Input<'_>) -> Result<NaChange> {
    peek(alt(("c.", "n."))).parse_next(input)?;
    crate::hgvs::nucleotide::change(input).map(|(change, _)| change)
}

pub(super) fn change(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    if let Some(scope) = opt(span).parse_next(input)? {
        let category = cut_err(Category::parse).context(CODING).parse_next(input)?;
        return Ok(Change::Event(GeneEvent {
            scope: Some(scope),
            category: Some(category),
            effect: None,
            detail: None,
        }));
    }
    if input.starts_with("c.") || input.starts_with("n.") {
        let coding = nucleotide(input)?;
        let paired = input
            .strip_prefix(' ')
            .is_some_and(|tail| !tail.trim().is_empty() && !tail.starts_with('('));
        if paired {
            ' '.parse_next(input)?;
            let protein = protein(input)?;
            return Ok(Change::Protein {
                protein,
                coding: Some(coding),
            });
        }
        return Ok(Change::Coding(coding));
    }
    let protein = protein(input)?;
    let coding = opt(winnow::combinator::preceded(
        peek((' ', alt(("c.", "n.")))),
        winnow::combinator::preceded(' ', nucleotide),
    ))
    .parse_next(input)?;
    Ok(Change::Protein { protein, coding })
}

fn span(input: &mut Input<'_>) -> Result<Scope> {
    step(input)?;
    let (_, first, _) =
        ("c.", decimal::<u32>.verify(|n| *n > 0), alt(("-c.", "~c."))).parse_next(input)?;
    let (last, _) = cut_err((decimal::<u32>.verify(|n| *n > first), ' '))
        .context(CODING)
        .parse_next(input)?;
    Ok(Scope::CodingSpan { first, last })
}

pub(super) fn detail(
    input: &mut Input<'_>,
    category: Category,
) -> Result<Option<crate::hgvs::Change>> {
    step(input)?;
    if !matches!(
        category,
        Category::Itd | Category::Promoter | Category::Skipping
    ) {
        return Ok(None);
    }
    if opt((
        not(peek(winnow::combinator::backtrack_err(super::origin))),
        peek(" ("),
    ))
    .parse_next(input)?
    .is_none()
    {
        return Ok(None);
    }
    " (".parse_next(input)?;
    let value = cut_err(|input: &mut Input<'_>| {
        if input.starts_with("c.") || input.starts_with("n.") {
            nucleotide(input).map(crate::hgvs::Change::Nucleotide)
        } else {
            protein(input).map(crate::hgvs::Change::Protein)
        }
    })
    .context(CODING)
    .parse_next(input)?;
    cut_err(')').context(CODING).parse_next(input)?;
    Ok(Some(value))
}

pub(super) fn protein(input: &mut Input<'_>) -> Result<crate::hgvs::ProteinChange> {
    step(input)?;
    opt("p.").parse_next(input)?;
    let text: &str = input;
    let token = text.split(' ').next().unwrap_or(text);
    if token.starts_with('(') && token.ends_with(')') {
        return refuse(
            Code::Alteration(AlterationCode::MultiplicityUnsupported),
            '(',
        )
        .parse_next(input);
    }
    if token.contains(['-', '~']) && !token.contains("ext-") {
        return refuse(
            Code::Alteration(AlterationCode::RangeUnsupported),
            winnow::token::take_till(1.., |c| c == '-' || c == '~'),
        )
        .parse_next(input);
    }
    // Only a token ending in a number can be an unsupported codon-only form.
    // Probe its own cursor, leaving the shared protein grammar to read changes once.
    if token.ends_with(|c: char| c.is_ascii_digit()) {
        let mut codon_input = *input;
        let site = opt((crate::residue::residue, decimal::<u32>)).parse_next(&mut codon_input)?;
        if site.is_some_and(|(_, position)| position > 0)
            && (codon_input.is_empty() || codon_input.starts_with(' '))
        {
            return refuse(
                Code::Alteration(AlterationCode::CodonUnsupported),
                winnow::token::take_till(1.., ' '),
            )
            .parse_next(input);
        }
    }
    // Inserted counts and spaces are malformed in shorthand, not opaque sequences.
    if token.contains("ins")
        && token.rsplit("ins").next().is_some_and(|s| {
            s.is_empty() || s.starts_with(|c: char| c.is_ascii_digit()) || s.starts_with("X[")
        })
    {
        return refuse(
            Code::Alteration(AlterationCode::ProteinInvalid),
            winnow::token::take_till(1.., ' '),
        )
        .parse_next(input);
    }
    let protein = cut_err(crate::hgvs::protein::body)
        .context(Expect {
            code: Code::Alteration(AlterationCode::ProteinInvalid),
            label: "protein change",
        })
        .parse_next(input)?;
    Ok(protein)
}
