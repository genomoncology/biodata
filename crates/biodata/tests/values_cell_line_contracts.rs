//! Feature cell-line-types: source-owned cell identities and standalone contracts before I/O.
use biodata::{CellLineSourceId, CellosaurusAccession, ChemblCellId};
use serde_json::{Value, json};
fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(
    v: Value,
) -> serde_json::Result<Value> {
    serde_json::to_value(serde_json::from_value::<T>(v)?)
}
#[test]
fn checked_cell_identifiers_keep_their_exact_namespace() {
    for id in ["CVCL_2119", "CVCL_A794", "CVCL_E3VV", "CVCL_ZZZZ"] {
        assert_eq!(roundtrip::<CellosaurusAccession>(json!(id)).unwrap(), id);
        let parsed: CellosaurusAccession = id.parse().unwrap();
        assert_eq!(
            parsed.to_string().parse::<CellosaurusAccession>().unwrap(),
            parsed
        );
    }
    for id in [
        "CVCL_1",
        "CVCL_12345",
        "cvcl_2119",
        "CVCL_21-9",
        "CVCL_2119 ",
    ] {
        assert!(serde_json::from_value::<CellosaurusAccession>(json!(id)).is_err());
    }
    for id in [
        "DepMap:ACH-000362",
        "ChEMBL-Cells:CHEMBL3706573",
        "HGNC:HGNC:7132",
        "NCIt:C8263",
        "ORDO:Orphanet_514",
    ] {
        assert_eq!(roundtrip::<CellLineSourceId>(json!(id)).unwrap(), id);
        let parsed: CellLineSourceId = id.parse().unwrap();
        assert_eq!(
            parsed.to_string().parse::<CellLineSourceId>().unwrap(),
            parsed
        );
    }
    for id in [
        "ACH-000362",
        "CHEMBL3706573",
        ":foo",
        "DepMap:",
        "DepMap:ACH 1",
    ] {
        assert!(serde_json::from_value::<CellLineSourceId>(json!(id)).is_err());
    }
    for id in ["CHEMBL3706573", "CHEMBL3308063"] {
        assert_eq!(roundtrip::<ChemblCellId>(json!(id)).unwrap(), id);
        let parsed: ChemblCellId = id.parse().unwrap();
        assert_eq!(parsed.to_string().parse::<ChemblCellId>().unwrap(), parsed);
    }
    for id in [
        "CHEMBL",
        "chembl3706573",
        "CHEMBL37x",
        "chembl:CHEMBL3706573",
    ] {
        assert!(serde_json::from_value::<ChemblCellId>(json!(id)).is_err());
    }
}

// Synthetic contract rows preserve only the multiplicity and absence observed in the
// retained capture archives. They are not native replies or curation answers.
