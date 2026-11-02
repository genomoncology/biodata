//! Recognize deferred description families at their written markers.
use super::AlterationCode;
use crate::Code;
use crate::parse::{Input, Result, refuse, step};
use winnow::Parser;
use winnow::token::take;

#[expect(
    clippy::too_many_lines,
    reason = "preserve the reviewed parser layout and hot-path behavior"
)]
pub(super) fn unsupported(input: &mut Input<'_>) -> Result<()> {
    step(input)?;
    let text: &str = input;
    let lower = text.to_ascii_lowercase();
    let (subject, tail) = lower.split_once(' ').unwrap_or((&lower, ""));
    let marker = if subject.contains("mdr") && tail.starts_with("(minimally ") {
        Some((AlterationCode::DescriptionUnsupported, subject))
    } else if lower.starts_with("super enhancer at ") && lower.ends_with(" gene locus") {
        Some((AlterationCode::EventUnsupported, "super enhancer"))
    } else if let Some(marker) = [
        "whole exon ",
        "alternative transcription site",
        "poison exon ",
    ]
    .into_iter()
    .find(|marker| tail.starts_with(marker))
    {
        Some((AlterationCode::EventUnsupported, marker.trim_end()))
    } else if tail.starts_with("rs") && tail.as_bytes().get(2).is_some_and(u8::is_ascii_digit) {
        Some((AlterationCode::GenotypeUnsupported, "rs"))
    } else if tail.starts_with('v') && tail.contains("/v") {
        Some((
            AlterationCode::GenotypeUnsupported,
            tail.split('/').next().unwrap_or(tail),
        ))
    } else if (lower.contains(" score ") || lower.contains(" risk "))
        && ![
            "iwg-mrt",
            "nccn",
            "who",
            "mds",
            "high or",
            "intermediate or",
        ]
        .into_iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        Some((AlterationCode::InstrumentUnsupported, subject))
    } else if lower.ends_with(" subtype") {
        Some((AlterationCode::SubtypeUnsupported, "subtype"))
    } else if lower.contains(" risk ")
        && [
            "iwg-mrt",
            "nccn",
            "who",
            "mds",
            "high or",
            "intermediate or",
        ]
        .into_iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        Some((AlterationCode::RiskClassUnsupported, "risk"))
    } else if subject
        .split_once('.')
        .is_some_and(|(_, suffix)| suffix.bytes().all(|b| b.is_ascii_digit()))
        || ["antigen", "target expression", "molecular signature "]
            .into_iter()
            .any(|prefix| tail.starts_with(prefix))
        || (tail.is_empty()
            && ["ha-", "iamp"]
                .into_iter()
                .any(|prefix| subject.starts_with(prefix)))
        || lower == "globo h"
        || [
            "carcinoembryonic antigen ",
            "cellular tumor antigen ",
            "cpg island ",
            "ganglioside gd",
            "indian hedgehog ",
            "melanoma-overexpressed ",
            "nuclear epidermal ",
            "proheparin-binding ",
            "dickkopf-related ",
            "lewis y ",
            "strata immune ",
        ]
        .into_iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        Some((AlterationCode::SubjectUnsupported, subject))
    } else {
        None
    };
    if let Some((code, marker)) = marker {
        let start = lower
            .split_once(marker)
            .map_or(0, |(prefix, _)| prefix.chars().count());
        take(start).parse_next(input)?;
        return refuse(Code::Alteration(code), take(marker.chars().count())).parse_next(input);
    }
    Ok(())
}
