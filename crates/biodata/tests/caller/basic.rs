fn main() {
    check_basic();
    assert_eq!(values::text!("  \u{2003}ready\u{a0} ").as_str(), "ready");
    assert_eq!("ready".parse::<CallerState>().unwrap().to_string(), "ready");
    assert_eq!("ABC".parse::<CallerText>().unwrap().to_string(), "ABC");
}
