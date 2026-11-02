use super::ReportId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Independent overlapping report-level flags; no CTCAE or reaction-outcome conversion.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FaersSeriousness {
    /// Native death string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub death: Option<Box<str>>,
    /// Native lifethreatening string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub lifethreatening: Option<Box<str>>,
    /// Native hospitalization string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub hospitalization: Option<Box<str>>,
    /// Native disabling string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub disabling: Option<Box<str>>,
    /// Native congenitalanomali string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub congenitalanomali: Option<Box<str>>,
    /// Native other string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub other: Option<Box<str>>,
}
/// Native patient age and independently supplied sex and weight fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FaersPatient {
    /// Native numeric age with the exact publisher unit code, when stated.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub age: Option<crate::WithUnit<crate::FiniteNumber, super::FaersAgeUnit>>,
    /// Native sex string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sex: Option<Box<str>>,
    /// Native weight string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub weight: Option<Box<str>>,
}
/// Source reporter fields; qualification may be absent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FaersReporter {
    /// Native qualification string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub qualification: Option<Box<str>>,
    /// Native country string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub country: Option<Box<str>>,
}
/// Only the seven reviewed harmonized identifier collections; no administered-product inference.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
// Preserve Some([]) in retained native snapshots; consumer pages disclose counts and rows.
pub struct FaersIdentifiers {
    /// Ordered native unii strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub unii: Option<Box<[Box<str>]>>,
    /// Ordered native rxcui strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub rxcui: Option<Box<[Box<str>]>>,
    /// Ordered native product_ndc strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub product_ndc: Option<Box<[Box<str>]>>,
    /// Ordered native package_ndc strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub package_ndc: Option<Box<[Box<str>]>>,
    /// Ordered native spl_id strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub spl_id: Option<Box<[Box<str>]>>,
    /// Ordered native spl_set_id strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub spl_set_id: Option<Box<[Box<str>]>>,
    /// Ordered native application_number strings; missing differs from empty.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub application_number: Option<Box<[Box<str>]>>,
}
/// One independent native drug row; no join to a reaction row.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FaersDrug {
    /// Native medicinalproduct string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub medicinalproduct: Option<Box<str>>,
    /// Native drugcharacterization string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugcharacterization: Option<Box<str>>,
    /// Native drugindication string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugindication: Option<Box<str>>,
    /// Native drugadministrationroute string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugadministrationroute: Option<Box<str>>,
    /// Native drugauthorizationnumb string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugauthorizationnumb: Option<Box<str>>,
    /// Native drugbatchnumb string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugbatchnumb: Option<Box<str>>,
    /// Native drugcumulativedosagenumb string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugcumulativedosagenumb: Option<Box<str>>,
    /// Native drugcumulativedosageunit string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugcumulativedosageunit: Option<Box<str>>,
    /// Native drugdosageform string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugdosageform: Option<Box<str>>,
    /// Native drugdosagetext string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugdosagetext: Option<Box<str>>,
    /// Native drugenddate string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugenddate: Option<Box<str>>,
    /// Native drugenddateformat string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugenddateformat: Option<Box<str>>,
    /// Native drugintervaldosagedefinition string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugintervaldosagedefinition: Option<Box<str>>,
    /// Native drugintervaldosageunitnumb string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugintervaldosageunitnumb: Option<Box<str>>,
    /// Native drugrecurreadministration string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugrecurreadministration: Option<Box<str>>,
    /// Native drugseparatedosagenumb string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugseparatedosagenumb: Option<Box<str>>,
    /// Native drugstartdate string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugstartdate: Option<Box<str>>,
    /// Native drugstartdateformat string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugstartdateformat: Option<Box<str>>,
    /// Native drugstructuredosagenumb string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugstructuredosagenumb: Option<Box<str>>,
    /// Native drugstructuredosageunit string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugstructuredosageunit: Option<Box<str>>,
    /// Native drugtreatmentduration string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugtreatmentduration: Option<Box<str>>,
    /// Native drugtreatmentdurationunit string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugtreatmentdurationunit: Option<Box<str>>,
    /// Native actiondrug string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub actiondrug: Option<Box<str>>,
    /// Native drugadditional string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub drugadditional: Option<Box<str>>,
    /// Reviewed harmonized collections owned by this drug row.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub openfda: Option<FaersIdentifiers>,
}
/// Native reaction term with separate vocabulary version and outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FaersReaction {
    /// Native label without an invented CURIE.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub term: Option<crate::Concept>,
    /// Native vocabulary_version string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub vocabulary_version: Option<Box<str>>,
    /// Native outcome string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub outcome: Option<Box<str>>,
}
/// Neutral kept FAERS facts with independent ordered drug and reaction arrays.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FaersReport {
    /// Opaque published report identifier.
    pub safetyreportid: ReportId,
    /// Native safetyreportversion string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub safetyreportversion: Option<Box<str>>,
    /// Native receivedate string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub receivedate: Option<Box<str>>,
    /// Native receivedateformat string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub receivedateformat: Option<Box<str>>,
    /// Native receiptdate string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub receiptdate: Option<Box<str>>,
    /// Native receiptdateformat string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub receiptdateformat: Option<Box<str>>,
    /// Native transmissiondate string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub transmissiondate: Option<Box<str>>,
    /// Native transmissiondateformat string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub transmissiondateformat: Option<Box<str>>,
    /// Native serious string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub serious: Option<Box<str>>,
    /// Independent report-level native flags.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub seriousness: Option<FaersSeriousness>,
    /// Native patient fields.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub patient: Option<FaersPatient>,
    /// Native primary-source reporter fields.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub reporter: Option<FaersReporter>,
    /// Complete ordered source drug array.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub drugs: Box<[FaersDrug]>,
    /// Complete ordered source reaction array.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub reactions: Box<[FaersReaction]>,
}
