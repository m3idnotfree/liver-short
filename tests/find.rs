#[test]
fn string_value() {
    let json = r#"{"a": "1", "b": "2"}"#;
    assert_eq!("\"1\"", liver_shot::find("a", json).unwrap().get(json));
}

#[test]
fn number_value() {
    let json = r#"{"a": 1, "b": "2"}"#;
    assert_eq!("1", liver_shot::find("a", json).unwrap().get(json));
}

#[test]
fn boolean_value() {
    let json = r#"{"a": true, "b": false}"#;
    assert_eq!("true", liver_shot::find("a", json).unwrap().get(json));
    assert_eq!("false", liver_shot::find("b", json).unwrap().get(json));
}

#[test]
fn null_value() {
    let json = r#"{"a": null}"#;
    assert_eq!("null", liver_shot::find("a", json).unwrap().get(json));
}

#[test]
fn object_value() {
    let json = r#"{"a": {"b": "2", "c": "3"}, "d": "4"}"#;
    assert_eq!(
        r#"{"b": "2", "c": "3"}"#,
        liver_shot::find("a", json).unwrap().get(json)
    );
}

#[test]
fn array_value() {
    let json = r#"{"a": ["1", "2"], "b": "3"}"#;
    assert_eq!(
        r#"["1", "2"]"#,
        liver_shot::find("a", json).unwrap().get(json)
    );
}

#[test]
fn nested() {
    let json = r#"{"a": "1", "b": {"c": "3", "d": "4"}}"#;
    assert_eq!("\"3\"", liver_shot::find("b.c", json).unwrap().get(json));
}

#[test]
fn nested_skip_first() {
    let json = r#"{"a": "1", "b": {"c": "3", "d": "4"}}"#;
    assert_eq!("\"4\"", liver_shot::find("b.d", json).unwrap().get(json));
}

#[test]
fn deep() {
    let json = r#"{"a": {"b": {"c": "3"}}}"#;
    assert_eq!("\"3\"", liver_shot::find("a.b.c", json).unwrap().get(json));
}

#[test]
fn span_find() {
    let json = r#"{"a": {"b": "2", "c": "3"}}"#;
    let a = liver_shot::find("a", json).unwrap();
    let b = a.find("b", json).unwrap();
    let c = a.find("c", json).unwrap();
    assert_eq!(r#"{"b": "2", "c": "3"}"#, a.get(json));
    assert_eq!("\"2\"", b.get(json));
    assert_eq!("\"3\"", c.get(json));
}

#[test]
fn span_find_chain() {
    let json = r#"{"a": {"b": {"c": "3"}}}"#;
    let span = liver_shot::find("a", json)
        .unwrap()
        .find("b", json)
        .unwrap()
        .find("c", json)
        .unwrap();
    assert_eq!("\"3\"", span.get(json));
}

#[test]
fn missing_field() {
    let json = r#"{"a": "1"}"#;
    let err = liver_shot::find("b", json).unwrap_err();
    assert!(err.is_not_found());
}

#[test]
fn missing_outer() {
    let json = r#"{"a": {"inner": "value"}}"#;
    let err = liver_shot::find("outer.inner", json).unwrap_err();
    assert!(err.is_not_found());
}

#[test]
fn missing_inner() {
    let json = r#"{"outer": {"a": "value"}}"#;
    let err = liver_shot::find("outer.inner", json).unwrap_err();
    assert!(err.is_not_found());
}

#[test]
fn invalid_json() {
    let json = r#"{"a": "1}"#;
    let err = liver_shot::find("a", json).unwrap_err();
    assert!(err.is_invalid_json());
}

#[test]
fn invalid_primitive() {
    let json = r#"{"a": 12 3}"#;
    let err = liver_shot::find("a", json).unwrap_err();
    assert!(err.is_invalid_json());
}
