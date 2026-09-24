# liver-shot

[![Documentation](https://docs.rs/liver-shot/badge.svg)](https://docs.rs/liver-shot)

A lightweight, `no_std` JSON value position extractor with no allocation,
no `unsafe`, and no dependencies.

Returns a **`Span`** (start/end byte offsets) rather than a parsed value,
so the caller decides how to use the original string.

## Limitations

- Object navigation only - arrays at the top level or as a path step return `Error::is_unsupported_array`.
- Values are not parsed - `Span::get` returns the JSON text as written.
  Use `Span::value` to read it as a Rust type.

## Examples

### Extract an object

```rust
let json = r#"{"a": {"b": "value", "c": [1, 2, 3]}}"#;

let span = liver_shot::find("a", json)?;
assert_eq!(r#"{"b": "value", "c": [1, 2, 3]}"#, span.get(json));
```

### Extract a nested object

```rust
let json = r#"{"a": {"b": {"c": "value"}}}"#;

let span = liver_shot::find("a.b", json)?;
assert_eq!(r#"{"c": "value"}"#, span.get(json));
```

### Extract an array

```rust
let json = r#"{"a": {"b": "value", "c": [1, 2, 3]}}"#;

let span = liver_shot::find("a.c", json)?;
assert_eq!("[1, 2, 3]", span.get(json));
```

### Reuse `Span`

```rust
let json = r#"{"a": {"b": "value", "c": [1, 2, 3]}}"#;

let a = liver_shot::find("a", json)?;
let b   = a.find("b", json)?;
let c = a.find("c", json)?;

assert_eq!(r#"{"b": "value", "c": [1, 2, 3]}"#, a.get(json));
assert_eq!(r#""value""#, b.get(json));
assert_eq!("[1, 2, 3]", c.get(json));
```

### Read a value as a Rust type

```rust
let json = r#"{"type": null, "error": true, "expired": 1585484}"#;

let kind = liver_shot::find("type", json)?.value(json);
assert!(kind.is_null());

let error = liver_shot::find("error", json)?.value(json);
assert_eq!(Some(true), error.as_bool());

let expired = liver_shot::find("expired", json)?.value(json);
assert_eq!(1585484, expired.as_number().unwrap().parse::<u32>()?);
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
