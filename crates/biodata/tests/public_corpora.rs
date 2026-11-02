//! Public HGVS corpora exercise the public parser without network access.
#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unwrap_used,
    clippy::print_stdout,
    reason = "the fixture test validates fixed column counts, then reports parser outcomes"
)]

use biodata::{Outcome, hgvs};
use flate2::read::GzDecoder;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};

const BIOCOMMONS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/corpora/biocommons-hgvs/1.5.4/"
);
const VRS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/corpora/ga4gh-vrs/2.1.1/"
);
const YEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/corpora/yen-2017/");
const REVIEWED: &str = include_str!("corpora/reviewed-differences.tsv");

fn reviewed(corpus: &str) -> BTreeMap<String, String> {
    let mut rows = BTreeMap::new();
    for line in REVIEWED.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 8, "reviewed difference row: {line}");
        if fields[0] != corpus {
            continue;
        }
        let key = match corpus {
            "grammar" => format!("{}\t{}", fields[1], fields[4]),
            "clinvar" => format!("{}\t{}\t{}", fields[1], fields[2], fields[4]),
            "yen" => format!("{}\t{}\t{}\t{}", fields[1], fields[2], fields[3], fields[4]),
            _ => panic!("unknown corpus: {corpus}"),
        };
        assert!(
            !fields[6].is_empty() && fields[7].starts_with("https://"),
            "each reviewed difference names its reason and public source"
        );
        assert!(rows.insert(key, fields[5].to_owned()).is_none(), "{line}");
    }
    rows
}

fn check_reviewed(rows: &BTreeMap<String, String>, key: &str, result: &str) -> bool {
    match rows.get(key) {
        Some(expected) => {
            assert_eq!(result, expected, "reviewed difference: {key}");
            true
        }
        None => {
            assert_eq!(result, "parsed", "unreviewed difference: {key}");
            false
        }
    }
}

fn outcome(input: &str) -> String {
    match hgvs::parse(input).outcome {
        Outcome::Parsed(_) => "parsed".into(),
        Outcome::Unsupported(r) => format!("unsupported:{}", r.code.name()),
        Outcome::Invalid(r) => format!("invalid:{}", r.code.name()),
        Outcome::LimitExceeded(r) => format!("limit_exceeded:{}", r.code.name()),
    }
}

fn retain(name: &str, rows: &[String]) {
    let Some(root) = std::env::var_os("PUBLIC_HGVS_OBSERVATIONS") else {
        return;
    };
    std::fs::create_dir_all(&root).unwrap();
    let path = std::path::Path::new(&root).join(format!("{name}.tsv"));
    let mut file = std::fs::File::create(path).unwrap();
    for row in rows {
        writeln!(file, "{row}").unwrap();
    }
}

#[test]
fn biocommons_variant_grammar_has_no_unobserved_cases() {
    let source = std::fs::read_to_string(format!("{BIOCOMMONS}grammar_test.tsv")).unwrap();
    let mut count = 0;
    let mut fragments = 0;
    let mut variants = 0;
    let mut outcomes = BTreeMap::new();
    let mut differences = Vec::new();
    let mut observations = Vec::new();
    let reviewed = reviewed("grammar");
    let mut reviewed_seen = 0;
    for (line, row) in source.lines().enumerate().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        if fields.len() < 4 {
            continue;
        }
        if !fields[0].ends_with("_variant") {
            fragments += 1;
            continue;
        }
        variants += 1;
        assert_eq!(fields[2], "True", "grammar line {}", line + 1);
        for input in fields[1].split('|') {
            let result = outcome(input);
            reviewed_seen += usize::from(check_reviewed(
                &reviewed,
                &format!("{}\t{input}", line + 1),
                &result,
            ));
            observations.push(format!("{}\t{input}\t{result}", line + 1));
            *outcomes.entry(result.clone()).or_insert(0usize) += 1;
            if result != "parsed" {
                differences.push(format!("{}\t{input}\t{result}", line + 1));
            }
            count += 1;
        }
    }
    println!("biocommons grammar variants: {count}, {outcomes:?}");
    for row in differences.iter().take(10) {
        println!("grammar {row}");
    }
    retain("grammar", &observations);
    assert_eq!(count, 26);
    assert_eq!((variants, fragments), (16, 258));
    assert_eq!(reviewed_seen, reviewed.len());
}

