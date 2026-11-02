//! Public reference value contracts.
use biodata::{Build, Chromosome, Strand};

#[test]
fn primary_reference_values_parse_and_preserve_spelling() {
    for (text, build) in [("GRCh37", Build::GRCh37), ("GRCh38", Build::GRCh38)] {
        assert_eq!(text.parse::<Build>().unwrap(), build);
        assert_eq!(build.to_string(), text);
    }
    assert_eq!(
        "hg19".parse::<Build>().unwrap_err().to_string(),
        "coord.build.unknown"
    );
    for text in ["chr1", "X", "chrY", "MT", "chrM"] {
        let contig = biodata::parse_chromosome(text).unwrap();
        for build in [Build::GRCh37, Build::GRCh38] {
            assert_eq!(
                Chromosome::from_accession(contig.accession(build), build).unwrap(),
                contig
            );
        }
    }
    assert!(biodata::parse_chromosome("chr26").is_err());
    assert_eq!(Strand::Plus as i8, 1);
    assert_eq!(Strand::Minus as i8, -1);
}

#[test]
fn shared_reference_values_refuse_malformed_complete_text() {
    for text in ["", "NC_7.14", "NM_004333", "NC_000007.14 extra"] {
        assert!(text.parse::<biodata::Accession>().is_err(), "{text}");
    }
    for text in ["", "1ABC", "A B", "A!B"] {
        assert!(text.parse::<biodata::GeneSymbol>().is_err(), "{text}");
    }
    assert_eq!(
        "NC_000007.14"
            .parse::<biodata::Accession>()
            .unwrap()
            .to_string(),
        "NC_000007.14"
    );
    assert_eq!(
        "BRCA1".parse::<biodata::GeneSymbol>().unwrap().as_str(),
        "BRCA1"
    );
    assert!(Chromosome::new(0).is_none());
    assert!(Chromosome::new(26).is_none());
    assert_eq!(Strand::from_sign(1), Some(Strand::Plus));
    assert_eq!(Strand::from_sign(-1), Some(Strand::Minus));
    assert_eq!(Strand::from_sign(0), None);
}

#[cfg(feature = "serde")]
#[test]
fn shared_reference_values_keep_their_serialized_spelling() {
    let accession = "NC_000007.14".parse::<biodata::Accession>().unwrap();
    assert_eq!(
        serde_json::to_string(&accession).unwrap(),
        "\"NC_000007.14\""
    );
    assert_eq!(
        serde_json::from_str::<biodata::Accession>("\"NC_000007.14\"").unwrap(),
        accession
    );
    let symbol = "BRCA1".parse::<biodata::GeneSymbol>().unwrap();
    assert_eq!(serde_json::to_string(&symbol).unwrap(), "\"BRCA1\"");
    assert_eq!(
        serde_json::from_str::<biodata::GeneSymbol>("\"BRCA1\"")
            .unwrap()
            .as_str(),
        symbol.as_str()
    );
    assert_eq!(
        serde_json::from_str::<Strand>(&serde_json::to_string(&Strand::Minus).unwrap()).unwrap(),
        Strand::Minus
    );
}

#[test]
fn standalone_reference_values_keep_the_public_input_budget() {
    let text = "A".repeat(biodata::MAX_LEN + 1);
    assert_eq!(
        text.parse::<biodata::Accession>().unwrap_err().code.name(),
        "input.length.limit",
        "accessions enforce the public input limit"
    );
    assert_eq!(
        text.parse::<biodata::GeneSymbol>().unwrap_err().code(),
        "gene.symbol.invalid",
        "gene symbols enforce the public input limit"
    );
}

#[test]
fn reference_identifiers_keep_distinct_sorted_keys() {
    let identifiers: Vec<biodata::Accession> = [
        "NC_000007.14",
        "NM_004333.6",
        "NM_004333.4",
        "ENST00000288602.11",
        "ENST00000288602.12",
        "ENSG00000157764.1",
        "LRG_199",
        "LRG_199t1",
        "LRG_199t2",
        "LRG_199p1",
        "LRG_200",
    ]
    .into_iter()
    .map(|text| text.parse().unwrap())
    .collect();
    let keys: std::collections::BTreeSet<_> = identifiers.iter().copied().collect();
    assert_eq!(keys.len(), identifiers.len());
    for accession in identifiers {
        assert!(keys.contains(&accession));
    }
}
