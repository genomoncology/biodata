//! Marker refusals for observation families that have no consumer.
use super::{IscnCode, grammar::chromosome};
use crate::Code;
use crate::parse::{Input, Result, refuse, step};
use winnow::Parser;
use winnow::combinator::{alt, not, opt, peek, terminated};
use winnow::token::{one_of, take_while};

// All entry points share these markers. Refuse immediately, without parsing
// the unsupported observation's body.
pub(super) fn unsupported<T>(input: &mut Input<'_>) -> Result<T> {
    step(input)?;
    alt((
        refuse(
            Code::Iscn(IscnCode::FishUnsupported),
            terminated(alt(("nuc ish", "fib ish", "rev ish", "ish")), boundary),
        ),
        refuse(
            Code::Iscn(IscnCode::ArrayUnsupported),
            terminated(
                "arr",
                alt((
                    peek(one_of(|c: char| c.is_ascii_digit() || "XY?".contains(c))).void(),
                    boundary,
                )),
            ),
        ),
        refuse(
            Code::Iscn(IscnCode::RsaUnsupported),
            terminated("rsa", boundary),
        ),
        refuse(
            Code::Iscn(IscnCode::SequenceUnsupported),
            terminated("seq", boundary),
        ),
        refuse(
            Code::Iscn(IscnCode::MeioticUnsupported),
            terminated(
                (
                    opt(alt(("fem ", "mal "))),
                    alt((
                        alt(("MII", "AII", "PI", "MI", "AI")),
                        alt(("spm", "oom", "lep", "zyg", "pac", "dip", "dit", "dia")),
                    )),
                ),
                peek(','),
            ),
        ),
        refuse(
            Code::Iscn(IscnCode::UpdUnsupported),
            terminated("upd", boundary),
        ),
        refuse(
            Code::Iscn(IscnCode::NormalVariantUnsupported),
            (
                peek(one_of(|c: char| c.is_ascii_digit() || "XY?".contains(c))),
                chromosome,
                alt(("cen", "p", "q")),
                take_while(0.., |c: char| c.is_ascii_digit() || c == '.'),
                alt(("h", "stk", "s", "var")),
            ),
        ),
        refuse(Code::Iscn(IscnCode::CompoundUnsupported), '.'),
    ))
    .parse_next(input)
}

fn boundary(input: &mut Input<'_>) -> Result<()> {
    step(input)?;
    not(one_of(|c: char| c.is_ascii_alphanumeric() || c == '_')).parse_next(input)
}
