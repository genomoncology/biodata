//! Star allele and HLA notation, without allele interpretation.
use super::{AlterationCode, Change};
use crate::Code;
use crate::parse::{Expect, Input, Result, decimal, refuse, step};
use std::fmt;
use winnow::Parser;
use winnow::ascii::Caseless;
use winnow::combinator::{alt, cut_err, opt};
use winnow::token::one_of;

/// A numbered star allele and its optional suffix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Star {
    /// The allele number as written, without interpretation.
    pub number: u16,
    /// A letter or decimal suballele number.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub suffix: Option<StarSuffix>,
}
/// A suffix on a star allele.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum StarSuffix {
    /// An uppercase ASCII letter.
    Letter(u8),
    /// A decimal suballele number, rendered with at least three digits.
    Sub(u16),
}
crate::table::text_table!(HlaSuffix, "An HLA expression or protein group suffix.", |text| text, {
    N => "N",
    L => "L",
    S => "S",
    C => "C",
    A => "A",
    Q => "Q",
    P => "P",
}, "SCREAMING_SNAKE_CASE");
const STAR: Expect = Expect {
    code: Code::Alteration(AlterationCode::StarInvalid),
    label: "star allele or genotype",
};
const HLA: Expect = Expect {
    code: Code::Alteration(AlterationCode::HlaInvalid),
    label: "HLA allele with one to four fields of at least two digits",
};

pub(super) fn star_change(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    cut_err(|input: &mut Input<'_>| {
        let first = star(input)?;
        let second = if opt('/').parse_next(input)?.is_some() {
            Some(star(input)?)
        } else {
            Caseless(" allele").parse_next(input)?;
            None
        };
        Ok(Change::StarAlleles { first, second })
    })
    .context(STAR)
    .parse_next(input)
}
fn star(input: &mut Input<'_>) -> Result<Star> {
    step(input)?;
    '*'.parse_next(input)?;
    let number = decimal::<u16>.verify(|n| *n > 0).parse_next(input)?;
    let suffix = opt(alt((
        one_of('A'..='Z').map(|c| StarSuffix::Letter(c as u8)),
        ('.', cut_err(padded_number(3))).map(|(_, n)| StarSuffix::Sub(n)),
    )))
    .parse_next(input)?;
    Ok(Star { number, suffix })
}
pub(super) fn hla(input: &mut Input<'_>) -> Result<Change> {
    step(input)?;
    cut_err(hla_body).context(HLA).parse_next(input)
}
fn hla_body(input: &mut Input<'_>) -> Result<Change> {
    '*'.parse_next(input)?;
    let first = field(input)?;
    let mut fields = [Some(first), None, None, None];
    let mut count = 1;
    for field_slot in fields.iter_mut().skip(1) {
        if opt(':').parse_next(input)?.is_none() {
            break;
        }
        *field_slot = Some(field(input)?);
        count += 1;
    }
    if input.starts_with(':') {
        return refuse(Code::Alteration(AlterationCode::HlaInvalid), ':').parse_next(input);
    }
    let suffix = if input.starts_with(|c: char| c.is_ascii_alphabetic()) {
        // Any suffix needs at least two numeric fields, including unsupported groups.
        if count < 2 {
            return refuse(
                Code::Alteration(AlterationCode::HlaInvalid),
                one_of('A'..='Z'),
            )
            .parse_next(input);
        }
        if input.starts_with('G') {
            return refuse(Code::Alteration(AlterationCode::HlaGroupUnsupported), 'G')
                .parse_next(input);
        }
        Some(HlaSuffix::parse(input)?)
    } else {
        None
    };
    Ok(Change::Hla { fields, suffix })
}
fn field(input: &mut Input<'_>) -> Result<u16> {
    step(input)?;
    padded_number(2).parse_next(input)
}
fn padded_number<'a>(
    width: usize,
) -> impl Parser<Input<'a>, u16, winnow::error::ErrMode<crate::parse::Fail>> {
    crate::value::digits(move |number: u16| (number.max(1).ilog10() as usize + 1).max(width))
}
impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*{}", self.number)?;
        if let Some(suffix) = self.suffix {
            match suffix {
                StarSuffix::Letter(letter) => write!(f, "{}", char::from(letter))?,
                StarSuffix::Sub(number) => write!(f, ".{number:03}")?,
            }
        }
        Ok(())
    }
}
