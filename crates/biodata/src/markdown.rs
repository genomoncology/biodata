use crate::{Error, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Nonempty source Markdown retained as record data.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Markdown(Box<str>);
impl Markdown {
    /// Preserve source formatting, escape unsafe controls and enforce the text cap.
    pub fn new(value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::TextEmpty);
        }
        let mut safe = String::new();
        for ch in value.chars() {
            if crate::text_control::unsafe_control(ch) && !matches!(ch, '\n' | '\r' | '\t') {
                use std::fmt::Write;
                write!(&mut safe, "\\u{{{:04x}}}", u32::from(ch))
                    .map_err(|_| Error::TextControl)?;
            } else {
                safe.push(ch);
            }
            if safe.len() > Text::MAX_CHARS * 4 {
                return Err(Error::TextTooLong);
            }
        }
        if safe.chars().count() > Text::MAX_CHARS {
            return Err(Error::TextTooLong);
        }
        Ok(Self(safe.into()))
    }
    /// Borrow the source Markdown.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Markdown {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Markdown".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string", "format": "markdown", "minLength": 1,
            "maxLength": Text::MAX_CHARS,
            "pattern": r"^(?![\s\S]*[\x00-\x08\x0b\x0c\x0e-\x1f\x7f-\x9f\u061c\u200e\u200f\u202a-\u202e\u2066-\u2069])",
            "description": "Nonempty source Markdown retains LF, CR and tab formatting; unsafe controls use visible escapes. \
                The parser additionally rejects whitespace-only values."
        })
    }
}

impl TryFrom<String> for Markdown {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        Self::new(&value)
    }
}
