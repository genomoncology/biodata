//! Reference-free coding syntax for search, without allele or transcript inference.
//! HGVS 21.1: https://hgvs-nomenclature.org/21.1.3/background/numbering/
//! https://hgvs-nomenclature.org/21.1.3/recommendations/DNA/substitution/
//! https://hgvs-nomenclature.org/21.1.3/recommendations/DNA/insertion/
//! https://hgvs-nomenclature.org/21.1.3/recommendations/DNA/deletion/
//! https://hgvs-nomenclature.org/21.1.3/recommendations/DNA/duplication/
//! https://hgvs-nomenclature.org/21.1.3/recommendations/DNA/delins/
//! Uppercase IUPAC DNA: https://hgvs-nomenclature.org/21.1.3/recommendations/general/
use crate::{Base, Error, Sequence, text_value};
use std::{fmt, str::FromStr};
/// A literal HGVS c. substitution, deletion, duplication, insertion or deletion-insertion.
/// Positions may include UTR prefixes and intronic offsets. This syntax check does not
/// validate a reference sequence, normalize an edit or select a transcript.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodingChange(Box<str>);
impl CodingChange {
    /// Borrow the unchanged search notation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for CodingChange {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let invalid = || Error::CodingChangeInvalid;
        let body = value.strip_prefix("c.").ok_or_else(invalid)?;
        let (rest, start) = position(body)?;
        let (rest, range) = if let Some(tail) = rest.strip_prefix('_') {
            let (rest, end) = position(tail)?;
            if start.compare(&end) != std::cmp::Ordering::Less {
                return Err(invalid());
            }
            (rest, Some(end))
        } else {
            (rest, None)
        };
        let sequence = |text: &str| !text.is_empty() && text.parse::<Sequence>().is_ok();
        let valid = if let Some(bases) = rest.strip_prefix("delins") {
            sequence(bases)
        } else if let Some(bases) = rest.strip_prefix("ins") {
            range.is_some_and(|end| start.adjacent(&end)) && sequence(bases)
        } else if matches!(rest, "del" | "dup") {
            true
        } else if let Some((reference, alternate)) = rest.split_once('>') {
            range.is_none()
                && reference.parse::<Base>().is_ok()
                && alternate.parse::<Base>().is_ok()
                && reference != alternate
        } else {
            false
        };
        if !valid {
            return Err(invalid());
        }
        Ok(Self(value.into()))
    }
}
// Positions remain decimal text. HGVS imposes no machine integer width.
struct Position<'a> {
    region: u8,
    anchor: &'a str,
    offset_sign: i8,
    offset: &'a str,
}
impl Position<'_> {
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.region.cmp(&other.region).then_with(|| {
            let anchor = decimal_compare(self.anchor, other.anchor);
            let anchor = if self.region == 0 {
                anchor.reverse()
            } else {
                anchor
            };
            anchor.then_with(|| {
                signed_compare(
                    self.offset_sign,
                    self.offset,
                    other.offset_sign,
                    other.offset,
                )
            })
        })
    }
    fn adjacent(&self, other: &Self) -> bool {
        if self.region == other.region && self.anchor == other.anchor {
            return incremented(
                self.offset_sign,
                self.offset,
                other.offset_sign,
                other.offset,
            );
        }
        // Even an intron cannot make skipped transcript anchors adjacent.
        // The final coding anchor is unknown, but the first downstream anchor is *1.
        if self.offset_sign < 0 || other.offset_sign > 0 {
            return false;
        }
        if self.region == other.region {
            if self.region == 0 {
                increment(other.anchor) == self.anchor
            } else {
                increment(self.anchor) == other.anchor
            }
        } else {
            self.region == 0 && other.region == 1 && self.anchor == "1" && other.anchor == "1"
                || self.region == 1 && other.region == 2 && other.anchor == "1"
        }
    }
}
fn decimal_compare(left: &str, right: &str) -> std::cmp::Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}
fn signed_compare(a_sign: i8, a: &str, b_sign: i8, b: &str) -> std::cmp::Ordering {
    a_sign.cmp(&b_sign).then_with(|| {
        let order = decimal_compare(a, b);
        if a_sign < 0 { order.reverse() } else { order }
    })
}
fn incremented(a_sign: i8, a: &str, b_sign: i8, b: &str) -> bool {
    match (a_sign, b_sign) {
        (-1, -1) => increment(b) == a,
        (-1, 0) => a == "1",
        (0, 1) => b == "1",
        (1, 1) => increment(a) == b,
        _ => false,
    }
}
fn increment(value: &str) -> String {
    let mut digits = value.as_bytes().to_vec();
    for digit in digits.iter_mut().rev() {
        if *digit != b'9' {
            *digit += 1;
            return digits.into_iter().map(char::from).collect();
        }
        *digit = b'0';
    }
    digits.insert(0, b'1');
    digits.into_iter().map(char::from).collect()
}
fn position(text: &str) -> Result<(&str, Position<'_>), Error> {
    let (text, region) = if let Some(tail) = text.strip_prefix('-') {
        (tail, 0)
    } else if let Some(tail) = text.strip_prefix('*') {
        (tail, 2)
    } else {
        (text, 1)
    };
    let (mut rest, anchor) = number(text)?;
    let (mut offset_sign, mut offset) = (0, "");
    if let Some(sign @ ('+' | '-')) = rest.chars().next() {
        let (tail, digits) = number(&rest[1..])?;
        offset_sign = if sign == '-' { -1 } else { 1 };
        offset = digits;
        rest = tail;
    }
    Ok((
        rest,
        Position {
            region,
            anchor,
            offset_sign,
            offset,
        },
    ))
}
fn number(text: &str) -> Result<(&str, &str), Error> {
    let length = text.bytes().take_while(u8::is_ascii_digit).count();
    let digits = &text[..length];
    if digits.is_empty() || digits.starts_with('0') {
        return Err(Error::CodingChangeInvalid);
    }
    Ok((&text[length..], digits))
}
impl fmt::Display for CodingChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    CodingChange,
    "CodingChange",
    {
        let n = "[1-9][0-9]*";
        let position = format!(r"[-*]?{n}(?:[+-]{n})?");
        let bases = Sequence::pattern();
        let bases = bases.trim_start_matches('^').trim_end_matches('$');
        // Sequence permits empty; the coding edit requires at least one base.
        let base = Base::pattern();
        let base = base.trim_start_matches('^').trim_end_matches('$');
        format!(
            concat!(
                r"^c\.(?:{position}{base}>{base}|",
                r"{position}(?:_{position})?(?:del|dup|delins(?={base}){bases})|",
                r"{position}_{position}ins(?={base}){bases})$"
            ),
            position = position,
            base = base,
            bases = bases
        )
    },
    "HGVS 21.1 coding edit syntax without a reference. The parser additionally requires \
    increasing range endpoints, different substitution bases and insertion adjacency where \
    literal positions establish it. It does not check transcript sequence or exon boundaries."
);
