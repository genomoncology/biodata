//! Every case row passes through the library: outcome, code, span, render and round trip.
#![allow(
    clippy::disallowed_methods,
    clippy::print_stdout,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "the test reads case files, prints agreement counts, and fails by panicking"
)]

#[path = "support/cases.rs"]
mod cases;
use biodata::Outcome;
use std::fmt::Display;

#[test]
fn every_public_case_row_passes() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cases/public");
    let mut rows = 0;
    for file in std::fs::read_dir(root).unwrap() {
        let path = file.unwrap().path();
        if path
            .extension()
            .is_none_or(|extension| extension != "jsonl")
        {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap();
        for (number, line) in std::fs::read_to_string(&path).unwrap().lines().enumerate() {
            let row: serde_json::Value = serde_json::from_str(line).unwrap();
            let at = format!("{name}:{}", number + 1);
            if name.starts_with("alteration-") {
                cases::check(&row, biodata::alteration::parse, &at);
            } else {
                assert!(name.starts_with("hgvs-"));
                cases::check(&row, biodata::hgvs::parse, &at);
            }
            rows += 1;
        }
    }
    assert!(rows > 0);
}

#[test]
fn debug_never_prints_the_source() {
    for input in ["NC_000007.14:g.140753336A>T", "NC_000007.14:g.140753336A>J"] {
        let debug = format!("{:?}", biodata::hgvs::parse(input));
        assert!(!debug.contains(input), "{debug}");
    }
}

#[test]
fn ranged_identity_with_stated_bases_has_a_named_compatibility_refusal() {
    for (input, marker) in [
        ("NC_000006.11:g.112508769_112508770TG=", "TG"),
        ("NM_002290.3:c.827_828CA=", "CA"),
        ("NC_000011.10:g.47349933_47349935AGG=", "AGG"),
        ("NC_000011.9:g.47371484_47371486AGG=", "AGG"),
        ("g.(21_22CA=)", "CA"),
    ] {
        let Outcome::Unsupported(refusal) = biodata::hgvs::parse(input).outcome else {
            panic!("{input}: expected a compatibility refusal");
        };
        assert_eq!(
            refusal.code.name(),
            "hgvs.range_identity.unsupported",
            "{input}"
        );
        let span = refusal.span.unwrap();
        assert_eq!(&input[span.start as usize..span.end as usize], marker);
    }
    for input in ["g.21A=", "g.21_22="] {
        assert!(matches!(
            biodata::hgvs::parse(input).outcome,
            Outcome::Parsed(_)
        ));
    }
    assert!(matches!(
        biodata::hgvs::parse("g.21_22CA=)").outcome,
        Outcome::Invalid(_)
    ));
}

#[test]
fn nucleotide_values_construct_sequences_and_render_without_a_wrapper() {
    use biodata::Base;
    use biodata::hgvs::{Change, Nucleotides};

    assert!(Nucleotides::new(Vec::<Base>::new()).is_none());
    let bases = Nucleotides::new(vec![Base::A, Base::C]).unwrap();
    assert_eq!(bases.dna().unwrap(), &[Base::A, Base::C]);
    assert_eq!(bases.to_string(), "AC");
    assert_writer_refusal(&bases);
    let Outcome::Parsed(value) = biodata::hgvs::parse("c.(7_8delAC)").outcome else {
        panic!("the predicted deletion must parse");
    };
    let Change::Nucleotide(change) = value.change else {
        panic!("the predicted deletion must be a nucleotide change");
    };
    assert_eq!(change.to_string(), "c.7_8delAC");
}

