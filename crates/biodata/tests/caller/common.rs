#![deny(unexpected_cfgs)]
use std::{fmt, str::FromStr};
values::canonical_enum! {
    /// A vocabulary declared in a caller with no feature flags.
    #[repr(u8)]
    #[cfg_attr(feature = "schema", schemars(extend("x-caller" = true)))]
    CallerState, WireEnumUnknown {
        /// A declared ready state.
        Ready => "ready",
        /// A declared waiting state.
        Waiting => "waiting"
    }
}
#[derive(Debug, PartialEq)]
struct CallerText(String);
impl FromStr for CallerText {
    type Err = values::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if !value.is_empty() && value.bytes().all(|c| c.is_ascii_uppercase()) {
            Ok(Self(value.to_owned()))
        } else { Err(values::Error::TextEmpty) }
    }
}
impl fmt::Display for CallerText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}
values::text_value!(CallerText, "CallerText", "^[A-Z]+$", "Checked caller text.");
values::wire_enum! {
    /// A closed caller vocabulary.
    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    #[derive(PartialOrd, Ord)]
    CallerWire { Ready => "ready", Waiting => "waiting" }
}
values::wire_enum! { described
    /// A closed caller vocabulary with descriptions.
    CallerDescribed {
        /// A ready wire state.
        Ready => "ready",
        /// A waiting wire state.
        Waiting => "waiting"
    }
}
values::native_enum! {
    /// A caller vocabulary that preserves unknown values.
    CallerNative { Ready => "ready" }
}
values::checked_id!(CallerId, TextEmpty, |value: &str| value == "ID", "^ID$", "A checked caller ID.");

fn check_basic() {
    assert_eq!(values::text!("  ready  ").as_str(), "ready");
    assert!(CallerWire::Ready < CallerWire::Waiting);
    assert_eq!("ready".parse::<CallerWire>().unwrap().as_str(), "ready");
    assert_eq!("waiting".parse::<CallerDescribed>().unwrap().to_string(), "waiting");
    assert_eq!(CallerNative::from("new value".to_owned()).as_str(), "new value");
    assert!(CallerNative::from("new value".to_owned()).is_unknown());
    assert_eq!(CallerId::new("ID").unwrap().as_str(), "ID");
    let protein = values::UniProtIsoform::new("P04637-2").unwrap();
    assert_eq!(protein.to_string(), protein.as_str());
    assert_eq!("P04637-2".parse::<values::UniProtIsoform>().unwrap(), protein);
    assert_eq!("P04637-0".parse::<values::UniProtIsoform>().unwrap_err(), values::Error::UniProtIsoformInvalid);
    assert_eq!(values::Strand::try_from("+".to_owned()).unwrap().to_string(), "+");
    assert_eq!(<&'static str>::from(values::RefSeqPrefix::Nm), "NM_");
    assert_eq!(u32::from(values::Position1::try_from(3).unwrap()), 3);
    assert_eq!("ACDX".parse::<values::AminoAcids>().unwrap().to_string(), "ACDX");
    let date = values::PartialDate::new(2024, Some(2), Some(29)).unwrap();
    fn copy_hash<T: Copy + std::hash::Hash>(_: T) {}
    copy_hash(date);
    assert_eq!((date.year(), date.month(), date.day()), (2024, Some(2), Some(29)));
    assert_eq!(date.to_string(), "2024-02-29");
}
