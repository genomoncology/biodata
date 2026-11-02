//! adverse-event-types: caller-owned no-I/O report contracts.
use biodata::{FaersReport, MaudeReport};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
fn roundtrip<T: Serialize + DeserializeOwned>(value: Value) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(value)?)
}
#[test]
fn report_records_preserve_array_order_empty_strings_and_missing_reporter_qualification() {
    let drugs: Vec<_> = (0..19)
        .map(|n| json!({"medicinalproduct":format!("synthetic-{n}"),"openfda":{"unii":["A","B"]}}))
        .collect();
    let reactions: Vec<_> = (0..22)
        .map(|n| json!({"term":{"label":format!("synthetic reaction {n}")},"outcome":"1"}))
        .collect();
    let report = json!(
        {"safetyreportid":"26940567","safetyreportversion":"1","receivedate":"20240101","receiptdate":"20240201","seriousness":{"death":"1",
        "hospitalization":"1"},"reporter":{"country":""},"patient":{"age":{"value":42.5,"unit":"801"}},"drugs":drugs,"reactions":reactions}
    );
    assert_eq!(roundtrip::<FaersReport>(report.clone()).unwrap(), report);
    for length in [0, 5] {
        let narrative: Vec<_> = (0..length)
            .map(|n| json!({"text_type_code":"N","text":format!("Synthetic narrative {n}")}))
            .collect();
        let mut device = json!({"mdr_report_key":"50952244","devices":[{"brand_name":""},{"brand_name":"synthetic"}],"narrative":narrative});
        if length == 0 {
            device.as_object_mut().unwrap().remove("narrative");
        }
        assert_eq!(roundtrip::<MaudeReport>(device.clone()).unwrap(), device);
    }
}
