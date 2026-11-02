//! Typed conventional chromosome descriptions in ISCN 2016.
mod grammar;
mod observation;
mod render;
mod structure;
mod types;

pub(crate) use grammar::{chromosome, event, event_start};
pub(crate) use render::parts as render_parts;
pub(crate) use structure::{Parts, parts};
pub use types::*;

use crate::Parsed;
use crate::parse::run;

/// Parses a conventional karyotype without interpreting its findings.
pub fn parse(text: &str) -> Parsed<Karyotype> {
    run(text, grammar::iscn)
}

crate::table::codes! {
/// Stable refusals raised by the ISCN grammar.
pub enum IscnCode {
    KaryotypeInvalid => "iscn.karyotype.invalid",
    CountInvalid => "iscn.count.invalid",
    ChromosomeInvalid => "iscn.chromosome.invalid",
    BreakpointInvalid => "iscn.breakpoint.invalid",
    EventInvalid => "iscn.event.invalid",
    CellsInvalid => "iscn.cells.invalid",
    ShortFormUnsupported => "iscn.short_form.unsupported",
    DetailedUnsupported => "iscn.detailed.unsupported",
    FishUnsupported => "iscn.fish.unsupported",
    ArrayUnsupported => "iscn.array.unsupported",
    RsaUnsupported => "iscn.rsa.unsupported",
    SequenceUnsupported => "iscn.sequence.unsupported",
    MeioticUnsupported => "iscn.meiotic.unsupported",
    UpdUnsupported => "iscn.upd.unsupported",
    NormalVariantUnsupported => "iscn.normal_variant.unsupported",
    CompoundUnsupported => "iscn.compound.unsupported",
    EventUnsupported => "iscn.event.unsupported",
    CompositeUnsupported => "iscn.composite.unsupported",
    MixtureUnsupported => "iscn.mixture.unsupported",
    ChromosomeUnsupported => "iscn.chromosome.unsupported",
}
}
