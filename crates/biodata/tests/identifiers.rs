//! Public identifier parsing, rendering and serialization behavior.
#![allow(clippy::unwrap_used, reason = "outside-in literal parser assertions")]
use biodata::{
    CaId, ClinVarAlleleId, ClinVarVariationId, CosmicId, CosmicKind, HgncId, NcbiGeneId, OmimId,
    PaId, RcvId, RsId, ScvId, VariantId, VcvId, VrsId,
};
use std::{fmt::Debug, str::FromStr};

fn round_trip<T>(text: &str)
where
    T: FromStr + std::fmt::Display + PartialEq + Debug,
    T::Err: Debug,
{
    let value: T = text.parse().unwrap();
    assert_eq!(
        value.to_string(),
        text,
        "canonical identifiers preserve source spelling"
    );
    assert_eq!(
        value.to_string().parse::<T>().unwrap(),
        value,
        "canonical identifiers retain identity after parsing"
    );
}

#[test]
fn gene_identifiers_keep_their_namespaces() {
    round_trip::<HgncId>("HGNC:1097");
    round_trip::<HgncId>("HGNC:4294967295");
    round_trip::<NcbiGeneId>("673");
    round_trip::<NcbiGeneId>("4294967295");
    round_trip::<OmimId>("113705");
    for invalid in ["HGNC:0", "HGNC:01097", "1097", "HGNC:-1", "HGNC:4294967296"] {
        assert!(invalid.parse::<HgncId>().is_err(), "{invalid}");
    }
    for invalid in ["0", "01", "-1", "4294967296"] {
        assert!(invalid.parse::<NcbiGeneId>().is_err(), "{invalid}");
        assert!(invalid.parse::<OmimId>().is_err(), "{invalid}");
    }
}

#[test]
fn variant_identifiers_preserve_level_and_published_spelling() {
    round_trip::<RsId>("rs113488022");
    round_trip::<RsId>("rs18446744073709551615");
    round_trip::<CaId>("CA000434");
    round_trip::<CaId>("CA10594475");
    round_trip::<PaId>("PA123456");
    round_trip::<VcvId>("VCV000013961");
    round_trip::<VcvId>("VCV000013961.98");
    round_trip::<ClinVarVariationId>("clinvar:13961");
    round_trip::<CosmicId>("COSV55386424");
    round_trip::<CosmicId>("COSM476");
    round_trip::<CosmicId>("COSN123");
    for (text, kind) in [
        ("COSV55386424", CosmicKind::Cosv),
        ("COSM476", CosmicKind::Cosm),
        ("COSN123", CosmicKind::Cosn),
    ] {
        assert_eq!(text.parse::<CosmicId>().unwrap().kind(), kind);
    }
    round_trip::<VrsId>("ga4gh:VA.0123456789abcdefghijklmnopqrstuv");
    for text in [
        "rs113488022",
        "CA000434",
        "PA123456",
        "VCV000013961.98",
        "clinvar:13961",
        "COSV55386424",
        "ga4gh:VA.0123456789abcdefghijklmnopqrstuv",
    ] {
        round_trip::<VariantId>(text);
    }
    for invalid in [
        "rs0",
        "rs01",
        "rs18446744073709551616",
        "CA000000",
        "CA434",
        "CA0000434",
        "PA0",
        "PA01",
        "VCV13961",
        "VCV000013961.0",
        "VCV000013961.01",
        "VCV1000000000",
        "clinvar:0",
        "COSX123",
        "COSM0",
        "ga4gh:VA.short",
    ] {
        assert!(invalid.parse::<VariantId>().is_err(), "{invalid}");
    }
}

#[test]
fn attribution_and_allele_identifiers_do_not_parse_as_variant_ids() {
    round_trip::<RcvId>("RCV000013961.12");
    round_trip::<ScvId>("SCV000061234.3");
    round_trip::<ClinVarAlleleId>("12345");
    for text in ["RCV000013961.12", "SCV000061234.3", "12345"] {
        assert!(text.parse::<VariantId>().is_err(), "{text}");
    }
    for invalid in [
        "RCV13961",
        "RCV000013961.01",
        "RCV000000000",
        "RCV1000000000",
    ] {
        assert!(invalid.parse::<RcvId>().is_err(), "{invalid}");
    }
    for invalid in [
        "SCV61234",
        "SCV000061234.0",
        "SCV000000000",
        "SCV1000000000",
    ] {
        assert!(invalid.parse::<ScvId>().is_err(), "{invalid}");
    }
    for invalid in ["0", "01", "4294967296"] {
        assert!(invalid.parse::<ClinVarAlleleId>().is_err(), "{invalid}");
    }
}

#[cfg(feature = "serde")]
#[test]
fn serde_checks_the_same_canonical_text() {
    let value: VcvId = "VCV000013961.98".parse().unwrap();
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        "\"VCV000013961.98\""
    );
    assert_eq!(
        serde_json::from_str::<VcvId>("\"VCV000013961.98\"").unwrap(),
        value
    );
    assert!(serde_json::from_str::<VcvId>("\"VCV13961\"").is_err());
}
