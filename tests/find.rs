use liver_shot::Value;

#[test]
fn string_value() {
    let json = r#"{"a": "1", "b": "2"}"#;
    let span = liver_shot::find("a", json).unwrap();

    assert_eq!("\"1\"", span.get(json));
    assert_eq!(Value::String("1"), span.value(json));
}

#[test]
fn number_value() {
    let json = r#"{"a": 1, "b": "2"}"#;
    let span = liver_shot::find("a", json).unwrap();

    assert_eq!("1", span.get(json));
    assert_eq!(Value::Number("1"), span.value(json));
}

#[test]
fn boolean_value() {
    let json = r#"{"a": true, "b": false}"#;
    let a = liver_shot::find("a", json).unwrap();
    let b = liver_shot::find("b", json).unwrap();

    assert_eq!("true", a.get(json));
    assert_eq!("false", b.get(json));
    assert_eq!(Value::Bool(true), a.value(json));
    assert_eq!(Value::Bool(false), b.value(json));
}

#[test]
fn null_value() {
    let json = r#"{"a": null}"#;
    let span = liver_shot::find("a", json).unwrap();

    assert_eq!("null", span.get(json));
    assert_eq!(Value::Null, span.value(json));
}

#[test]
fn object_value() {
    let json = r#"{"a": {"b": "2", "c": "3"}, "d": "4"}"#;
    let span = liver_shot::find("a", json).unwrap();

    assert_eq!(r#"{"b": "2", "c": "3"}"#, span.get(json));
    assert_eq!(Value::Object(r#"{"b": "2", "c": "3"}"#), span.value(json));
}

#[test]
fn array_value() {
    let json = r#"{"a": ["1", "2"], "b": "3"}"#;
    let span = liver_shot::find("a", json).unwrap();

    assert_eq!(r#"["1", "2"]"#, span.get(json));
    assert_eq!(Value::Array(r#"["1", "2"]"#), span.value(json));
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

// PR #1 (https://github.com/m3idnotfree/liver-short/pull/1)
// Claimed `\u{` was counted as an opening brace in `scan_string`.
// It is not: `scan_string` branches only on `"` and `\`, so a `{` inside a
// string never reaches the `depth` counter in `scan_object`.
// They pass without the patch.
#[test]
fn pr_1_hex_unicode() {
    let json = r#"{"message": "Hello \u{1F600} World"}"#;
    let span = liver_shot::find("message", json).unwrap();
    assert_eq!(r#""Hello \u{1F600} World""#, span.get(json));

    let json = r#"{"message": "Hello A World"}"#;
    let span = liver_shot::find("message", json).unwrap();
    assert_eq!(r#""Hello A World""#, span.get(json));

    let json = r#"{"template": "Hello {name}!"}"#;
    let span = liver_shot::find("template", json).unwrap();
    assert_eq!(r#""Hello {name}!""#, span.get(json));
}

// PR #1 (https://github.com/m3idnotfree/liver-short/pull/1)
// Escape handling is deliberately loose: `scan_string` skips exactly one
// byte after a backslash, then reads on until an unescaped `"`.
// So the digit count after `\u` never matters and the escape is not validated.
#[test]
fn flexible_escape() {
    let json = r#"{"message": "Hello \u1F600 World"}"#;
    let span = liver_shot::find("message", json).unwrap();
    assert_eq!(r#""Hello \u1F600 World""#, span.get(json));

    let json = r#"{"message": "\u00"}"#;
    let span = liver_shot::find("message", json).unwrap();
    assert_eq!(r#""\u00""#, span.get(json));
}

#[test]
fn not_found() {
    for (json, path) in [
        ("1", "a"),
        (r#""a""#, "a"),
        (r#"{"a": 1}"#, "a.b"),
        (r#"{"a": { "b": true}}"#, "a.b.c"),
    ] {
        let err = liver_shot::find(path, json).unwrap_err();
        assert!(err.is_not_found(), "{json} {path}");
    }
}
