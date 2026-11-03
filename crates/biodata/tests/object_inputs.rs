//! Object values refuse positional arrays at their public boundary.
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn refuses_array<T: DeserializeOwned + Serialize>(
    object: Value,
    order: &[&str],
) -> Result<(), serde_json::Error> {
    let parsed: T = serde_json::from_value(object.clone())?;
    let output = serde_json::to_value(parsed)?;
    assert!(output.is_object(), "object output shape");
    let array = Value::Array(
        order
            .iter()
            .map(|key| object.get(*key).cloned().unwrap_or(Value::Null))
            .collect(),
    );
    assert!(
        serde_json::from_value::<T>(array).is_err(),
        "{} must refuse positional arrays",
        std::any::type_name::<T>()
    );
    let roundtrip: T = serde_json::from_value(output.clone())?;
    assert_eq!(
        serde_json::to_value(roundtrip)?,
        output,
        "object output stays stable"
    );
    Ok(())
}

macro_rules! witness {
    ($name:ident, $ty:ty, $object:expr, [$($field:literal),*]) => {
        #[test]
        fn $name() -> Result<(), serde_json::Error> {
            refuses_array::<$ty>($object, &[$($field),*])
        }
    };
}

fn gene() -> Value {
    json!({"label":"SYNTH1"})
}
fn attribution() -> Value {
    json!({"source":"synthetic source"})
}
fn identity() -> Value {
    json!({"id":"CHEMBL1","namespace":"chembl","name":"synthetic drug","identity_level":"unknown"})
}
fn interval() -> Value {
    json!({"build":"GRCh38","chromosome":"1","span":{"start":0,"end":1}})
}

witness!(
    span,
    biodata::Span0,
    json!({"start":0,"end":1}),
    ["start", "end"]
);
witness!(
    bound,
    biodata::Bound,
    json!({"min":0,"max":1}),
    ["min", "max"]
);
witness!(
    uncertain_span,
    biodata::UncertainSpan0,
    json!({"start":{"min":0,"max":0},"end":{"min":1,"max":1}}),
    ["start", "end"]
);
witness!(
    symbol_gene,
    biodata::GeneRef,
    gene(),
    ["hgnc_id", "ncbi_gene_id", "ensembl_gene_id", "label"]
);
witness!(
    source_gene,
    biodata::SourceGeneRef,
    gene(),
    ["hgnc_id", "ncbi_gene_id", "ensembl_gene_id", "label"]
);
witness!(
    mane,
    biodata::Mane,
    json!({"status":"select","refseq":"NM_000001.1","ensembl":"ENST00000000001.1"}),
    ["status", "refseq", "ensembl"]
);
witness!(
    region,
    biodata::Region,
    json!({"kind":"exon","number":1,"total":2}),
    ["kind", "number", "total"]
);
witness!(
    protein_site,
    biodata::ProteinSite,
    json!({"gene":gene(),"position":1}),
    [
        "gene",
        "transcript",
        "protein",
        "position",
        "end",
        "residue"
    ]
);
witness!(
    coordinates,
    biodata::TrialCoordinates,
    json!({"latitude":1.0,"longitude":2.0}),
    ["latitude", "longitude"]
);
witness!(
    allele,
    biodata::Allele,
    json!({"location":{"sequence":"NC_000001.11","span":{"start":0,"end":1}},"reference":"A","alternate":"T"}),
    ["location", "reference", "alternate"]
);
witness!(
    expression,
    biodata::Expression,
    json!({"syntax":"hgvs.c","value":"NM_000001.1:c.1A>G"}),
    ["syntax", "value", "syntax_version"]
);
witness!(
    scientific_p_value,
    biodata::ScientificPValue,
    json!({"mantissa":"1","exponent":-2}),
    ["mantissa", "exponent"]
);
witness!(
    profile_expression,
    biodata::ProfileExpression,
    json!({"source":"#VID1","tree":{"operator":"member","record":"civic:1"}}),
    ["source", "tree"]
);
witness!(
    consequence,
    biodata::TranscriptConsequence,
    json!({"transcript":"NM_000001.1","consequences":[{"system":"so","code":"SO:0001583"}],"attribution":attribution()}),
    [
        "transcript",
        "gene",
        "consequences",
        "coding_hgvs",
        "protein_hgvs",
        "region",
        "attribution"
    ]
);
witness!(
    interaction,
    biodata::DrugInteractionClaim,
    json!({"anchor":identity(),"partner":{"id":"CHEMBL2","namespace":"chembl","name":"synthetic partner","identity_level":"unknown"},"attribution":attribution(),"additional_attributions":[]}),
    [
        "anchor",
        "partner",
        "level",
        "attribution",
        "additional_attributions"
    ]
);
witness!(
    target_relation,
    biodata::DrugTargetRelation,
    json!({"drug":identity(),"target":gene(),"native_target":{"namespace":"chembl","id":"CHEMBL3"},"references":[],"attribution":attribution()}),
    [
        "drug",
        "target",
        "native_target",
        "relation",
        "mechanism",
        "references",
        "attribution"
    ]
);
witness!(
    population,
    biodata::PopulationFrequency,
    json!({"subject":{"level":"protein","gene":gene(),"label":"p.A1T"},"build":"GRCh38","dataset":"synthetic dataset","dataset_version":"1","filters":[],"attribution":attribution()}),
    [
        "subject",
        "build",
        "dataset",
        "dataset_version",
        "data_type",
        "ancestry_group",
        "sex",
        "subset",
        "allele_count",
        "allele_number",
        "allele_frequency",
        "homozygote_count",
        "hemizygote_count",
        "faf95",
        "filters",
        "genotype_filter",
        "grpmax_group",
        "heterozygote_count",
        "homoplasmic_count",
        "heteroplasmic_count",
        "homoplasmic_frequency",
        "heteroplasmic_frequency",
        "attribution"
    ]
);
witness!(
    gene_subject,
    biodata::Subject,
    json!({"level":"gene","label":"SYNTH1"}),
    [
        "level",
        "hgnc_id",
        "ncbi_gene_id",
        "ensembl_gene_id",
        "label"
    ]
);
witness!(
    region_subject,
    biodata::Subject,
    json!({"level":"region","interval":interval(),"label":"synthetic region"}),
    ["level", "interval", "label", "record"]
);

fn method_relation() -> Value {
    json!({"kind":"method","relation":{"test_id":"GTR000000001.1","list":"methods","index":0,"value":"synthetic method"}})
}
witness!(
    coding,
    biodata::Coding,
    json!({"system":"so","code":"SO:0001583"}),
    ["system", "code"]
);
witness!(
    pathway_id,
    biodata::PathwayId,
    json!({"namespace":"wikipathways","id":"WP1"}),
    ["namespace", "id"]
);
witness!(
    gtr_relation,
    biodata::GtrTestRelation,
    method_relation(),
    ["kind", "relation"]
);
witness!(
    diagnostic_record,
    biodata::DiagnosticRecord,
    json!({"kind":"gtr_test_relation","record":method_relation()}),
    ["kind", "record"]
);

