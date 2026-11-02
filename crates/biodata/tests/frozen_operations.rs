//! Frozen additive operations remain usable through the public crate.
use biodata::{
    Accession, AcmgAmp2015Code, AminoAcids, Build, CoordinateError, EnsemblFeature, Error,
    Expression, ExpressionSyntax, ManeStatus, Position1, RefSeqPrefix, Strand, Text,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

fn hash(value: &impl Hash) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
}

fn canonical<T>(values: &[T]) -> Result<(), String>
where
    T: Copy + Ord + Hash + std::fmt::Display + std::fmt::Debug + FromStr + TryFrom<String>,
    <T as FromStr>::Err: std::fmt::Debug,
    <T as TryFrom<String>>::Error: std::fmt::Debug,
{
    assert!(
        values.windows(2).all(|pair| pair.first() < pair.get(1)),
        "ordering must follow the frozen declaration order"
    );
    for &value in values {
        let text = value.to_string();
        let parsed = text.parse::<T>().map_err(|error| format!("{error:?}"))?;
        assert_eq!(
            parsed, value,
            "canonical display must parse to the same value"
        );
        assert_eq!(
            T::try_from(text.clone()).map_err(|error| format!("{error:?}"))?,
            value,
            "owned string construction must use the canonical parser"
        );
        assert_eq!(
            hash(&value),
            hash(&parsed),
            "equal values must hash equally"
        );
        assert!(
            format!(" {text}").parse::<T>().is_err(),
            "canonical parsers must refuse leading whitespace"
        );
    }
    Ok(())
}

#[test]
fn canonical_operations_preserve_declared_values_and_refusals() -> Result<(), String> {
    canonical(Build::ALL)?;
    canonical(Strand::ALL)?;
    canonical(ManeStatus::ALL)?;
    canonical(ExpressionSyntax::ALL)?;
    canonical(RefSeqPrefix::ALL)?;
    canonical(EnsemblFeature::ALL)?;
    assert_eq!(<&'static str>::from(Strand::Minus), "-");
    assert_eq!("bad".parse::<Strand>().unwrap_err(), Error::StrandUnknown);
    assert_eq!(
        "bad".parse::<ManeStatus>().unwrap_err(),
        Error::ManeStatusUnknown
    );
    assert_eq!(
        "bad".parse::<ExpressionSyntax>().unwrap_err(),
        Error::ExpressionSyntaxUnknown
    );
    assert_eq!(
        "bad".parse::<RefSeqPrefix>().unwrap_err(),
        Error::AccessionInvalid
    );
    assert_eq!(AcmgAmp2015Code::Pathogenic.as_str(), "pathogenic");
    assert_eq!(
        "pathogenic".parse::<AcmgAmp2015Code>().unwrap().to_string(),
        "pathogenic"
    );
    assert_eq!(
        "bad".parse::<AcmgAmp2015Code>().unwrap_err(),
        Error::WireEnumUnknown
    );
    Ok(())
}

#[test]
fn hash_and_conversions_follow_the_existing_identity_and_validators() {
    let first: Accession = "NM_000001.1".parse().unwrap();
    let second: Accession = "NM_000001.2".parse().unwrap();
    assert_ne!(first, second);
    assert_eq!(first.base(), second.base());
    assert_eq!(hash(&first.base()), hash(&second.base()));
    let position = Position1::try_from(3).unwrap();
    assert_eq!(u32::from(position), 3);
    assert_eq!(
        Position1::try_from(0).unwrap_err(),
        CoordinateError::ZeroPosition
    );
    let residues: AminoAcids = "ACDX".parse().unwrap();
    assert_eq!(residues.to_string(), "ACDX");
    assert_eq!(
        "!".parse::<AminoAcids>().unwrap_err(),
        Error::ProteinResidueInvalid
    );
    assert!(residues < "ACDY".parse().unwrap());
    assert_eq!(hash(&residues), hash(&AminoAcids::new("ACDX").unwrap()));
    let expression = Expression::new(
        ExpressionSyntax::HgvsC,
        Text::new("NM_000001.1:c.3A>G").unwrap(),
        Some(Text::new("21.1").unwrap()),
    )
    .unwrap();
    assert_eq!(expression.syntax_version().map(Text::as_str), Some("21.1"));
    assert_eq!(hash(&expression), hash(&expression.clone()));
}

#[cfg(feature = "serde")]
#[test]
fn prefix_deserialization_preserves_current_wire_context() {
    assert_eq!(serde_json::to_string(&RefSeqPrefix::Nm).unwrap(), "\"nm\"");
    assert_eq!(
        serde_json::to_string(&EnsemblFeature::Gene).unwrap(),
        "\"gene\""
    );
    assert_eq!(
        serde_json::from_str::<RefSeqPrefix>("\"nm\"").unwrap(),
        RefSeqPrefix::Nm
    );
    assert_eq!(
        serde_json::from_str::<EnsemblFeature>("\"gene\"").unwrap(),
        EnsemblFeature::Gene
    );
    assert_eq!("NM_".parse::<RefSeqPrefix>().unwrap(), RefSeqPrefix::Nm);
    assert_eq!(
        "ENSG".parse::<EnsemblFeature>().unwrap(),
        EnsemblFeature::Gene
    );
    assert!("nm".parse::<RefSeqPrefix>().is_err());
    assert!(serde_json::from_str::<RefSeqPrefix>("\"NM_\"").is_err());
    assert!(serde_json::from_str::<EnsemblFeature>("\"ENSG\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn clinvar_display_preserves_publisher_labels_and_wire_codes() {
    let review: biodata::ReviewStatus = "practice guideline".parse().unwrap();
    assert_eq!(review.to_string(), "practice guideline");
    assert_eq!(serde_json::to_value(review).unwrap(), "practice_guideline");
    let origin: biodata::AlleleOrigin = "de novo".parse().unwrap();
    assert_eq!(origin.to_string(), "de novo");
    assert_eq!(serde_json::to_value(origin).unwrap(), "de_novo");
}
