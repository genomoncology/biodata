//! Feature protein-types: neutral native records and checked standalone protein inputs.
use biodata::{
    AminoAcids, Bound, ComplexPortalId, Error, InterProId, ProteinComplexHeader,
    ProteinComplexMember, ProteinCoordinateConvention, ProteinDomainMatch, ProteinFeature,
    ProteinHeader, ProteinSequenceAvailability, UncertainSpan0, UniProtFeature, UniProtIsoform,
};
use serde_json::{Value, json};

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(
    value: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::to_value(serde_json::from_value::<T>(value)?)?)
}

#[test]
fn stated_isoforms_and_residues_roundtrip_without_imputation()
-> Result<(), Box<dyn std::error::Error>> {
    for id in ["P15056", "P15056-2", "P00533-1", "A0A024RBG1-12"] {
        assert_eq!(roundtrip::<UniProtIsoform>(json!(id))?, id);
    }
    for id in ["P15056-0", "P15056-01", "P15056-", "P15056-2-1", "P15056.1"] {
        assert!(serde_json::from_value::<UniProtIsoform>(json!(id)).is_err());
    }
    assert_eq!(roundtrip::<UniProtIsoform>(json!("P15056"))?, "P15056");
    assert_eq!(
        roundtrip::<AminoAcids>(json!("ACDEFGHIKLMNPQRSTVWYUOX"))?,
        "ACDEFGHIKLMNPQRSTVWYUOX"
    );
    for residues in ["", "a", "B", "J", "Z", "*", "A C"] {
        assert!(serde_json::from_value::<AminoAcids>(json!(residues)).is_err());
    }
    assert_eq!(
        Error::ProteinResidueInvalid.code(),
        "protein.residue.invalid"
    );
    assert_eq!(Error::ProteinSiteOrder.code(), "protein.site.order");
    assert_eq!(
        Error::UniProtIsoformInvalid.code(),
        "id.uniprot_isoform.invalid"
    );
    for id in ["IPR000001", "IPR052999"] {
        assert_eq!(roundtrip::<InterProId>(json!(id))?, id);
    }
    for id in ["IPR1", "IPR00000x", "interpro:IPR000001"] {
        assert!(serde_json::from_value::<InterProId>(json!(id)).is_err());
    }
    assert_eq!(roundtrip::<ComplexPortalId>(json!("CPX-715"))?, "CPX-715");
    for id in ["CPX-0", "CPX-0715", "cpx-715", "CPX-715x"] {
        assert!(serde_json::from_value::<ComplexPortalId>(json!(id)).is_err());
    }
    Ok(())
}

#[test]
fn uncertain_feature_bounds_keep_source_ranges() -> Result<(), Box<dyn std::error::Error>> {
    let feature = json!(
        {"protein":"P00533-1","span":{"start":{"min":711,"max":711},"end":{"min":979}},"kind":{"label":"Domain"},"original":"L",
        "alternative":["R"],"evidence":["ECO:0000269|PubMed:1"]}
    );
    assert_eq!(roundtrip::<UniProtFeature>(feature.clone())?, feature);
    for bound in [json!({}), json!({"min":2,"max":1})] {
        assert!(serde_json::from_value::<Bound>(bound).is_err());
    }
    let span = UncertainSpan0::from_inclusive(
        Bound::new(Some(712), Some(714))?,
        Bound::new(Some(979), Some(979))?,
    )?;
    assert_eq!(
        serde_json::to_value(span)?,
        json!({"start":{"min":711,"max":713},"end":{"min":979,"max":979}})
    );
    Ok(())
}

#[test]
fn source_rows_keep_discontinuous_fragments_and_independent_complex_totals()
-> Result<(), Box<dyn std::error::Error>> {
    let domain = json!(
        {"id":"IPR000494","name":"Synthetic source match","entry_type":"family","protein":"P00533","protein_length":1210,"taxon":9606,
        "locations":[{"fragments":[{"span":{"start":{"min":56,"max":56},"end":{"min":167,"max":167}},"dc_status":"N_TERMINAL_DISC"},
        {"span":{"start":{"min":360,"max":360},"end":{"min":480,"max":480}},"dc_status":"C_TERMINAL_DISC"}]},
        {"fragments":[{"span":{"start":{"min":700,"max":700},"end":{"min":710,"max":710}}}]}],"member_signatures":[{"namespace":"pfam",
        "id":"PF01030","name":"Synthetic member"}],"attribution":{"source":"interpro","record":"IPR000494"}}
    );
    assert_eq!(roundtrip::<ProteinDomainMatch>(domain.clone())?, domain);
    let complex = json!(
        {"id":"CPX-715","name":"Synthetic complex","taxon":9606,"predicted":true,"participant_total":93,"matched_participant":"P38398",
        "url":"https://example.org/CPX-715"}
    );
    assert_eq!(roundtrip::<ProteinComplexHeader>(complex.clone())?, complex);
    let member = json!({"id":"CHEBI:1","participant_type":"small molecule","name":"Synthetic non-protein member","stoichiometry":"minValue: 1, maxValue: 1"});
    let v = roundtrip::<ProteinComplexMember>(member.clone())?;
    assert_eq!(v, member);
    assert!(v.get("protein").is_none());
    Ok(())
}

#[test]
fn exact_isoform_header_uses_shared_identity_fields_without_base_replacement()
-> Result<(), Box<dyn std::error::Error>> {
    let header = json!(
        {"protein":{"accession":"P38398-2","name":"Synthetic exact isoform","length":63},"entry_id":"Synthetic entry","taxon":9606,
        "reviewed":true,"url":"https://example.org/P38398-2"}
    );
    assert_eq!(roundtrip::<ProteinHeader>(header.clone())?, header);
    Ok(())
}

#[test]
fn inclusive_conversion_refuses_reversed_or_zero_residues() -> Result<(), Box<dyn std::error::Error>>
{
    let exact = |n| Bound::new(Some(n), Some(n));
    assert_eq!(
        UncertainSpan0::from_inclusive(exact(2)?, exact(1)?),
        Err(biodata::CoordinateError::ReversedBounds)
    );
    assert_eq!(
        UncertainSpan0::from_inclusive(exact(0)?, exact(1)?),
        Err(biodata::CoordinateError::ZeroPosition)
    );
    assert_eq!(
        UncertainSpan0::from_inclusive(exact(1)?, exact(0)?),
        Err(biodata::CoordinateError::ZeroPosition)
    );
    assert!(
        UncertainSpan0::from_inclusive(
            Bound::new(Some(2), Some(4))?,
            Bound::new(Some(1), Some(3))?
        )
        .is_ok()
    );
    Ok(())
}

#[test]
fn feature_provenance_and_sequence_availability_use_closed_native_contracts()
-> Result<(), Box<dyn std::error::Error>> {
    let feature = json!(
        {"feature":{"protein":"P00533","span":{"start":{"min":711,"max":711},"end":{"min":979,"max":979}},"kind":{"label":"Domain"}},
        "attribution":{"source":"uniprot","record":"P00533"},"source_coordinates":"one_based_inclusive"}
    );
    assert_eq!(roundtrip::<ProteinFeature>(feature.clone())?, feature);
    for state in ["available", "not_available", "not_checked"] {
        assert_eq!(
            roundtrip::<ProteinSequenceAvailability>(json!(state))?,
            state
        );
    }
    assert!(serde_json::from_value::<ProteinSequenceAvailability>(json!("inferred")).is_err());
    assert!(serde_json::from_value::<ProteinCoordinateConvention>(json!("zero_based")).is_err());
    Ok(())
}
