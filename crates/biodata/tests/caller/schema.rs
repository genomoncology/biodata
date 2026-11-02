fn main() {
    check_basic();
    let state = schema::schema_for!(CallerState).to_value();
    assert_eq!(state["x-caller"], true);
    assert_eq!(state["title"], "CallerState");
    assert_eq!(state["enum"], serde_json::json!(["ready", "waiting"]));
    assert_eq!(state["x-enum-descriptions"], serde_json::json!([" A declared ready state.", " A declared waiting state."]));
    assert_eq!(schema::schema_for!(CallerText).to_value()["pattern"], "^[A-Z]+$");
    assert_eq!(serde_json::to_string(&CallerState::Ready).unwrap(), "\"ready\"");
    assert_eq!(schema::schema_for!(CallerWire).to_value()["x-enum-open"], true);
    assert_eq!(schema::schema_for!(CallerWire).to_value()["enum"], serde_json::json!(["ready", "waiting"]));
    assert_eq!(schema::schema_for!(CallerDescribed).to_value()["x-enum-descriptions"], serde_json::json!([" A ready wire state.", " A waiting wire state."]));
    assert_eq!(schema::schema_for!(CallerNative).to_value()["type"], "string");
    assert_eq!(schema::schema_for!(CallerId).to_value()["pattern"], "^ID$");
    assert_eq!(serde_json::to_string(&CallerWire::Ready).unwrap(), "\"ready\"");
}