#[test]
fn biocommons_placeholders_test_supported_change_bodies() {
    let reviewed = reviewed("grammar");
    let mut count = 0;
    for (key, _) in reviewed {
        let (line, input) = key.split_once('\t').unwrap();
        let Some(body) = input.strip_prefix("AC_01234.5:") else {
            continue;
        };
        if body.starts_with("r.") {
            continue;
        }
        let reference = match body.chars().next().unwrap() {
            'c' => "NM_000001.1",
            'g' => "NC_000001.10",
            'm' => "NC_012920.1",
            'n' => "NR_000001.1",
            'p' => "NP_000001.1",
            molecule => panic!("unexpected molecule {molecule}: grammar line {line}"),
        };
        let paired = format!("{reference}:{body}");
        let Outcome::Parsed(parsed) = hgvs::parse(&paired).outcome else {
            panic!("grammar line {line}: change body failed with {paired}");
        };
        let Outcome::Parsed(bare) = hgvs::parse(body).outcome else {
            panic!("grammar line {line}: bare change body failed: {body}");
        };
        assert_eq!(parsed.change, bare.change, "grammar line {line}");
        count += 1;
    }
    assert_eq!(count, 13);
}

#[test]
fn biocommons_clinvar_has_no_unobserved_cases() {
    let file = std::fs::File::open(format!("{BIOCOMMONS}clinvar.gz")).unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    let mut records = 0;
    let mut expressions = 0;
    let mut outcomes = BTreeMap::new();
    let mut differences = Vec::new();
    let mut observations = Vec::new();
    let reviewed = reviewed("clinvar");
    let mut reviewed_seen = 0;
    for row in reader.lines() {
        let row = row.unwrap();
        if row.starts_with('#') || row.starts_with("gene\t") {
            continue;
        }
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 3, "ClinVar record {records}");
        records += 1;
        for input in fields[2].split_ascii_whitespace() {
            let result = outcome(input);
            reviewed_seen += usize::from(check_reviewed(
                &reviewed,
                &format!("{}\t{}\t{input}", records, fields[1]),
                &result,
            ));
            observations.push(format!("{}\t{}\t{input}\t{result}", records, fields[1]));
            *outcomes.entry(result.clone()).or_insert(0usize) += 1;
            if result != "parsed" {
                differences.push(format!("{}\t{}\t{input}\t{result}", records, fields[1]));
            }
            expressions += 1;
        }
    }
    println!("biocommons ClinVar: {records} records, {expressions} expressions, {outcomes:?}");
    for row in differences.iter().take(10) {
        println!("ClinVar {row}");
    }
    retain("clinvar", &observations);
    assert_eq!(records, 7498);
    assert_eq!(expressions, 25815);
    assert_eq!(reviewed_seen, reviewed.len());
}

#[test]
fn biocommons_reject_fixture_has_no_active_cases() {
    let source = std::fs::read_to_string(format!("{BIOCOMMONS}reject")).unwrap();
    let mut comments = 0;
    let mut active = 0;
    for row in source.lines().filter(|row| !row.is_empty()) {
        if row.starts_with('#') {
            comments += 1;
        } else {
            active += 1;
        }
    }
    assert_eq!((active, comments), (0, 1));
}

