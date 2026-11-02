//! Transcript references serialize as one string with either accession form.
use schemars::{JsonSchema, Schema, SchemaGenerator};
use std::borrow::Cow;

fn schema() -> Schema {
    schemars::json_schema!({
        "description": "A transcript reference in versioned Accession or unversioned AccessionBase form.",
        "type": "string",
        "pattern": format!("({}|{})", crate::Accession::pattern(), crate::AccessionBase::pattern())
    })
}

impl JsonSchema for crate::TranscriptReference {
    fn schema_name() -> Cow<'static, str> {
        "TranscriptReference".into()
    }
    fn schema_id() -> Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        schema()
    }
}

impl JsonSchema for crate::TranscriptId {
    fn schema_name() -> Cow<'static, str> {
        "TranscriptId".into()
    }
    fn schema_id() -> Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        schema()
    }
}