#[test]
fn notation_display_propagates_a_writer_refusal() {
    for input in [
        "c.7+1_7+2delAC",
        "c.(7_8insAC)",
        "r.0",
        "c.7_8dupAC",
        "c.7delinsAC",
        "p.Ala7_Gly8insSer",
        "NC_000007.14:g.7A>T",
        "LRG_199t1:c.7del",
        "g.(7_8)inv",
        "g.7=",
        "g.7?",
        "c.7_8delACinsG",
        "p.Ala7_Gly8del",
        "p.Ala7_Gly8delinsSer",
        "p.Ala7SerfsTer8",
        "p.Met1Valext-5",
        "p.Ter8GlnextTer5",
        "p.Ala7?",
    ] {
        let Outcome::Parsed(value) = biodata::hgvs::parse(input).outcome else {
            panic!("the nucleotide edit must parse");
        };
        assert_writer_refusal(&value);
    }
    for input in [
        "BRAF V600E (somatic)",
        "GENE Exon 2 Activating Mutation",
        "BRAF c.1799T>A V600E",
        "FLT3 ITD (Y599_E604dup)",
        "CYP2D6 *1A/*2.003",
        "HLA-A*02:01P",
        "PD-L1 Expression (TPS >= 0.05%)",
        "PD-L1 Expression (TPS = 0.000000000000000000001%)",
        "PD-L1 Expression (TPS = 1-50%)",
        "HER2 Expression (IHC >= 2+)",
        "MSI-High",
        "StrucVarCount 11-17",
        "StrucLoss(13)(q21q31)",
    ] {
        let Outcome::Parsed(value) = biodata::alteration::parse(input).outcome else {
            panic!("the alteration must parse");
        };
        assert_writer_refusal(&value);
    }
}

fn assert_writer_refusal(value: &impl Display) {
    use std::fmt::{self, Write};
    struct Writer {
        remaining: usize,
    }
    impl Write for Writer {
        fn write_str(&mut self, text: &str) -> fmt::Result {
            self.remaining = self.remaining.checked_sub(text.len()).ok_or(fmt::Error)?;
            Ok(())
        }
    }

    let length = value.to_string().len();
    for remaining in 0..length {
        assert!(
            write!(&mut Writer { remaining }, "{value}").is_err(),
            "canonical writers propagate an exhausted output sink"
        );
    }
}

#[test]
fn gene_symbols_compare_without_case_and_keep_spelling() {
    let Outcome::Parsed(upper) = biodata::alteration::parse("BRAF V600E").outcome else {
        panic!("valid alteration");
    };
    let Outcome::Parsed(lower) = biodata::alteration::parse("braf V600E").outcome else {
        panic!("valid alteration");
    };
    assert_eq!(upper, lower);
    assert_eq!(lower.to_string(), "braf V600E");
}

/// A serializer's output error must survive every field in the public envelope.
#[cfg(feature = "serde")]
#[test]
fn parsed_json_propagates_a_writer_refusal() {
    struct Writer(usize);
    impl std::io::Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let count = self.0.min(bytes.len());
            if count == 0 {
                return Err(std::io::Error::other("output refused"));
            }
            self.0 -= count;
            Ok(count)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    for input in ["c.7A>T", "c.7A>J", "c.[7A>T]"] {
        let parsed = biodata::hgvs::parse(input);
        let bytes = serde_json::to_vec(&parsed).unwrap();
        for remaining in 0..bytes.len() {
            let error = serde_json::to_writer(Writer(remaining), &parsed).unwrap_err();
            assert!(error.is_io(), "{input}: {remaining}: {error}");
        }
    }
}

/// Deferred markers retain byte spans when surrounding text contains Unicode.
#[test]
fn deferred_markers_keep_byte_spans_after_unicode() {
    for (input, code, marker) in [
        (
            "GENE α subtype",
            "alteration.subtype.unsupported",
            "subtype",
        ),
        ("α rs1", "alteration.genotype.unsupported", "rs"),
        ("GENE vα/vβ", "alteration.genotype.unsupported", "vα"),
    ] {
        let Outcome::Unsupported(refusal) = biodata::alteration::parse(input).outcome else {
            panic!("{input}: expected a deferred-marker refusal");
        };
        assert_eq!(refusal.code.name(), code, "{input}");
        let start = input.find(marker).unwrap() as u32;
        assert_eq!(
            refusal.span,
            Some(biodata::Span {
                start,
                end: start + marker.len() as u32,
            }),
            "{input}"
        );
    }
}