#[test]
fn vrs_named_hgvs_expressions_have_no_unobserved_cases() {
    let source = std::fs::read_to_string(format!("{VRS}models.json")).unwrap();
    let models: serde_json::Value = serde_json::from_str(&source).unwrap();
    let mut count = 0;
    let mut structural = 0;
    let mut observations = Vec::new();
    for group in models.as_object().unwrap().values() {
        for fixture in group.as_array().unwrap() {
            structural += 1;
            let Some(name) = fixture["name"].as_str() else {
                continue;
            };
            let Some(input) = name.split_ascii_whitespace().next() else {
                continue;
            };
            if !input.contains(":c.") {
                continue;
            }
            let result = outcome(input);
            observations.push(format!("{name}\t{result}"));
            assert_eq!(result, "parsed", "VRS HGVS expression: {name}");
            count += 1;
        }
    }
    println!("VRS named HGVS expressions: {count}");
    retain("vrs", &observations);
    assert_eq!(count, 4);
    assert_eq!(structural, 23);
    let source = std::fs::read_to_string(format!("{VRS}functions.json")).unwrap();
    let functions: serde_json::Value = serde_json::from_str(&source).unwrap();
    let digest = functions["sha512t24u"].as_array().unwrap();
    assert_eq!(digest.len(), 2);
    assert!(
        digest
            .iter()
            .all(|row| row["in"]["blob"].is_string() && row["out"].is_string())
    );
}

#[test]
fn yen_curated_expressions_have_no_unobserved_cases() {
    let source = std::fs::read_to_string(format!("{YEN}variants.tsv")).unwrap();
    let mut rows = 0;
    let mut expressions = 0;
    let mut outcomes = BTreeMap::new();
    let mut differences = Vec::new();
    let mut observations = Vec::new();
    let reviewed = reviewed("yen");
    let mut reviewed_seen = 0;
    for row in source.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 9, "Yen row {rows}");
        rows += 1;
        for (kind, input) in [
            ("transcript", fields[6]),
            ("genomic", fields[7]),
            ("protein", fields[8]),
        ] {
            if input == "-" {
                continue;
            }
            let result = outcome(input);
            reviewed_seen += usize::from(check_reviewed(
                &reviewed,
                &format!("{}\t{}\t{kind}\t{input}", fields[0], fields[1]),
                &result,
            ));
            observations.push(format!(
                "{}\t{}\t{kind}\t{input}\t{result}",
                fields[0], fields[1]
            ));
            *outcomes.entry(result.clone()).or_insert(0usize) += 1;
            if result != "parsed" {
                differences.push(format!(
                    "{}\t{}\t{kind}\t{input}\t{result}",
                    fields[0], fields[1]
                ));
            }
            expressions += 1;
        }
    }
    println!("Yen curated: {rows} rows, {expressions} expressions, {outcomes:?}");
    for row in differences.iter().take(10) {
        println!("Yen {row}");
    }
    retain("yen", &observations);
    assert_eq!(rows, 126);
    assert_eq!(expressions, 342);
    assert_eq!(reviewed_seen, reviewed.len());
}

#[test]
fn yen_historical_coding_substitution_rates_match_the_paper() {
    let source = std::fs::read_to_string(format!("{YEN}historical-substitutions.tsv")).unwrap();
    let mut rows = 0;
    let mut snpeff_assessed = 0;
    let mut snpeff_correct = 0;
    let mut vep_assessed = 0;
    let mut vep_correct = 0;
    for row in source.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 10, "historical row {rows}");
        rows += 1;
        if fields[6] == "yes" {
            snpeff_assessed += 1;
            snpeff_correct += usize::from(matches!(fields[7], "yes" | "yes_m"));
        }
        if fields[8] == "yes" {
            vep_assessed += 1;
            vep_correct += usize::from(matches!(fields[9], "yes" | "yes_m"));
        }
    }
    assert_eq!(rows, 62);
    assert_eq!((snpeff_correct, snpeff_assessed), (53, 59));
    assert_eq!((vep_correct, vep_assessed), (51, 62));
}