// A binary format can present a record as a sequence. Keep that derived reader
// without adding a second serialization library just for the input witness.
struct BinaryRecord(Value);
impl<'de> serde::Deserializer<'de> for BinaryRecord {
    type Error = serde_json::Error;
    fn is_human_readable(&self) -> bool {
        false
    }
    fn deserialize_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        serde::Deserializer::deserialize_any(self.0, visitor)
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }
}

fn plain_record<T: DeserializeOwned + Serialize>(
    object: Value,
    order: &[&str],
) -> Result<(), serde_json::Error> {
    let parsed: T = serde_json::from_value(object.clone())?;
    let output = serde_json::to_value(parsed)?;
    let array = Value::Array(
        order
            .iter()
            .map(|key| object.get(*key).cloned().unwrap_or(Value::Null))
            .collect(),
    );
    let binary: T = serde::Deserialize::deserialize(BinaryRecord(array))?;
    assert_eq!(
        serde_json::to_value(binary)?,
        output,
        "binary field positions stay stable"
    );
    refuses_array::<T>(object, order)
}

// Count declarations from the source, including records that bypass the macro.
// Adding either kind without a witness must fail this walk.
#[expect(
    clippy::disallowed_methods,
    reason = "This test reads the source tree to detect records without witnesses"
)]
fn source_plain_records(
    folder: &std::path::Path,
) -> std::io::Result<std::collections::BTreeMap<String, bool>> {
    let mut records = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_dir() {
            records.extend(source_plain_records(&path)?);
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let source = std::fs::read_to_string(path)?;
        let mut attributes = String::new();
        let mut depth = 0_i32;
        let mut quoted = false;
        let mut escaped = false;
        let mut shared = false;
        for line in source.lines().map(str::trim) {
            if line.starts_with("crate::serde_object::object_record!") {
                shared = true;
            }
            if depth > 0 || line.starts_with("#[") {
                attributes.push_str(line);
                for character in line.chars() {
                    if escaped {
                        escaped = false;
                        continue;
                    }
                    if quoted && character == '\\' {
                        escaped = true;
                        continue;
                    }
                    if character == '"' {
                        quoted = !quoted;
                    }
                    if !quoted {
                        if character == '[' {
                            depth += 1;
                        }
                        if character == ']' {
                            depth -= 1;
                        }
                    }
                }
                continue;
            }
            if let Some(declaration) = line.strip_prefix("pub struct ") {
                if declaration.contains('{')
                    && (shared
                        || (attributes.contains("Deserialize") && !attributes.contains("from =")))
                {
                    let name = declaration
                        .split(['<', ' ', '{'])
                        .next()
                        .expect("split always yields the declaration prefix");
                    assert!(
                        records.insert(name.to_owned(), shared).is_none(),
                        "duplicate record {name}"
                    );
                }
                shared = false;
            }
            if !line.starts_with("///") && !line.is_empty() {
                attributes.clear();
            }
        }
    }
    Ok(records)
}

