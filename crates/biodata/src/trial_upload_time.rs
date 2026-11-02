//! ClinicalTrials.gov DateTimeMinutes retains native precision and no timezone.
use std::{fmt, str::FromStr};

/// A calendar-checked `yyyy-MM-dd'T'HH:mm` upload time with unstated timezone.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrialUploadTime(Box<str>);

impl TrialUploadTime {
    /// Borrow the unchanged native timestamp.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for TrialUploadTime {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let valid = value.split_once('T').is_some_and(|(date, time)| {
            date.len() == 10
                && date.parse::<crate::PartialDate>().is_ok()
                && matches!(time.as_bytes(), [h1, h2, b':', m1, m2]
                    if [h1, h2, m1, m2].iter().all(|b| b.is_ascii_digit())
                        && (h1 - b'0') * 10 + h2 - b'0' <= 23
                        && (m1 - b'0') * 10 + m2 - b'0' <= 59)
        });
        if valid {
            Ok(Self(value.into()))
        } else {
            Err(crate::Error::TrialUploadTimeInvalid)
        }
    }
}
impl fmt::Display for TrialUploadTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
crate::text_value!(
    TrialUploadTime,
    "TrialUploadTime",
    "^(?!0000)[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9](?![\\s\\S])",
    "Native ClinicalTrials.gov DateTimeMinutes without timezone. The parser additionally validates the calendar day, including leap years."
);
