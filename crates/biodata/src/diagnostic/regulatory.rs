use crate::{
    DiagnosticDeviceId, FdaDrugApplication, PmaNumber, PmaSupplement, PmaSupplementCell,
    WhoProductId,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Whole WHO listing, retaining each original product-code row in source order.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(inline))]
pub struct WhoIvdProduct {
    /// Native product listing identity.
    pub id: WhoProductId,
    /// All original rows belonging to this exact product, without clipping.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub native_rows: Box<[WhoIvdRow]>,
}
/// One original WHO CSV row; each repeated field retains its own row scope.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WhoIvdRow {
    /// Complete native product name cell; no interpretation or inferred date.
    pub product_name: Box<str>,
    /// Complete native product code cell; no interpretation or inferred date.
    pub product_code: Box<str>,
    /// Complete native assay format cell; no interpretation or inferred date.
    pub assay_format: Box<str>,
    /// Complete native manufacturer cell; no interpretation or inferred date.
    pub manufacturer: Box<str>,
    /// Complete native marker cell; no interpretation or inferred date.
    pub marker: Box<str>,
    /// Complete native regulatory version cell; no interpretation or inferred date.
    pub regulatory_version: Box<str>,
    /// Complete native prequalification year cell; no interpretation or inferred date.
    pub prequalification_year: Box<str>,
}
/// One FDA PMA record; an absent supplement differs from a native empty base cell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DeviceSubmission {
    /// Native base application.
    pub pma_number: PmaNumber,
    /// Native supplement cell; Some("") denotes the captured base, None denotes absent.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supplement_number: Option<PmaSupplementCell>,
    /// Native trade_name; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub trade_name: Option<Box<str>>,
    /// Native applicant; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub applicant: Option<Box<str>>,
    /// Native generic_name; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub generic_name: Option<Box<str>>,
    /// Native decision_date; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub decision_date: Option<Box<str>>,
    /// Native decision_code; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub decision_code: Option<Box<str>>,
    /// Native date_received; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_received: Option<Box<str>>,
    /// Native fed_reg_notice_date; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub fed_reg_notice_date: Option<Box<str>>,
    /// Native advisory_committee; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub advisory_committee: Option<Box<str>>,
    /// Native advisory_committee_description; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub advisory_committee_description: Option<Box<str>>,
    /// Native product_code; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub product_code: Option<Box<str>>,
    /// Native supplement_type; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supplement_type: Option<Box<str>>,
    /// Native supplement_reason; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supplement_reason: Option<Box<str>>,
    /// Native ao_statement; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub ao_statement: Option<Box<str>>,
    /// Native docket_number; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub docket_number: Option<Box<str>>,
    /// Native expedited_review_flag; explicit empty and absent remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub expedited_review_flag: Option<Box<str>>,
}
/// Exact executable PMA identity established by a companion row’s device cell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PmaIdentity {
    /// Native base application number.
    pub pma_number: PmaNumber,
    /// Explicit native supplement when present; no S000 alias.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub supplement_number: Option<PmaSupplement>,
}
/// Whole physical source-table cell and its own links.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticCell {
    /// Original complete text, including empty strings and line breaks.
    pub text: Box<str>,
    /// Only links physically owned by this cell, in native order.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub links: Box<[DiagnosticCellLink]>,
}
/// Source link without a name-based identity inference.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticCellLink {
    /// Original source URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri-reference")))]
    pub href: Box<str>,
    /// Native label when the capture supplies one.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<Box<str>>,
}
/// A typed drug application established by its own complete source cell and link.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticDrugApplication {
    /// Published NDA/BLA prefix and six digits.
    pub id: FdaDrugApplication,
    /// The same cell’s owned source URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Box<str>,
}
/// A physical companion-table row; the two table layouts cannot acquire each other’s cells.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum CompanionDiagnosticRelation {
    /// One complete native individual table row.
    Individual {
        /// Immutable local snapshot revision, not a publisher row ID.
        snapshot: Box<str>,
        /// Zero-based physical data-row index in its table.
        index: u32,
        /// Whole cells keyed by this table’s exact headers.
        cells: CompanionIndividualCells,
        /// Device facts from this row’s own dedicated device cell only.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        device_ids: Box<[DiagnosticDeviceId]>,
        /// Unambiguous typed applications owned by this row’s drug cell.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        drug_applications: Box<[DiagnosticDrugApplication]>,
    },
    /// One complete native group table row.
    Group {
        /// Immutable local snapshot revision, not a publisher row ID.
        snapshot: Box<str>,
        /// Zero-based physical data-row index in its table.
        index: u32,
        /// Whole cells keyed by this table’s exact headers.
        cells: CompanionGroupCells,
        /// Device facts from this row’s own dedicated device cell only.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        device_ids: Box<[DiagnosticDeviceId]>,
        /// Unambiguous typed applications owned by this row’s drug cell.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        drug_applications: Box<[DiagnosticDrugApplication]>,
    },
}
/// Known native individual-table cells keyed by their exact source headers.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CompanionIndividualCells {
    /// Complete original diagnostic cell.
    #[cfg_attr(feature = "serde", serde(rename = "Diagnostic Name (Manufacturer)"))]
    pub diagnostic: DiagnosticCell,
    /// Complete original indication cell.
    #[cfg_attr(feature = "serde", serde(rename = "Indication - Sample Type"))]
    pub indication: DiagnosticCell,
    /// Complete original drug cell.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Drug Trade Name (Generic) NDA / BLA")
    )]
    pub drug: DiagnosticCell,
    /// Complete original biomarker cell.
    #[cfg_attr(feature = "serde", serde(rename = "Biomarker(s)"))]
    pub biomarker: DiagnosticCell,
    /// Complete original details cell.
    #[cfg_attr(feature = "serde", serde(rename = "Biomarker(s) (Details)"))]
    pub details: DiagnosticCell,
    /// Complete original device cell.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "PMA / 510(k) /\n513(f)(2) / HDE (Approval / Clearance / Grant Date)")
    )]
    pub device: DiagnosticCell,
}
/// Known native group-table cells keyed by their exact source headers.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CompanionGroupCells {
    /// Complete original diagnostic cell.
    #[cfg_attr(feature = "serde", serde(rename = "Diagnostic Name\n(Manufacturer)"))]
    pub diagnostic: DiagnosticCell,
    /// Complete original indication cell.
    #[cfg_attr(feature = "serde", serde(rename = "Indication(s) - Sample Type"))]
    pub indication: DiagnosticCell,
    /// Complete original device cell.
    #[cfg_attr(feature = "serde", serde(rename = "PMA (Approval Date)"))]
    pub device: DiagnosticCell,
    /// Complete original drug_group cell.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA"
        )
    )]
    pub drug_group: DiagnosticCell,
}
