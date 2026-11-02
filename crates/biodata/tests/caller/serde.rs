fn main() {
    check_basic();
    assert_eq!(serde_json::to_string(&CallerState::Ready).unwrap(), "\"ready\"");
    assert_eq!(serde_json::from_str::<CallerState>("\"waiting\"").unwrap(), CallerState::Waiting);
    assert_eq!(serde_json::to_string(&"ABC".parse::<CallerText>().unwrap()).unwrap(), "\"ABC\"");
    assert_eq!(serde_json::from_str::<CallerText>("\"ABC\"").unwrap().to_string(), "ABC");
    assert!(serde_json::from_str::<CallerText>("\"bad\"").is_err());
    assert_eq!(serde_json::to_string(&CallerWire::Ready).unwrap(), "\"ready\"");
    assert_eq!(serde_json::from_str::<CallerDescribed>("\"waiting\"").unwrap(), CallerDescribed::Waiting);
    for invalid in ["true", "1", "null", "[]", "{}", "\"unknown\""] {
        assert!(serde_json::from_str::<CallerWire>(invalid).unwrap_err().to_string().starts_with("wire_enum.unknown"));
    }
    assert_eq!(serde_json::from_str::<CallerNative>("\"new value\"").unwrap().as_str(), "new value");
    assert_eq!(serde_json::to_string(&CallerNative::Ready).unwrap(), "\"ready\"");
    assert_eq!(serde_json::to_string(&CallerId::new("ID").unwrap()).unwrap(), "\"ID\"");
    assert!(serde_json::from_str::<CallerId>("\"bad\"").is_err());
}
