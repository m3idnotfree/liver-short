use liver_shot::Value;

const JSON: &str = r#"
{
  "string": "x",
  "number": 1,
  "true": true,
  "false": false,
  "null": null,
  "object": {
    "k": 1
  },
  "array": [1]
}"#;

const OBJECT: &str = r#"{
    "k": 1
  }"#;

const KEYS: [&str; 7] = [
    "string", "number", "true", "false", "null", "object", "array",
];

fn value(key: &str) -> Value<'static> {
    liver_shot::find(key, JSON).unwrap().value(JSON)
}

#[test]
fn predicates() {
    for (kind, key) in [
        ("string", "string"),
        ("number", "number"),
        ("bool", "true"),
        ("bool", "false"),
        ("null", "null"),
        ("object", "object"),
        ("array", "array"),
    ] {
        let v = value(key);

        assert_eq!(kind == "string", v.is_string(), "{key}.is_string");
        assert_eq!(kind == "number", v.is_number(), "{key}.is_number");
        assert_eq!(kind == "bool", v.is_bool(), "{key}.is_bool");
        assert_eq!(kind == "null", v.is_null(), "{key}.is_null");
        assert_eq!(kind == "object", v.is_object(), "{key}.is_object");
        assert_eq!(kind == "array", v.is_array(), "{key}.is_array");
    }
}

#[test]
fn as_str() {
    assert_eq!(Some("x"), value("string").as_str());

    for key in KEYS {
        if key != "string" {
            assert_eq!(None, value(key).as_str(), "{key}.as_str");
        }
    }
}

#[test]
fn as_number() {
    assert_eq!(Some("1"), value("number").as_number());

    for key in KEYS {
        if key != "number" {
            assert_eq!(None, value(key).as_number(), "{key}.as_number");
        }
    }
}

#[test]
fn as_bool() {
    assert_eq!(Some(true), value("true").as_bool());
    assert_eq!(Some(false), value("false").as_bool());

    for key in KEYS {
        if key != "true" && key != "false" {
            assert_eq!(None, value(key).as_bool(), "{key}.as_bool");
        }
    }
}

#[test]
fn as_object() {
    assert_eq!(Some(OBJECT), value("object").as_object());

    for key in KEYS {
        if key != "object" {
            assert_eq!(None, value(key).as_object(), "{key}.as_object");
        }
    }
}

#[test]
fn as_array() {
    assert_eq!(Some("[1]"), value("array").as_array());

    for key in KEYS {
        if key != "array" {
            assert_eq!(None, value(key).as_array(), "{key}.as_array");
        }
    }
}

#[test]
fn empty_string_is_not_null() {
    let json = r#"{"a": ""}"#;
    let v = liver_shot::find("a", json).unwrap().value(json);

    assert_eq!(Some(""), v.as_str());
    assert!(!v.is_null());
}

#[test]
fn common_missunderstand_string() {
    for json in [
        r#"{"a": "1"}"#,
        r#"{"a": "true"}"#,
        r#"{"a": "false"}"#,
        r#"{"a": "null"}"#,
    ] {
        let v = liver_shot::find("a", json).unwrap().value(json);

        assert!(v.is_string(), "{json}");
        assert!(!v.is_null(), "{json}");
        assert_eq!(None, v.as_bool(), "{json}");
        assert_eq!(None, v.as_number(), "{json}");
    }
}

fn number<'a>(key: &'a str, data: &'a str) -> &'a str {
    liver_shot::find(key, data)
        .unwrap()
        .value(data)
        .as_number()
        .unwrap()
}

#[test]
fn parse_number() {
    let json = r#"{"a": 1, "b": -1}"#;

    assert_eq!(1, number("a", json).parse::<u8>().unwrap());
    assert_eq!(-1, number("b", json).parse::<i8>().unwrap());
}

#[test]
fn debug() {
    assert_eq!(r#"String("x")"#, format!("{:?}", value("string")));
    assert_eq!("Bool(true)", format!("{:?}", value("true")));
    assert_eq!("Null", format!("{:?}", value("null")));
}
