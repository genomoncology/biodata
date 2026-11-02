use super::ReportId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Native device facts; administrative addresses are outside the reviewed projection.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MaudeDevice {
    /// Native device_sequence_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_sequence_number: Option<Box<str>>,
    /// Native brand_name string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub brand_name: Option<Box<str>>,
    /// Native generic_name string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub generic_name: Option<Box<str>>,
    /// Native manufacturer_d_name string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub manufacturer_d_name: Option<Box<str>>,
    /// Native device_report_product_code string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_report_product_code: Option<Box<str>>,
    /// Native model_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub model_number: Option<Box<str>>,
    /// Native catalog_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub catalog_number: Option<Box<str>>,
    /// Native udi_di string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub udi_di: Option<Box<str>>,
    /// Native udi_public string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub udi_public: Option<Box<str>>,
    /// Native lot_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub lot_number: Option<Box<str>>,
    /// Native device_operator string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_operator: Option<Box<str>>,
    /// Native implant_flag string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub implant_flag: Option<Box<str>>,
    /// Native implant_date_year string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub implant_date_year: Option<Box<str>>,
    /// Native date_removed_flag string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_removed_flag: Option<Box<str>>,
    /// Native date_removed_year string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_removed_year: Option<Box<str>>,
    /// Native device_age_text string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_age_text: Option<Box<str>>,
    /// Native expiration_date_of_device string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub expiration_date_of_device: Option<Box<str>>,
    /// Native device_availability string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_availability: Option<Box<str>>,
    /// Native device_evaluated_by_manufacturer string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_evaluated_by_manufacturer: Option<Box<str>>,
    /// Native device_event_key string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub device_event_key: Option<Box<str>>,
    /// Native date_received string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_received: Option<Box<str>>,
    /// Native other_id_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub other_id_number: Option<Box<str>>,
    /// Native combination_product_flag string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub combination_product_flag: Option<Box<str>>,
    /// Native serviced_by_3rd_party_flag string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub serviced_by_3rd_party_flag: Option<Box<str>>,
}
/// One unchanged native narrative; never truncate an oversized row.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MaudeNarrative {
    /// Native mdr_text_key string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub mdr_text_key: Option<Box<str>>,
    /// Native patient_sequence_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub patient_sequence_number: Option<Box<str>>,
    /// Native text_type_code string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub text_type_code: Option<Box<str>>,
    /// Native text string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub text: Option<Box<str>>,
}
/// Kept MAUDE facts; native count strings remain separate from array lengths.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MaudeReport {
    /// Opaque native report key.
    pub mdr_report_key: ReportId,
    /// Native report_number string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub report_number: Option<Box<str>>,
    /// Native event_type string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub event_type: Option<Box<str>>,
    /// Native date_of_event string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_of_event: Option<Box<str>>,
    /// Native date_received string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_received: Option<Box<str>>,
    /// Native report_date string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub report_date: Option<Box<str>>,
    /// Native date_report string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_report: Option<Box<str>>,
    /// Native date_report_to_fda string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub date_report_to_fda: Option<Box<str>>,
    /// Native number_devices_in_event string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub number_devices_in_event: Option<Box<str>>,
    /// Native number_patients_in_event string, including a present empty value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub number_patients_in_event: Option<Box<str>>,
    /// Ordered native device array.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub devices: Box<[MaudeDevice]>,
    /// Ordered native narrative array, including an empty array.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub narrative: Box<[MaudeNarrative]>,
}
