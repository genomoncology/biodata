//! Checked static input for constructing bounded text without a runtime refusal.
use crate::{Error, Text, text_control::unsafe_control};

/// A static string that satisfies the same contract as `Text::new`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextLiteral(&'static str);

impl TextLiteral {
    /// Validate a static string, including Unicode whitespace and controls.
    pub const fn new(value: &'static str) -> Result<Self, Error> {
        let bytes = value.as_bytes();
        let mut offset = 0;
        let mut count = 0;
        let mut trailing = 0;
        let mut started = false;
        let mut control = false;
        while offset < bytes.len() {
            let (ch, width) = scalar(bytes, offset);
            offset += width;
            control = control || unsafe_control(ch);
            if whitespace(ch) {
                if started {
                    count += 1;
                    trailing += 1;
                }
            } else {
                started = true;
                count += 1;
                trailing = 0;
            }
        }
        if !started {
            Err(Error::TextEmpty)
        } else if control {
            Err(Error::TextControl)
        } else if count - trailing > Text::MAX_CHARS {
            Err(Error::TextTooLong)
        } else {
            Ok(Self(value))
        }
    }

    pub(crate) const fn source(self) -> &'static str {
        self.0
    }
}

// Rust's Unicode White_Space set, used by str::trim. Boundary tests compare
// this const-capable predicate with the existing runtime constructor.
const fn whitespace(ch: char) -> bool {
    matches!(ch, '\u{0009}'..='\u{000d}' | '\u{0020}' | '\u{0085}' | '\u{00a0}' |
        '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' |
        '\u{202f}' | '\u{205f}' | '\u{3000}')
}

#[expect(
    clippy::indexing_slicing,
    reason = "offset visits scalar boundaries of a valid str; each UTF-8 width is present"
)]
const fn scalar(bytes: &[u8], offset: usize) -> (char, usize) {
    let first = bytes[offset];
    let (mut code, width) = if first < 0x80 {
        (first as u32, 1)
    } else if first < 0xe0 {
        ((first & 0x1f) as u32, 2)
    } else if first < 0xf0 {
        ((first & 0x0f) as u32, 3)
    } else {
        ((first & 0x07) as u32, 4)
    };
    let mut n = 1;
    while n < width {
        code = (code << 6) | (bytes[offset + n] & 0x3f) as u32;
        n += 1;
    }
    (
        match char::from_u32(code) {
            Some(ch) => ch,
            None => '\0',
        },
        width,
    )
}

/// Construct `Text` from a literal validated during compilation.
///
/// ```
/// let label = biodata::text!("  source label  ");
/// assert_eq!(label.as_str(), "source label");
/// ```
///
/// ```compile_fail
/// let invalid = biodata::text!("\u{202e}unsafe");
/// ```
#[macro_export]
macro_rules! text {
    ($value:literal) => {{
        #[expect(
            clippy::panic,
            reason = "invalid literals fail const evaluation and cannot reach runtime"
        )]
        const VALIDATED: $crate::TextLiteral = match $crate::TextLiteral::new($value) {
            Ok(value) => value,
            Err(_) => panic!("invalid Text literal"),
        };
        $crate::Text::from(VALIDATED)
    }};
}
