//! study-types: public no-I/O contracts from retained cBioPortal metadata shapes.
use biodata::{Study, StudySampleList};
use serde_json::{Value, json};

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(
    v: Value,
) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(v)?)
}
fn header() -> Value {
    json!(
        {"studyId":"brca_tcga_pan_can_atlas_2018","name":"Synthetic study","cancerTypeId":"brca","allSampleCount":1084,"pmids":["29625048",
        "29596782","29625048"],"referenceGenome":"hg19"}
    )
}
fn study() -> Value {
    json!(
        {"header":header(),"description":"Synthetic source description","sequencedSampleCount":1066,"cnaSampleCount":1080,
        "structuralVariantCount":0,"treatmentCount":0,"importDate":"2026-08-20 21:46:46","groups":"",
        "readPermission":true,"publicStudy":true}
    )
}
#[test]
fn native_counts_and_publication_multiplicity_survive_without_member_ids()
-> Result<(), Box<dyn std::error::Error>> {
    let v = study();
    assert_eq!(roundtrip::<Study>(v.clone())?, v);
    let mut zero = v;
    zero["patientCount"] = json!(0);
    assert_eq!(roundtrip::<Study>(zero.clone())?, zero);
    let list = json!(
        {"sampleListId":"brca_tcga_pan_can_atlas_2018_all","studyId":"brca_tcga_pan_can_atlas_2018","name":"Synthetic list",
        "description":"Synthetic description","category":"all_cases_in_study","sampleCount":1084}
    );
    assert_eq!(roundtrip::<StudySampleList>(list.clone())?, list);
    let mut absent = list.clone();
    absent
        .as_object_mut()
        .ok_or("object")?
        .remove("sampleCount");
    assert_eq!(roundtrip::<StudySampleList>(absent.clone())?, absent);
    let mut forbidden = list.clone();
    forbidden["sampleIds"] = json!(["synthetic-member"]);
    assert_eq!(roundtrip::<StudySampleList>(forbidden)?, list);
    Ok(())
}
#[test]
fn exact_profile_resource_and_file_metadata_keep_their_native_scope()
-> Result<(), Box<dyn std::error::Error>> {
    use biodata::{Study, StudyFile, StudyId, StudyImportTime, StudyProfile};
    let profile = json!(
        {"molecularProfileId":"brca_tcga_pan_can_atlas_2018_armlevel_cna","studyId":"brca_tcga_pan_can_atlas_2018",
        "molecularAlterationType":"GENERIC_ASSAY","genericAssayType":"ARMLEVEL_CNA","datatype":"CATEGORICAL","name":"Synthetic profile",
        "description":"Synthetic description","patientLevel":false,"showProfileInAnalysisTab":true}
    );
    assert_eq!(roundtrip::<StudyProfile>(profile.clone())?, profile);
    let mut absent = profile;
    absent
        .as_object_mut()
        .ok_or("object")?
        .remove("genericAssayType");
    assert_eq!(roundtrip::<StudyProfile>(absent.clone())?, absent);
    let resource = json!(
        {"resourceId":"IDC_OHIF_CT","displayName":"Synthetic resource","description":"Synthetic resource description",
        "resourceType":"PATIENT","priority":"1","openByDefault":true,"cancerStudyIdentifier":"brca_tcga_pan_can_atlas_2018","sampleCount":7,
        "patientCount":6}
    );
    let mut card = study();
    card["resourceCounts"] = json!([resource]);
    assert_eq!(roundtrip::<Study>(card.clone())?, card);
    let file = json!(
        {"name":"LICENSE","path":"public/brca_tcga_pan_can_atlas_2018/LICENSE","htmlUrl":"https://example.invalid/blob",
        "downloadUrl":"https://example.invalid/raw","sha":"db7b143aab711f10d0349d3f7d56ded293a44c63","repositoryBlobSize":446,"kind":"file"}
    );
    assert_eq!(roundtrip::<StudyFile>(file.clone())?, file);
    let mut directory = file;
    directory["kind"] = json!("dir");
    directory
        .as_object_mut()
        .ok_or("object")?
        .remove("downloadUrl");
    assert_eq!(roundtrip::<StudyFile>(directory.clone())?, directory);
    for invalid in [
        "2026-02-29 01:02:03",
        "2026-10-26 24:00:00",
        "2026-10-26T01:00:00Z",
    ] {
        assert!(invalid.parse::<StudyImportTime>().is_err());
    }
    let imported: StudyImportTime = "2026-08-20 21:46:46".parse()?;
    assert_eq!(imported.to_string(), imported.as_str());
    let error = "bad/id".parse::<StudyId>().err().ok_or("rejected")?;
    assert_eq!(error.code(), "study.value.invalid");
    assert_eq!(error.to_string(), "study.value.invalid");
    Ok(())
}
