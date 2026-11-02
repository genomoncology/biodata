//! Public author contracts before adapters exist; rows below are synthetic.
use biodata::{AuthorHeader, AuthorId, AuthorPaperDetail};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
fn roundtrip<T: DeserializeOwned + Serialize>(v: Value) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(v)?)
}
#[test]
fn source_qualified_identity_checks_checksum_and_declared_namespace() {
    for (input, canonical) in [
        ("semantic-scholar:2060978", "semantic-scholar:2060978"),
        ("orcid:0000-0002-4299-3018", "orcid:0000-0002-4299-3018"),
        ("orcid:0000-0002-1694-233X", "orcid:0000-0002-1694-233X"),
    ] {
        let id: AuthorId = input.parse().unwrap();
        assert_eq!(id.to_string(), canonical);
        assert_eq!(
            roundtrip::<AuthorId>(json!(input)).unwrap(),
            json!(canonical)
        );
        assert_eq!(canonical.parse::<AuthorId>().unwrap(), id);
    }
    for bad in [
        "2060978",
        "semanticscholar:2060978",
        "semantic_scholar:2060978",
        "Semantic-Scholar:2060978",
        "0000-0002-4299-3018",
        "orcid:0000-0002-4299-3019",
        "semantic-scholar:0",
        "semantic-scholar:12 3",
    ] {
        assert!(bad.parse::<AuthorId>().is_err(), "{bad}");
    }
}
#[cfg(feature = "schema")]
#[test]
fn author_identity_schema_matches_the_declared_namespace() {
    let validator = jsonschema::validator_for(&schemars::schema_for!(AuthorId).to_value()).unwrap();
    for (input, expected) in [
        ("semantic-scholar:2060978", true),
        ("semanticscholar:2060978", false),
        ("semantic_scholar:2060978", false),
        ("Semantic-Scholar:2060978", false),
        ("semantic-scholar:0", false),
        ("semantic-scholar:01", false),
    ] {
        assert_eq!(
            input.parse::<AuthorId>().is_ok(),
            expected,
            "the parser accepts only the declared namespace and identifier"
        );
        assert_eq!(
            validator.is_valid(&json!(input)),
            expected,
            "the schema keeps the same namespace boundary"
        );
    }
}

#[test]
fn metadata_absence_and_repeated_byline_names_remain_source_local() {
    // Synthetic repetition protects the required contract; D's detailed pages
    // contain no repeated display names. No native titles or abstracts enter Git.
    let header = json!({"paper_id":"0123456789abcdef0123456789abcdef01234567",
"title":"Synthetic paper",
"external_ids":[{"namespace":"CorpusId","id":"CorpusId:123"}],
"doi":"10.1234/example"});
    let detail = json!(
        {"header":header,"authors":[{"id":"123","name":"Repeated name"},{"id":"456","name":"Repeated name"}],
        "publication_types":["JournalArticle","Review"],"open_access_pdf":{}}
    );
    assert_eq!(
        roundtrip::<AuthorPaperDetail>(detail.clone()).unwrap(),
        detail
    );
    assert!(detail.get("header").unwrap().get("pmid").is_none());
    assert!(detail.get("fields_of_study").is_none());
    let absent = json!({"id":"semantic-scholar:123","name":"Synthetic name"});
    assert_eq!(roundtrip::<AuthorHeader>(absent.clone()).unwrap(), absent);
    let empty = json!({"id":"semantic-scholar:123","name":"Synthetic name","counts":{"scope":"provider_record","paper_count":0}});
    assert_eq!(roundtrip::<AuthorHeader>(empty.clone()).unwrap(), empty);
}