macro_rules! plain_records {
    ($($name:ident => $ty:ty, $object:expr, [$($field:literal),*];)*) => {
        #[test]
        fn every_plain_record_requires_an_object() -> Result<(), Box<dyn std::error::Error>> {
            let source = source_plain_records(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"))?;
            let witnesses: std::collections::BTreeSet<_> = [$(stringify!($name).to_owned()),*].into_iter().collect();
            assert_eq!(source.keys().cloned().collect::<std::collections::BTreeSet<_>>(), witnesses,
                "every source record needs an object/array witness");
            let mut failures = Vec::new();
            $(if let Err(error) = plain_record::<$ty>($object, &[$($field),*]) {
                failures.push(format!("{}: {error}", stringify!($name)));
            })*
            assert!(failures.is_empty(), "invalid synthetic witnesses: {failures:?}");
            assert!(source.values().all(|shared| *shared), "every plain record must use the shared declaration");
            Ok(())
        }
    };
}

// Synthetic source-shaped objects; no fixture comes from a patient or publication.
plain_records! {
    AlleleKey => biodata::AlleleKey, json!({"build":"GRCh38","allele":{"location":{"sequence":"NC_000001.11","span":{"start":0,"end":1}},"reference":"A","alternate":"T"}}), ["build", "allele"];
    Attribution => biodata::Attribution, json!({"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}), ["approved", "published", "source", "record", "version", "release", "submitter", "evaluated", "url"];
    AuthorByline => biodata::AuthorByline, json!({"id":null,"name":null}), ["id", "name"];
    AuthorCard => biodata::AuthorCard, json!({"header":{"id":"semantic-scholar:1","name":null,"affiliations":null,"counts":null},"url":null}), ["header", "url"];
    AuthorCounts => biodata::AuthorCounts, json!({"scope":"provider_record","paper_count":null,"citation_count":null,"h_index":null}), ["scope", "paper_count", "citation_count", "h_index"];
    AuthorHeader => biodata::AuthorHeader, json!({"id":"semantic-scholar:1","name":null,"affiliations":null,"counts":null}), ["id", "name", "affiliations", "counts"];
    AuthorPaperAccess => biodata::AuthorPaperAccess, json!({"url":null,"status":null,"license":null,"disclaimer":null}), ["url", "status", "license", "disclaimer"];
    AuthorPaperDetail => biodata::AuthorPaperDetail, json!({"header":{"paper_id":"0000000000000000000000000000000000000000","pmid":null,"pmcid":null,"doi":null,"external_ids":[],"title":null,"year":null,"venue":null},"abstract_text":null,"publication_date":null,"reference_count":null,"citation_count":null,"influential_citation_count":null,"is_open_access":null,"open_access_pdf":null,"fields_of_study":null,"publication_types":null,"authors":null}), ["header", "abstract_text", "publication_date", "reference_count", "citation_count", "influential_citation_count", "is_open_access", "open_access_pdf", "fields_of_study", "publication_types", "authors"];
    AuthorPaperExternalId => biodata::AuthorPaperExternalId, json!({"namespace":"synthetic text","id":"synthetic:1"}), ["namespace", "id"];
    AuthorPaperHeader => biodata::AuthorPaperHeader, json!({"paper_id":"0000000000000000000000000000000000000000","pmid":null,"pmcid":null,"doi":null,"external_ids":[],"title":null,"year":null,"venue":null}), ["paper_id", "pmid", "pmcid", "doi", "external_ids", "title", "year", "venue"];
    AuthorizationDate => biodata::AuthorizationDate, json!({"role":"synthetic text","date":"2026-01-01"}), ["role", "date"];
    AuthorizationEvent => biodata::AuthorizationEvent, json!({"record":"CHEMBL1","product_number":null,"jurisdiction":null,"status":null,"kind":null,"dates":[],"indication":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["record", "product_number", "jurisdiction", "status", "kind", "dates", "indication", "attribution"];
    CellLineEntryDates => biodata::CellLineEntryDates, json!({"created":null,"updated":null,"version":null}), ["created", "updated", "version"];
    CellLineGene => biodata::CellLineGene, json!({"reference":{"namespace":"synthetic text","id":"synthetic text","label":null,"url":null,"category":null,"iri":null,"discontinued":null},"hgnc":null}), ["reference", "hgnc"];
    CellLineIdentity => biodata::CellLineIdentity, json!({"accession":"CVCL_0001","name":"synthetic text","category":"synthetic text","species":[],"diseases":[],"age":null,"sex":null}), ["accession", "name", "category", "species", "diseases", "age", "sex"];
    CellLineName => biodata::CellLineName, json!({"kind":"synthetic text","value":"synthetic text"}), ["kind", "value"];
    CellLineProvenance => biodata::CellLineProvenance, json!({"reference":null,"institution":null,"value":null,"xref":null}), ["reference", "institution", "value", "xref"];
    CellLineReference => biodata::CellLineReference, json!({"resource_internal_ref":"synthetic text"}), ["resource_internal_ref"];
    CellLineVariation => biodata::CellLineVariation, json!({"variation_type":null,"mutation_type":null,"description":null,"note":null,"zygosity":null,"genes":[],"references":[],"provenance":[]}), ["variation_type", "mutation_type", "description", "note", "zygosity", "genes", "references", "provenance"];
    CellLineWarning => biodata::CellLineWarning, json!({"scope":"synthetic text","text":"synthetic text"}), ["scope", "text"];
    CellLineXref => biodata::CellLineXref, json!({"namespace":"synthetic text","id":"synthetic text","label":null,"url":null,"category":null,"iri":null,"discontinued":null}), ["namespace", "id", "label", "url", "category", "iri", "discontinued"];
    ClinvarHaplotype => biodata::ClinvarHaplotype, json!({"kind":"Haplotype","id":"clinvar:1","accession":"VCV000000001","version":"synthetic text","number_of_chromosomes":"synthetic text","members":[]}), ["kind", "id", "accession", "version", "number_of_chromosomes", "members"];
    ClinvarHaplotypeMember => biodata::ClinvarHaplotypeMember, json!({"id":"clinvar:1","allele_id":"1"}), ["id", "allele_id"];
    CohortObservation => biodata::CohortObservation, json!({"subject":{"level":"protein","gene":{"label":"SYNTH1"},"label":"p.A1T"},"dataset":"synthetic text","dataset_version":null,"unit":"samples","sample_count":1,"profiled_sample_count":null,"cohort_sample_count":null,"by_condition":[],"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["subject", "dataset", "dataset_version", "unit", "sample_count", "profiled_sample_count", "cohort_sample_count", "by_condition", "attribution"];
    CompanionGroupCells => biodata::CompanionGroupCells, json!({"Diagnostic Name\n(Manufacturer)":{"text":"synthetic text","links":[]},"Indication(s) - Sample Type":{"text":"synthetic text","links":[]},"PMA (Approval Date)":{"text":"synthetic text","links":[]},"Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA":{"text":"synthetic text","links":[]}}), ["Diagnostic Name\n(Manufacturer)", "Indication(s) - Sample Type", "PMA (Approval Date)", "Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA"];
    CompanionIndividualCells => biodata::CompanionIndividualCells, json!({"Diagnostic Name (Manufacturer)":{"text":"synthetic text","links":[]},"Indication - Sample Type":{"text":"synthetic text","links":[]},"Drug Trade Name (Generic) NDA / BLA":{"text":"synthetic text","links":[]},"Biomarker(s)":{"text":"synthetic text","links":[]},"Biomarker(s) (Details)":{"text":"synthetic text","links":[]},"PMA / 510(k) /\n513(f)(2) / HDE (Approval / Clearance / Grant Date)":{"text":"synthetic text","links":[]}}), ["Diagnostic Name (Manufacturer)", "Indication - Sample Type", "Drug Trade Name (Generic) NDA / BLA", "Biomarker(s)", "Biomarker(s) (Details)", "PMA / 510(k) /\n513(f)(2) / HDE (Approval / Clearance / Grant Date)"];
    Concept => biodata::Concept, json!({"id":null,"label":"synthetic text"}), ["id", "label"];
    ConditionCount => biodata::ConditionCount, json!({"condition":{"id":null,"label":"synthetic text"},"count":1,"profiled_sample_count":null}), ["condition", "count", "profiled_sample_count"];
    CopyNumberChange => biodata::CopyNumberChange, json!({"location":{"sequence":"NC_000001.11","span":{"start":{"min":0,"max":0},"end":{"min":1,"max":1}}},"copy_change":"loss"}), ["location", "copy_change"];
    CopyNumberCount => biodata::CopyNumberCount, json!({"location":{"sequence":"NC_000001.11","span":{"start":{"min":0,"max":0},"end":{"min":1,"max":1}}},"copies":{"min":1,"max":1}}), ["location", "copies"];
    DeviceSubmission => biodata::DeviceSubmission, json!({"pma_number":"P000001","supplement_number":null,"trade_name":null,"applicant":null,"generic_name":null,"decision_date":null,"decision_code":null,"date_received":null,"fed_reg_notice_date":null,"advisory_committee":null,"advisory_committee_description":null,"product_code":null,"supplement_type":null,"supplement_reason":null,"ao_statement":null,"docket_number":null,"expedited_review_flag":null}), ["pma_number", "supplement_number", "trade_name", "applicant", "generic_name", "decision_date", "decision_code", "date_received", "fed_reg_notice_date", "advisory_committee", "advisory_committee_description", "product_code", "supplement_type", "supplement_reason", "ao_statement", "docket_number", "expedited_review_flag"];
    DiagnosticCell => biodata::DiagnosticCell, json!({"text":"synthetic text","links":[]}), ["text", "links"];
    DiagnosticCellLink => biodata::DiagnosticCellLink, json!({"href":"synthetic text","label":null}), ["href", "label"];
    DiagnosticDrugApplication => biodata::DiagnosticDrugApplication, json!({"id":"NDA000001","url":"synthetic text"}), ["id", "url"];
    DiseaseAssociation => biodata::DiseaseAssociation, json!({"requested_id":"synthetic:1","subject":{"id":"synthetic:1","label":null},"subject_taxon":null,"predicate":"synthetic:1","object":{"id":"synthetic:1","label":null},"original_subject":null,"original_object":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null},"primary_knowledge_source":null,"aggregator_knowledge_source":null,"provided_by":null,"evidence":null,"qualifiers":[],"publications":[],"original_predicate":null,"category":null,"agent_type":null,"knowledge_level":null,"evidence_count":null,"negated":null,"has_count":null,"has_total":null,"has_percentage":null,"has_quotient":null,"subject_closure":[],"object_closure":[]}), ["requested_id", "subject", "subject_taxon", "predicate", "object", "original_subject", "original_object", "attribution", "primary_knowledge_source", "aggregator_knowledge_source", "provided_by", "evidence", "qualifiers", "publications", "original_predicate", "category", "agent_type", "knowledge_level", "evidence_count", "negated", "has_count", "has_total", "has_percentage", "has_quotient", "subject_closure", "object_closure"];
    DiseaseAssociationQualifier => biodata::DiseaseAssociationQualifier, json!({"field":"synthetic text","id":"synthetic:1","label":null}), ["field", "id", "label"];
    DiseaseAssociationReference => biodata::DiseaseAssociationReference, json!({"id":"synthetic:1","label":null,"url":null}), ["id", "label", "url"];
    DiseaseEndpoint => biodata::DiseaseEndpoint, json!({"id":"synthetic:1","label":null}), ["id", "label"];
    DiseaseRelation => biodata::DiseaseRelation, json!({"description":null,"subject":{"id":"synthetic:1","label":null},"predicate":"synthetic:1","object":{"id":"synthetic:1","label":null},"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["description", "subject", "predicate", "object", "attribution"];
    DiseaseStatistic => biodata::DiseaseStatistic, json!({"requested_site":"synthetic text","requested_selectors":[],"returned_selectors":[],"year":null,"measure":"synthetic text","observed":null,"modeled":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["requested_site", "requested_selectors", "returned_selectors", "year", "measure", "observed", "modeled", "attribution"];
    DiseaseSynonym => biodata::DiseaseSynonym, json!({"value":"synthetic text","scope":null,"language":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["value", "scope", "language", "attribution"];
    DrugDiseaseReference => biodata::DrugDiseaseReference, json!({"namespace":"synthetic text","id":"synthetic text","label":null}), ["namespace", "id", "label"];
    DrugIdentity => biodata::DrugIdentity, json!({"id":"CHEMBL1","namespace":"synthetic text","name":null,"identity_level":"unknown","type":null,"url":null}), ["id", "namespace", "name", "identity_level", "type", "url"];
    DrugIdentityRelation => biodata::DrugIdentityRelation, json!({"identity":{"id":"CHEMBL1","namespace":"synthetic text","name":null,"identity_level":"unknown","type":null,"url":null},"relation":"synthetic text","attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["identity", "relation", "attribution"];
    DrugIndicationClaim => biodata::DrugIndicationClaim, json!({"drug":{"id":"CHEMBL1","namespace":"synthetic text","name":null,"identity_level":"unknown","type":null,"url":null},"indication":null,"disease_references":[],"stage":null,"references":[],"population":null,"combination":[],"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["drug", "indication", "disease_references", "stage", "references", "population", "combination", "attribution"];
    DrugLabelReference => biodata::DrugLabelReference, json!({"id":"CHEMBL1","version":null,"revised":null,"url":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["id", "version", "revised", "url", "attribution"];
    DrugLabelSection => biodata::DrugLabelSection, json!({"label":{"id":"CHEMBL1","version":null,"revised":null,"url":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}},"heading":"synthetic text","text":"synthetic text"}), ["label", "heading", "text"];
    DrugNativeReference => biodata::DrugNativeReference, json!({"ref_type":"synthetic text","ref_id":null,"ref_url":null}), ["ref_type", "ref_id", "ref_url"];
    DrugNativeTarget => biodata::DrugNativeTarget, json!({"namespace":"synthetic text","id":"synthetic text","label":null,"url":null}), ["namespace", "id", "label", "url"];
    DrugProduct => biodata::DrugProduct, json!({"identity":{"id":"CHEMBL1","namespace":"synthetic text","name":null,"identity_level":"unknown","type":null,"url":null},"formulations":[],"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["identity", "formulations", "attribution"];
    DrugStage => biodata::DrugStage, json!({"field":"synthetic text","value":"synthetic text"}), ["field", "value"];
    FaersDrug => biodata::FaersDrug, json!({"medicinalproduct":null,"drugcharacterization":null,"drugindication":null,"drugadministrationroute":null,"drugauthorizationnumb":null,"drugbatchnumb":null,"drugcumulativedosagenumb":null,"drugcumulativedosageunit":null,"drugdosageform":null,"drugdosagetext":null,"drugenddate":null,"drugenddateformat":null,"drugintervaldosagedefinition":null,"drugintervaldosageunitnumb":null,"drugrecurreadministration":null,"drugseparatedosagenumb":null,"drugstartdate":null,"drugstartdateformat":null,"drugstructuredosagenumb":null,"drugstructuredosageunit":null,"drugtreatmentduration":null,"drugtreatmentdurationunit":null,"actiondrug":null,"drugadditional":null,"openfda":null}), ["medicinalproduct", "drugcharacterization", "drugindication", "drugadministrationroute", "drugauthorizationnumb", "drugbatchnumb", "drugcumulativedosagenumb", "drugcumulativedosageunit", "drugdosageform", "drugdosagetext", "drugenddate", "drugenddateformat", "drugintervaldosagedefinition", "drugintervaldosageunitnumb", "drugrecurreadministration", "drugseparatedosagenumb", "drugstartdate", "drugstartdateformat", "drugstructuredosagenumb", "drugstructuredosageunit", "drugtreatmentduration", "drugtreatmentdurationunit", "actiondrug", "drugadditional", "openfda"];
    FaersPatient => biodata::FaersPatient, json!({"age":null,"sex":null,"weight":null}), ["age", "sex", "weight"];
    FaersReaction => biodata::FaersReaction, json!({"term":null,"vocabulary_version":null,"outcome":null}), ["term", "vocabulary_version", "outcome"];
    FaersReport => biodata::FaersReport, json!({"safetyreportid":"000001","safetyreportversion":null,"receivedate":null,"receivedateformat":null,"receiptdate":null,"receiptdateformat":null,"transmissiondate":null,"transmissiondateformat":null,"serious":null,"seriousness":null,"patient":null,"reporter":null,"drugs":[],"reactions":[]}), ["safetyreportid", "safetyreportversion", "receivedate", "receivedateformat", "receiptdate", "receiptdateformat", "transmissiondate", "transmissiondateformat", "serious", "seriousness", "patient", "reporter", "drugs", "reactions"];
    FaersReporter => biodata::FaersReporter, json!({"qualification":null,"country":null}), ["qualification", "country"];
    FaersSeriousness => biodata::FaersSeriousness, json!({"death":null,"lifethreatening":null,"hospitalization":null,"disabling":null,"congenitalanomali":null,"other":null}), ["death", "lifethreatening", "hospitalization", "disabling", "congenitalanomali", "other"];
    Gene => biodata::Gene, json!({"hgnc_id":"HGNC:1","symbol":"SYNTH1","entrez_id":null,"ensembl_id":null,"status":"approved","locus_group":null,"locus_type":null,"location":null,"previous_symbols":[],"aliases":[],"uniprot":[],"omim":[],"mane":[],"spans":[],"name":null,"summary":null}), ["hgnc_id", "symbol", "entrez_id", "ensembl_id", "status", "locus_group", "locus_type", "location", "previous_symbols", "aliases", "uniprot", "omim", "mane", "spans", "name", "summary"];
    GenomicInterval => biodata::GenomicInterval, json!({"build":null,"chromosome":"1","span":{"start":0,"end":1},"strand":null}), ["build", "chromosome", "span", "strand"];
    GtrConditionReference => biodata::GtrConditionReference, json!({"id":"synthetic:1","label":null}), ["id", "label"];
    GtrConditionRelation => biodata::GtrConditionRelation, json!({"test_id":"GTR000000001.1","test_type":null,"condition":{"id":"synthetic:1","label":null},"mim_number":"synthetic text","snomed_ct_id":"synthetic text"}), ["test_id", "test_type", "condition", "mim_number", "snomed_ct_id"];
    GtrGeneReference => biodata::GtrGeneReference, json!({"label":null,"ncbi_gene_id":null}), ["label", "ncbi_gene_id"];
    GtrGeneRelation => biodata::GtrGeneRelation, json!({"test_id":"GTR000000001.1","test_type":null,"gene":{"label":null,"ncbi_gene_id":null},"native_gene_or_snomed_ct_id":null,"native_medgen":null,"mim_number":"synthetic text","object_name":"synthetic text"}), ["test_id", "test_type", "gene", "native_gene_or_snomed_ct_id", "native_medgen", "mim_number", "object_name"];
    GtrMethod => biodata::GtrMethod, json!({"test_id":"GTR000000001.1","list":"methods","index":1,"value":"synthetic text"}), ["test_id", "list", "index", "value"];
    GtrTestVersion => biodata::GtrTestVersion, json!({"test_accession_ver":"GTR000000001.1","name_of_laboratory":"synthetic text","name_of_institution":"synthetic text","facility_state":"synthetic text","facility_postcode":"synthetic text","facility_country":"synthetic text","CLIA_number":"synthetic text","state_licenses":"synthetic text","state_license_numbers":"synthetic text","lab_test_id":"synthetic text","last_touch_date":"synthetic text","lab_test_name":"synthetic text","manufacturer_test_name":"synthetic text","test_development":"synthetic text","lab_id":"synthetic text","condition_identifiers":"synthetic text","indication_types":"synthetic text","inheritances":"synthetic text","method_categories":"synthetic text","methods":"synthetic text","platforms":"synthetic text","genes":"synthetic text","drug_responses":"synthetic text","now_current":"synthetic text","test_currStat":"synthetic text","test_pubStat":"synthetic text","lab_currStat":"synthetic text","lab_pubStat":"synthetic text","test_create_date":"synthetic text","test_deletion_date":"synthetic text","test_order_code":"synthetic text"}), ["test_accession_ver", "name_of_laboratory", "name_of_institution", "facility_state", "facility_postcode", "facility_country", "CLIA_number", "state_licenses", "state_license_numbers", "lab_test_id", "last_touch_date", "lab_test_name", "manufacturer_test_name", "test_development", "lab_id", "condition_identifiers", "indication_types", "inheritances", "method_categories", "methods", "platforms", "genes", "drug_responses", "now_current", "test_currStat", "test_pubStat", "lab_currStat", "lab_pubStat", "test_create_date", "test_deletion_date", "test_order_code"];
    Identifier => biodata::Identifier<i64>, json!({"namespace":"synthetic","id":1}), ["namespace", "id"];
    MaudeDevice => biodata::MaudeDevice, json!({"device_sequence_number":null,"brand_name":null,"generic_name":null,"manufacturer_d_name":null,"device_report_product_code":null,"model_number":null,"catalog_number":null,"udi_di":null,"udi_public":null,"lot_number":null,"device_operator":null,"implant_flag":null,"implant_date_year":null,"date_removed_flag":null,"date_removed_year":null,"device_age_text":null,"expiration_date_of_device":null,"device_availability":null,"device_evaluated_by_manufacturer":null,"device_event_key":null,"date_received":null,"other_id_number":null,"combination_product_flag":null,"serviced_by_3rd_party_flag":null}), ["device_sequence_number", "brand_name", "generic_name", "manufacturer_d_name", "device_report_product_code", "model_number", "catalog_number", "udi_di", "udi_public", "lot_number", "device_operator", "implant_flag", "implant_date_year", "date_removed_flag", "date_removed_year", "device_age_text", "expiration_date_of_device", "device_availability", "device_evaluated_by_manufacturer", "device_event_key", "date_received", "other_id_number", "combination_product_flag", "serviced_by_3rd_party_flag"];
    MaudeNarrative => biodata::MaudeNarrative, json!({"mdr_text_key":null,"patient_sequence_number":null,"text_type_code":null,"text":null}), ["mdr_text_key", "patient_sequence_number", "text_type_code", "text"];
    MaudeReport => biodata::MaudeReport, json!({"mdr_report_key":"000001","report_number":null,"event_type":null,"date_of_event":null,"date_received":null,"report_date":null,"date_report":null,"date_report_to_fda":null,"number_devices_in_event":null,"number_patients_in_event":null,"devices":[],"narrative":[]}), ["mdr_report_key", "report_number", "event_type", "date_of_event", "date_received", "report_date", "date_report", "date_report_to_fda", "number_devices_in_event", "number_patients_in_event", "devices", "narrative"];
    Method => biodata::Method, json!({"label":"synthetic text","version":null}), ["label", "version"];
    OrcidAssertionSource => biodata::OrcidAssertionSource, json!({"source_orcid":null,"source_client_id":null,"source_name":null,"assertion_origin_orcid":null,"assertion_origin_client_id":null,"assertion_origin_name":null}), ["source_orcid", "source_client_id", "source_name", "assertion_origin_orcid", "assertion_origin_client_id", "assertion_origin_name"];
    OrcidExternalId => biodata::OrcidExternalId, json!({"namespace":"synthetic","id":"synthetic text","relationship":null,"url":null}), ["namespace", "id", "relationship", "url"];
    OrcidWorkAssertion => biodata::OrcidWorkAssertion, json!({"put_code":1,"title":null,"subtitle":null,"translated_title":null,"work_type":null,"publication_date":null,"journal_title":null,"url":null,"external_ids":[],"source":null,"visibility":"public","display_index":null}), ["put_code", "title", "subtitle", "translated_title", "work_type", "publication_date", "journal_title", "url", "external_ids", "source", "visibility", "display_index"];
    OrcidWorkGroup => biodata::OrcidWorkGroup, json!({"external_ids":[],"assertions":[]}), ["external_ids", "assertions"];
    Pathway => biodata::Pathway, json!({"id":{"namespace":"wikipathways","id":"WP1"},"name":null,"species":"Homo sapiens","description":null,"summations":null,"references":null,"cited_in":null,"version":null,"revision":null}), ["id", "name", "species", "description", "summations", "references", "cited_in", "version", "revision"];
    PathwayLiteratureRef => biodata::PathwayLiteratureRef, json!({"db_id":{"namespace":"synthetic","id":1},"pubmed":null,"title":null,"url":null}), ["db_id", "pubmed", "title", "url"];
    PathwayMember => biodata::PathwayMember, json!({"node_id":null,"reference":{"namespace":"synthetic","id":"synthetic text"},"name":null,"source_url":null,"gene":null,"schema_class":null,"physical_context_count":1,"participants":null}), ["node_id", "reference", "name", "source_url", "gene", "schema_class", "physical_context_count", "participants"];
    PathwayParticipant => biodata::PathwayParticipant, json!({"db_id":{"namespace":"synthetic","id":1},"name":null,"schema_class":null}), ["db_id", "name", "schema_class"];
    PathwayRef => biodata::PathwayRef, json!({"id":"R-HSA-1","name":null}), ["id", "name"];
    PathwaySourceRef => biodata::PathwaySourceRef, json!({"namespace":"synthetic","id":"synthetic text"}), ["namespace", "id"];
    PathwaySummation => biodata::PathwaySummation, json!({"db_id":{"namespace":"synthetic","id":1},"text":null}), ["db_id", "text"];
    PgxAllelePhenotype => biodata::PgxAllelePhenotype, json!({"id":{"namespace":"synthetic","id":1},"allele":null,"limitedEvidence":null,"phenotype":null}), ["id", "allele", "limitedEvidence", "phenotype"];
    PgxAnnotationDetail => biodata::PgxAnnotationDetail, json!({"id":{"namespace":"synthetic","id":1},"accessionId":"PA1","allele_phenotypes":[],"conflicting_variant_annotation_ids":[],"history":[],"levelOfEvidence":null,"location":null,"name":null,"objCls":null,"overrideLevel":null,"pediatric":null,"related_alleles":[],"related_chemicals":[],"relatedChemicalsLogic":null,"related_diseases":[],"related_guidelines":[],"related_labels":[],"score":null,"scoreDetails":null,"types":[],"override_level_description":null}), ["id", "accessionId", "allele_phenotypes", "conflicting_variant_annotation_ids", "history", "levelOfEvidence", "location", "name", "objCls", "overrideLevel", "pediatric", "related_alleles", "related_chemicals", "relatedChemicalsLogic", "related_diseases", "related_guidelines", "related_labels", "score", "scoreDetails", "types", "override_level_description"];
    PgxAnnotationHeader => biodata::PgxAnnotationHeader, json!({"id":{"namespace":"synthetic","id":1},"accessionId":"PA1","levelOfEvidence":null,"location":null,"name":null,"related_chemicals":[],"relatedChemicalsLogic":null,"related_guidelines":[],"related_labels":[],"types":[]}), ["id", "accessionId", "levelOfEvidence", "location", "name", "related_chemicals", "relatedChemicalsLogic", "related_guidelines", "related_labels", "types"];
    PgxAnnotationHistory => biodata::PgxAnnotationHistory, json!({"id":{"namespace":"synthetic","id":1},"date":null,"type":null,"version":null,"description":null}), ["id", "date", "type", "version", "description"];
    PgxAnnotationReference => biodata::PgxAnnotationReference, json!({"objCls":"synthetic text","id":"PA1","name":null,"cross_references":[]}), ["objCls", "id", "name", "cross_references"];
    PgxCrossReference => biodata::PgxCrossReference, json!({"id":{"namespace":"synthetic","id":1},"resource":null,"resourceId":null,"_url":null,"name":null}), ["id", "resource", "resourceId", "_url", "name"];
    PgxDiplotypeReference => biodata::PgxDiplotypeReference, json!({"id":{"namespace":"synthetic","id":1}}), ["id"];
    PgxDiplotypeResult => biodata::PgxDiplotypeResult, json!({"genesymbol":"SYNTH1","diplotype":"synthetic text","function1":null,"function2":null,"activityvalue1":null,"activityvalue2":null,"totalactivityscore":null,"description":null,"diplotypekey":null,"generesult":null,"ehrpriority":null,"consultationtext":null,"lookupkey":null}), ["genesymbol", "diplotype", "function1", "function2", "activityvalue1", "activityvalue2", "totalactivityscore", "description", "diplotypekey", "generesult", "ehrpriority", "consultationtext", "lookupkey"];
    PgxDrugRecommendation => biodata::PgxDrugRecommendation, json!({"id":{"namespace":"synthetic","id":1},"guidelineid":null,"drugid":"synthetic:1","implications":null,"drugrecommendation":null,"classification":null,"phenotypes":null,"activityscore":null,"allelestatus":null,"lookupkey":{},"population":null,"comments":null,"version":null,"dosinginformation":null,"alternatedrugavailable":null,"otherprescribingguidance":null}), ["id", "guidelineid", "drugid", "implications", "drugrecommendation", "classification", "phenotypes", "activityscore", "allelestatus", "lookupkey", "population", "comments", "version", "dosinginformation", "alternatedrugavailable", "otherprescribingguidance"];
    PgxEvidenceTerm => biodata::PgxEvidenceTerm, json!({"id":{"namespace":"synthetic","id":1},"resource":null,"term":null,"termId":null,"synonyms":null,"valid":null}), ["id", "resource", "term", "termId", "synonyms", "valid"];
    PgxGeneIdentity => biodata::PgxGeneIdentity, json!({"symbol":"SYNTH1","hgncid":null,"ncbiid":null,"ensemblid":null,"clinpgxid":null,"lookupmethod":null,"version":null}), ["symbol", "hgncid", "ncbiid", "ensemblid", "clinpgxid", "lookupmethod", "version"];
    PgxGuidelineGene => biodata::PgxGuidelineGene, json!({"url":null,"symbol":"SYNTH1"}), ["url", "symbol"];
    PgxGuidelineReference => biodata::PgxGuidelineReference, json!({"guideline_name":"synthetic text","guideline_url":null,"drugs":[],"genes":[]}), ["guideline_name", "guideline_url", "drugs", "genes"];
    PgxLocation => biodata::PgxLocation, json!({"id":{"namespace":"synthetic","id":1},"build_version":null,"chromosome_id":null,"chromosome_name":null,"copyNumber":null,"diplotypes":[],"displayName":null,"genes":[],"gpPosition":null,"haplotypes":[],"linked_objects":[],"ref_seq_cross_reference":null,"refSeqPosition":null,"rsid":null,"tagGene":null,"type":null,"variant":null}), ["id", "build_version", "chromosome_id", "chromosome_name", "copyNumber", "diplotypes", "displayName", "genes", "gpPosition", "haplotypes", "linked_objects", "ref_seq_cross_reference", "refSeqPosition", "rsid", "tagGene", "type", "variant"];
    PgxObjectReference => biodata::PgxObjectReference, json!({"obj_cls":null,"id":"PA1","name":null,"symbol":null}), ["obj_cls", "id", "name", "symbol"];
    PgxPair => biodata::PgxPair, json!({"pairid":{"namespace":"synthetic","id":1},"drugid":null,"drugname":null,"genesymbol":"SYNTH1","guidelinename":null,"guidelineurl":null,"cpiclevel":null,"clinpgxlevel":null,"pgxtesting":null,"pmids":null,"usedforrecommendation":null,"provisional":null}), ["pairid", "drugid", "drugname", "genesymbol", "guidelinename", "guidelineurl", "cpiclevel", "clinpgxlevel", "pgxtesting", "pmids", "usedforrecommendation", "provisional"];
    PgxRecommendation => biodata::PgxRecommendation, json!({"recommendationid":{"namespace":"synthetic","id":1},"lookupkey":{},"drugname":null,"guidelinename":null,"guidelineurl":null,"implications":null,"drugrecommendation":null,"classification":null,"phenotypes":null,"activityscore":null,"population":null,"comments":null}), ["recommendationid", "lookupkey", "drugname", "guidelinename", "guidelineurl", "implications", "drugrecommendation", "classification", "phenotypes", "activityscore", "population", "comments"];
    PhenotypeAssociationCount => biodata::PhenotypeAssociationCount, json!({"key":"synthetic:1","category":"synthetic:1","label":null,"count":null,"count_direct":null,"count_with_orthologs":null}), ["key", "category", "label", "count", "count_direct", "count_with_orthologs"];
    PhenotypeHeader => biodata::PhenotypeHeader, json!({"id":"HP:0000001","label":null,"match_score":null,"matched_field":null,"match_type":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["id", "label", "match_score", "matched_field", "match_type", "attribution"];
    PhenotypeHierarchyLink => biodata::PhenotypeHierarchyLink, json!({"id":"synthetic:1","label":null,"relation":"super_classes","attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["id", "label", "relation", "attribution"];
    PhenotypeSynonym => biodata::PhenotypeSynonym, json!({"value":"synthetic text","relationship":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["value", "relationship", "attribution"];
    PhenotypeXref => biodata::PhenotypeXref, json!({"id":"synthetic:1","predicate":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["id", "predicate", "attribution"];
    PmaIdentity => biodata::PmaIdentity, json!({"pma_number":"P000001","supplement_number":null}), ["pma_number", "supplement_number"];
    PredictorScore => biodata::PredictorScore, json!({"gene":null,"transcript":null,"subject":{"level":"protein","gene":{"label":"SYNTH1"},"label":"p.A1T"},"method":"synthetic text","score":1,"label":null,"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["gene", "transcript", "subject", "method", "score", "label", "attribution"];
    Protein => biodata::Protein, json!({"accession":"P00001","name":null,"length":null,"function":[],"subcellular_locations":[]}), ["accession", "name", "length", "function", "subcellular_locations"];
    ProteinAudit => biodata::ProteinAudit, json!({"entry_version":null,"sequence_version":null,"first_public":null,"last_annotation_update":null,"last_sequence_update":null,"release":null}), ["entry_version", "sequence_version", "first_public", "last_annotation_update", "last_sequence_update", "release"];
    ProteinComplexHeader => biodata::ProteinComplexHeader, json!({"id":"CPX-1","name":null,"taxon":1,"predicted":null,"participant_total":1,"matched_participant":"synthetic text","url":"synthetic text"}), ["id", "name", "taxon", "predicted", "participant_total", "matched_participant", "url"];
    ProteinComplexMember => biodata::ProteinComplexMember, json!({"id":"synthetic text","participant_type":"synthetic text","taxon":null,"organism":null,"name":null,"stoichiometry":null,"xrefs":[],"protein":null}), ["id", "participant_type", "taxon", "organism", "name", "stoichiometry", "xrefs", "protein"];
    ProteinDomainFragment => biodata::ProteinDomainFragment, json!({"span":{"start":{"min":0,"max":0},"end":{"min":1,"max":1}},"dc_status":null}), ["span", "dc_status"];
    ProteinDomainLocation => biodata::ProteinDomainLocation, json!({"fragments":[]}), ["fragments"];
    ProteinDomainMatch => biodata::ProteinDomainMatch, json!({"id":"IPR000001","name":null,"entry_type":null,"protein":"P00001","protein_length":null,"taxon":null,"locations":[],"member_signatures":[],"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["id", "name", "entry_type", "protein", "protein_length", "taxon", "locations", "member_signatures", "attribution"];
    ProteinFeature => biodata::ProteinFeature, json!({"feature":{"protein":"P00001","span":{"start":{"min":0,"max":0},"end":{"min":1,"max":1}},"kind":{"label":"synthetic text","codings":[]},"description":null,"id":null,"original":null,"alternative":[],"evidence":[]},"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null},"source_coordinates":"one_based_inclusive"}), ["feature", "attribution", "source_coordinates"];
    ProteinFunction => biodata::ProteinFunction, json!({"molecule":null,"text":"synthetic text","evidence":[]}), ["molecule", "text", "evidence"];
    ProteinHeader => biodata::ProteinHeader, json!({"protein":{"accession":"P00001","name":null,"length":null,"function":[],"subcellular_locations":[]},"entry_id":null,"gene_names":[],"gene_ids":[],"taxon":1,"reviewed":false,"url":"synthetic text"}), ["protein", "entry_id", "gene_names", "gene_ids", "taxon", "reviewed", "url"];
    ProteinIsoform => biodata::ProteinIsoform, json!({"ids":[],"name":null,"synonyms":[],"sequence_status":null,"sequence_ids":[],"direct_sequence":"not_checked"}), ["ids", "name", "synonyms", "sequence_status", "sequence_ids", "direct_sequence"];
    ProteinMemberSignature => biodata::ProteinMemberSignature, json!({"namespace":"synthetic text","id":"synthetic text","name":null}), ["namespace", "id", "name"];
    ProteinSequence => biodata::ProteinSequence, json!({"accession":"P00001","sequence":"AC","length":1,"crc64":null,"md5":null,"audit":{"entry_version":null,"sequence_version":null,"first_public":null,"last_annotation_update":null,"last_sequence_update":null,"release":null}}), ["accession", "sequence", "length", "crc64", "md5", "audit"];
    ProteinStructureProperty => biodata::ProteinStructureProperty, json!({"key":"synthetic text","value":"synthetic text"}), ["key", "value"];
    ProteinStructureRef => biodata::ProteinStructureRef, json!({"namespace":"synthetic text","id":"synthetic text","url":null,"properties":[]}), ["namespace", "id", "url", "properties"];
    Representation => biodata::Representation, json!({"source":"synthetic text","expression":{"syntax":"hgvs.c","value":"NM_000001.1:c.1A>G"},"build":null,"allele":null}), ["source", "expression", "build", "allele"];
    SequenceLocation => biodata::SequenceLocation, json!({"sequence":"NC_000001.11","span":{"start":0,"end":1}}), ["sequence", "span"];
    Statement => biodata::Statement, json!({"method":null,"mode_of_inheritance":null,"criteria_not_met":[],"status":null,"subject":{"level":"gene","label":"SYNTH1"},"proposition":"pathogenicity","predicate":null,"direction":null,"classification":{"label":"synthetic text","codings":[]},"strength":null,"condition":null,"therapies":[],"allele_origin":null,"gene_context":null,"criteria":[],"citations":[],"attribution":{"approved":null,"published":null,"source":"synthetic text","record":null,"version":null,"release":null,"submitter":null,"evaluated":null,"url":null}}), ["method", "mode_of_inheritance", "criteria_not_met", "status", "subject", "proposition", "predicate", "direction", "classification", "strength", "condition", "therapies", "allele_origin", "gene_context", "criteria", "citations", "attribution"];
    StatisticEstimate => biodata::StatisticEstimate, json!({"value":null,"standard_error":null,"lower_ci":null,"upper_ci":null,"count":null}), ["value", "standard_error", "lower_ci", "upper_ci", "count"];
    StatisticSelector => biodata::StatisticSelector, json!({"name":"synthetic text","code":"synthetic text","label":null}), ["name", "code", "label"];
    Study => biodata::Study, json!({"header":{"studyId":"synthetic_study","name":null,"cancerTypeId":null,"allSampleCount":null,"pmids":null,"citation":null,"referenceGenome":null},"description":null,"sequencedSampleCount":null,"cnaSampleCount":null,"structuralVariantCount":null,"treatmentCount":null,"patientCount":null,"resourceCounts":[],"importDate":null,"groups":"synthetic text","readPermission":false,"publicStudy":false}), ["header", "description", "sequencedSampleCount", "cnaSampleCount", "structuralVariantCount", "treatmentCount", "patientCount", "resourceCounts", "importDate", "groups", "readPermission", "publicStudy"];
    StudyFile => biodata::StudyFile, json!({"name":"synthetic text","path":"synthetic text","htmlUrl":"synthetic text","downloadUrl":null,"sha":"0000000000000000000000000000000000000000","repositoryBlobSize":1,"kind":"file"}), ["name", "path", "htmlUrl", "downloadUrl", "sha", "repositoryBlobSize", "kind"];
    StudyHeader => biodata::StudyHeader, json!({"studyId":"synthetic_study","name":null,"cancerTypeId":null,"allSampleCount":null,"pmids":null,"citation":null,"referenceGenome":null}), ["studyId", "name", "cancerTypeId", "allSampleCount", "pmids", "citation", "referenceGenome"];
    StudyProfile => biodata::StudyProfile, json!({"molecularProfileId":"synthetic_profile","studyId":"synthetic_study","molecularAlterationType":null,"datatype":null,"name":null,"description":null,"genericAssayType":null,"patientLevel":null,"showProfileInAnalysisTab":null}), ["molecularProfileId", "studyId", "molecularAlterationType", "datatype", "name", "description", "genericAssayType", "patientLevel", "showProfileInAnalysisTab"];
    StudyResourceCount => biodata::StudyResourceCount, json!({"resourceId":"synthetic_resource","displayName":null,"description":null,"resourceType":null,"priority":null,"openByDefault":false,"cancerStudyIdentifier":"synthetic_study","sampleCount":null,"patientCount":null}), ["resourceId", "displayName", "description", "resourceType", "priority", "openByDefault", "cancerStudyIdentifier", "sampleCount", "patientCount"];
    StudySampleList => biodata::StudySampleList, json!({"sampleListId":"synthetic_list","studyId":"synthetic_study","category":null,"name":null,"description":null,"sampleCount":null}), ["sampleListId", "studyId", "category", "name", "description", "sampleCount"];
    Summary => biodata::Summary, json!({"text":"synthetic text","source":"refseq","dated":null}), ["text", "source", "dated"];
    Term => biodata::Term, json!({"label":"synthetic text","codings":[]}), ["label", "codings"];
    TrialArm => biodata::TrialArm, json!({"label":null,"kind":null,"description":null,"intervention_names":[]}), ["label", "kind", "description", "intervention_names"];
    TrialCard => biodata::TrialCard, json!({"nct_id":"NCT00000001","title":null,"status":null,"phases":[],"study_type":null,"conditions":[],"interventions":[],"sponsor":null,"start":null,"primary_completion":null,"completion":null,"updated":null,"updated_kind":null,"brief_summary":null,"url":"synthetic text"}), ["nct_id", "title", "status", "phases", "study_type", "conditions", "interventions", "sponsor", "start", "primary_completion", "completion", "updated", "updated_kind", "brief_summary", "url"];
    TrialContact => biodata::TrialContact, json!({"name":null,"role":null,"phone":null,"phone_ext":null,"email":null}), ["name", "role", "phone", "phone_ext", "email"];
    TrialDate => biodata::TrialDate, json!({"date":"2026-01-01","kind":null}), ["date", "kind"];
    TrialDocument => biodata::TrialDocument, json!({"filename":"synthetic text","url":"synthetic text","kind":null,"label":null,"date":null,"uploaded":null,"size":null,"has_protocol":null,"has_sap":null,"has_icf":null,"license":"cc_by"}), ["filename", "url", "kind", "label", "date", "uploaded", "size", "has_protocol", "has_sap", "has_icf", "license"];
    TrialEligibility => biodata::TrialEligibility, json!({"minimum_age":null,"maximum_age":null,"sex":null,"healthy_volunteers":null,"criteria":null}), ["minimum_age", "maximum_age", "sex", "healthy_volunteers", "criteria"];
    TrialIntervention => biodata::TrialIntervention, json!({"name":null,"kind":null,"description":null,"arm_group_labels":[],"other_names":[]}), ["name", "kind", "description", "arm_group_labels", "other_names"];
    TrialLocation => biodata::TrialLocation, json!({"facility":null,"city":null,"state":null,"zip":null,"country":null,"status":null,"coordinates":null,"contacts":[]}), ["facility", "city", "state", "zip", "country", "status", "coordinates", "contacts"];
    TrialOfficial => biodata::TrialOfficial, json!({"name":null,"affiliation":null,"role":null}), ["name", "affiliation", "role"];
    TrialOutcome => biodata::TrialOutcome, json!({"kind":"primary","measure":null,"description":null,"time_frame":null}), ["kind", "measure", "description", "time_frame"];
    TrialReference => biodata::TrialReference, json!({"pmid":null,"kind":null,"citation":null}), ["pmid", "kind", "citation"];
    UncertainLocation => biodata::UncertainLocation, json!({"sequence":"NC_000001.11","span":{"start":{"min":0,"max":0},"end":{"min":1,"max":1}}}), ["sequence", "span"];
    UniProtFeature => biodata::UniProtFeature, json!({"protein":"P00001","span":{"start":{"min":0,"max":0},"end":{"min":1,"max":1}},"kind":{"label":"synthetic text","codings":[]},"description":null,"id":null,"original":null,"alternative":[],"evidence":[]}), ["protein", "span", "kind", "description", "id", "original", "alternative", "evidence"];
    Variant => biodata::Variant, json!({"ids":[],"representations":[],"genes":[]}), ["ids", "representations", "genes"];
    WhoIvdProduct => biodata::WhoIvdProduct, json!({"id":"0001-001-00","native_rows":[]}), ["id", "native_rows"];
    WhoIvdRow => biodata::WhoIvdRow, json!({"product_name":"synthetic text","product_code":"synthetic text","assay_format":"synthetic text","manufacturer":"synthetic text","marker":"synthetic text","regulatory_version":"synthetic text","prequalification_year":"synthetic text"}), ["product_name", "product_code", "assay_format", "manufacturer", "marker", "regulatory_version", "prequalification_year"];
    WithUnit => biodata::WithUnit<i64>, json!({"value":1,"unit":null}), ["value", "unit"];
}
