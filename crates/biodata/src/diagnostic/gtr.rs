use crate::{Curie, GtrTestId, NcbiGeneId, Text};
#[cfg(feature = "serde")]
use serde::Serialize;
crate::serde_object::object_record! {
    /// One complete GTR test-version row; repeated identities remain separate records.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct GtrTestVersion {
        /// Complete accession and version.
        pub test_accession_ver: GtrTestId,
        /// Native `name_of_laboratory` cell; an empty string stays explicit and pipe values stay whole.
        pub name_of_laboratory: Box<str>,
        /// Native `name_of_institution` cell; an empty string stays explicit and pipe values stay whole.
        pub name_of_institution: Box<str>,
        /// Native `facility_state` cell; an empty string stays explicit and pipe values stay whole.
        pub facility_state: Box<str>,
        /// Native `facility_postcode` cell; an empty string stays explicit and pipe values stay whole.
        pub facility_postcode: Box<str>,
        /// Native `facility_country` cell; an empty string stays explicit and pipe values stay whole.
        pub facility_country: Box<str>,
        /// Native `CLIA_number` cell; an empty string stays explicit and pipe values stay whole.
        #[cfg_attr(feature = "serde", serde(rename = "CLIA_number"))]
        pub clia_number: Box<str>,
        /// Native `state_licenses` cell; an empty string stays explicit and pipe values stay whole.
        pub state_licenses: Box<str>,
        /// Native `state_license_numbers` cell; an empty string stays explicit and pipe values stay whole.
        pub state_license_numbers: Box<str>,
        /// Native `lab_test_id` cell; an empty string stays explicit and pipe values stay whole.
        pub lab_test_id: Box<str>,
        /// Native `last_touch_date` cell; an empty string stays explicit and pipe values stay whole.
        pub last_touch_date: Box<str>,
        /// Native `lab_test_name` cell; an empty string stays explicit and pipe values stay whole.
        pub lab_test_name: Box<str>,
        /// Native `manufacturer_test_name` cell; an empty string stays explicit and pipe values stay whole.
        pub manufacturer_test_name: Box<str>,
        /// Native `test_development` cell; an empty string stays explicit and pipe values stay whole.
        pub test_development: Box<str>,
        /// Native `lab_id` cell; an empty string stays explicit and pipe values stay whole.
        pub lab_id: Box<str>,
        /// Native `condition_identifiers` cell; an empty string stays explicit and pipe values stay whole.
        pub condition_identifiers: Box<str>,
        /// Native `indication_types` cell; an empty string stays explicit and pipe values stay whole.
        pub indication_types: Box<str>,
        /// Native `inheritances` cell; an empty string stays explicit and pipe values stay whole.
        pub inheritances: Box<str>,
        /// Native `method_categories` cell; an empty string stays explicit and pipe values stay whole.
        pub method_categories: Box<str>,
        /// Native `methods` cell; an empty string stays explicit and pipe values stay whole.
        pub methods: Box<str>,
        /// Native `platforms` cell; an empty string stays explicit and pipe values stay whole.
        pub platforms: Box<str>,
        /// Native `genes` cell; an empty string stays explicit and pipe values stay whole.
        pub genes: Box<str>,
        /// Native `drug_responses` cell; an empty string stays explicit and pipe values stay whole.
        pub drug_responses: Box<str>,
        /// Native `now_current` cell; an empty string stays explicit and pipe values stay whole.
        pub now_current: Box<str>,
        /// Native `test_currStat` cell; an empty string stays explicit and pipe values stay whole.
        #[cfg_attr(feature = "serde", serde(rename = "test_currStat"))]
        pub test_current: Box<str>,
        /// Native `test_pubStat` cell; an empty string stays explicit and pipe values stay whole.
        #[cfg_attr(feature = "serde", serde(rename = "test_pubStat"))]
        pub test_public: Box<str>,
        /// Native `lab_currStat` cell; an empty string stays explicit and pipe values stay whole.
        #[cfg_attr(feature = "serde", serde(rename = "lab_currStat"))]
        pub laboratory_current: Box<str>,
        /// Native `lab_pubStat` cell; an empty string stays explicit and pipe values stay whole.
        #[cfg_attr(feature = "serde", serde(rename = "lab_pubStat"))]
        pub laboratory_public: Box<str>,
        /// Native `test_create_date` cell; an empty string stays explicit and pipe values stay whole.
        pub test_create_date: Box<str>,
        /// Native `test_deletion_date` cell; an empty string stays explicit and pipe values stay whole.
        pub test_deletion_date: Box<str>,
        /// Native `test_order_code` cell; an empty string stays explicit and pipe values stay whole.
        pub test_order_code: Box<str>,
    }
}
crate::wire_enum! {
    /// Native GTR relation test category, not an offering or authorization judgment.
    GtrTestType { #[doc = "Native Clinical spelling."] Clinical => "Clinical",
 #[doc = "Native Research spelling."] Research => "Research",
 #[doc = "Native ClinicalMicrobial spelling."] ClinicalMicrobial => "Clinical microbial" }
}
crate::serde_object::object_enum! {
RelationFields, "kind", "relation", "snake_case";
/// Source-owned native membership or one independent method-list entry.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "kind", content = "relation", rename_all = "snake_case")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GtrTestRelation {
    /// Native gene relation row.
    Gene(GtrGeneRelation),
    /// Native condition relation row.
    Condition(GtrConditionRelation),
    /// One independent method-list entry.
    Method(GtrMethod),
}
}

