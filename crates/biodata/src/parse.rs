//! The parse wrapper: winnow plus the length and step budget, and the one
//! place a winnow error becomes a [`Refusal`].

use crate::outcome::{Code, InputCode, Outcome, Parsed, Refusal, Span};
use std::fmt::{self, Display, Write as _};
use std::str::FromStr;
use winnow::error::{AddContext, ErrMode, ParserError};
use winnow::stream::{Location, Stream};
use winnow::{LocatingSlice, Parser, Stateful};

/// The longest text, in bytes, any parse accepts.
pub const MAX_LEN: usize = 4096;
const MAX_STEPS: u32 = 16_384;

/// The cursor every grammar reads: located text plus the budget spent so far.
pub(crate) type Input<'a> = Stateful<LocatingSlice<&'a str>, Budget>;
pub(crate) type Result<T> = winnow::ModalResult<T, Fail>;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Budget {
    steps: u32,
}

/// A grammar failure: where it happened, and the code and label of the
/// innermost construct that names it.
pub(crate) struct Fail {
    start: usize,
    end: Option<usize>,
    code: Option<Code>,
    expected: Option<&'static str>,
}

/// The code and label a construct gives a failure inside it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Expect {
    pub(crate) code: Code,
    pub(crate) label: &'static str,
}

impl ParserError<Input<'_>> for Fail {
    type Inner = Self;

    fn from_input(input: &Input<'_>) -> Self {
        Self {
            start: input.current_token_start(),
            end: None,
            code: None,
            expected: None,
        }
    }

    fn into_inner(self) -> std::result::Result<Self, Self> {
        Ok(self)
    }
}

impl AddContext<Input<'_>, Expect> for Fail {
    fn add_context(
        self,
        _: &Input<'_>,
        _: &<Input<'_> as Stream>::Checkpoint,
        expect: Expect,
    ) -> Self {
        match self.code {
            Some(_) => self,
            None => Self {
                code: Some(expect.code),
                expected: Some(expect.label),
                ..self
            },
        }
    }
}

const TOO_LONG: Refusal = Refusal {
    code: Code::Input(InputCode::LengthLimit),
    span: None,
    expected: None,
};

/// Runs a grammar over the whole text within the budget.
pub(crate) fn run<T: Display>(text: &str, grammar: fn(&mut Input<'_>) -> Result<T>) -> Parsed<T> {
    Parsed {
        source: text.into(),
        outcome: value(text, grammar),
    }
}

pub(crate) fn value<T: Display>(
    text: &str,
    mut grammar: fn(&mut Input<'_>) -> Result<T>,
) -> Outcome<T> {
    if text.len() > MAX_LEN {
        Outcome::refused(TOO_LONG)
    } else {
        let input = Stateful {
            input: LocatingSlice::new(text),
            state: Budget::default(),
        };
        match grammar.parse(input) {
            Ok(value) => {
                if write!(&mut CanonicalBudget(MAX_LEN), "{value}").is_err() {
                    Outcome::refused(Refusal {
                        code: Code::Input(InputCode::RenderLengthLimit),
                        span: None,
                        expected: None,
                    })
                } else {
                    Outcome::Parsed(value)
                }
            }
            Err(error) => Outcome::refused(error.into_inner().refusal(text)),
        }
    }
}

struct CanonicalBudget(usize);
impl fmt::Write for CanonicalBudget {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.0 = self.0.checked_sub(text.len()).ok_or(fmt::Error)?;
        Ok(())
    }
}

/// Parses bytes that may not be UTF-8, such as one line of a stream. Length is
/// checked first, so a line cut inside a character is refused as too long.
pub fn parse_bytes<T>(bytes: &[u8], parse: fn(&str) -> Parsed<T>) -> Parsed<T> {
    let error = match std::str::from_utf8(bytes) {
        Ok(text) => return parse(text),
        Err(error) => error,
    };
    let refusal = if bytes.len() > MAX_LEN {
        TOO_LONG
    } else {
        let start = error.valid_up_to();
        let end = start + error.error_len().unwrap_or(bytes.len() - start);
        let span = Span {
            start: start as u32,
            end: end as u32,
        };
        Refusal {
            code: Code::Input(InputCode::Utf8Invalid),
            span: Some(span),
            expected: None,
        }
    };
    Parsed {
        source: String::from_utf8_lossy(bytes).into(),
        outcome: Outcome::refused(refusal),
    }
}

impl Fail {
    pub(crate) fn override_context(mut self, expect: Expect) -> Self {
        if !self.code.is_some_and(Code::is_limit) {
            self.code = Some(expect.code);
            self.expected = Some(expect.label);
        }
        self
    }

    fn refusal(self, text: &str) -> Refusal {
        // Only the final end-of-input check fails without a code.
        let (code, end) = match self.code {
            Some(code) => (code, self.end),
            None => (Code::Input(InputCode::TrailingInvalid), Some(text.len())),
        };
        let end = end.unwrap_or_else(|| {
            text.get(self.start..)
                .and_then(|rest| rest.chars().next())
                .map_or(self.start, |c| self.start + c.len_utf8())
        });
        // Offsets fit: the text is at most MAX_LEN bytes.
        let span = Span {
            start: self.start as u32,
            end: end as u32,
        };
        Refusal {
            code,
            span: Some(span),
            expected: self.expected,
        }
    }
}

/// Spends one step of the budget. Every grammar rule calls it first.
pub(crate) fn step(input: &mut Input<'_>) -> Result<()> {
    input.state.steps += 1;
    if input.state.steps > MAX_STEPS {
        return Err(limit(input, InputCode::StepLimit));
    }
    Ok(())
}

fn limit(input: &Input<'_>, code: InputCode) -> ErrMode<Fail> {
    ErrMode::Cut(Fail {
        code: Some(Code::Input(code)),
        ..Fail::from_input(input)
    })
}

/// Refuses, with the given code, the construct the parser recognizes. A
/// construct this crate does not parse yet is refused at its marker.
pub(crate) fn refuse<'a, O, T>(
    code: Code,
    mut marker: impl Parser<Input<'a>, O, ErrMode<Fail>>,
) -> impl Parser<Input<'a>, T, ErrMode<Fail>> {
    move |input: &mut Input<'a>| {
        let start = input.current_token_start();
        marker.parse_next(input)?;
        Err(ErrMode::Cut(Fail {
            start,
            end: Some(input.current_token_start()),
            code: Some(code),
            expected: None,
        }))
    }
}

/// A run of ASCII digits read as a number. A number out of range fails over the whole run.
pub(crate) fn decimal<T: FromStr>(input: &mut Input<'_>) -> Result<T> {
    let (digits, range) = winnow::ascii::digit1.with_span().parse_next(input)?;
    let digits: &str = digits;
    digits.parse().map_err(|_| {
        ErrMode::Backtrack(Fail {
            start: range.start,
            end: Some(range.end),
            code: None,
            expected: None,
        })
    })
}

/// Commits a failure at the current cursor with the owning construct's expectation.
pub(crate) fn reject<T>(input: &mut Input<'_>, expect: Option<Expect>) -> Result<T> {
    let failure = Fail::from_input(input);
    Err(ErrMode::Cut(match expect {
        Some(expect) => failure.add_context(input, &input.checkpoint(), expect),
        None => failure,
    }))
}