crate::wire_enum! {
    /// Independent native GTR method lists.
    GtrMethodList { #[doc = "Native Categories spelling."] Categories => "method_categories",
 #[doc = "Native Methods spelling."] Methods => "methods",
 #[doc = "Native Platforms spelling."] Platforms => "platforms" }
}
crate::serde_object::object_record! {
    /// Native gene relation; its numeric ID comes only from the gene row.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct GtrGeneRelation {
        /// Complete owning test accession/version.
        pub test_id: GtrTestId,
        /// Native source test type.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub test_type: Option<GtrTestType>,
        /// Shared source-given GeneID and gene_symbol; absent GeneID stays absent.
        pub gene: GtrGeneReference,
        /// Uninterpreted nonempty `gene_or_SNOMED_CT_ID` when it is not a checked NCBI Gene ID.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub native_gene_or_snomed_ct_id: Option<Box<str>>,
        /// Independent native MedGen identity from GTR_identifier.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub native_medgen: Option<Curie>,
        /// Native MIM cell, including empty.
        pub mim_number: Box<str>,
        /// Full source object_name without interpretation.
        pub object_name: Box<str>,
    }
}
crate::serde_object::object_record! {
    /// Native condition membership; SNOMED cells never become GeneID.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct GtrConditionRelation {
        /// Complete owning test accession/version.
        pub test_id: GtrTestId,
        /// Native source test type.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub test_type: Option<GtrTestType>,
        /// Shared MedGen concept with source label.
        pub condition: GtrConditionReference,
        /// Native MIM cell, including empty.
        pub mim_number: Box<str>,
        /// Native SNOMED CT cell, including empty; never a gene ID.
        pub snomed_ct_id: Box<str>,
    }
}
crate::serde_object::object_record! {
    /// Independently retained native gene target fields; mapping requires at least one usable key.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct GtrGeneReference {
        /// Source gene symbol, absent when only the source GeneID survives.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Text>,
        /// Native GeneID from a gene relation, never a condition's SNOMED field.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub ncbi_gene_id: Option<NcbiGeneId>,
    }
}
impl GtrGeneReference {
    /// Source-stated numeric gene identity.
    pub fn ncbi_gene_id(&self) -> Option<NcbiGeneId> {
        self.ncbi_gene_id
    }
}
crate::serde_object::object_record! {
    /// A checked native MedGen target with independently optional source label.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct GtrConditionReference {
        /// Native MedGen identity from the condition relation.
        pub id: Curie,
        /// Native source label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Text>,
    }
}
crate::serde_object::object_record! {
    /// One entry from one independent native method list.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct GtrMethod {
        /// Complete owning test accession/version.
        pub test_id: GtrTestId,
        /// Independent source list; lists are never positionally paired.
        pub list: GtrMethodList,
        /// Zero-based native list position.
        pub index: u32,
        /// Complete unmodified source entry.
        pub value: Box<str>,
    }
}
