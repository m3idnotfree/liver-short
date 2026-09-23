use rand::Rng;

fn number<T>(text: impl Into<String>) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let json = format!(r#"{{"a": {}}}"#, text.into());
    let span = liver_shot::find("a", &json).unwrap();

    span.value(&json).as_number().unwrap().parse().unwrap()
}

#[test]
fn float() {
    let mut rng = rand::rng();

    for _ in 0..1_000 {
        let value: f64 = rng.random_range(-1e6..1e6);
        assert_eq!(value, number(value.to_string()));
    }
}

#[test]
fn int() {
    let mut rng = rand::rng();

    for _ in 0..1_000 {
        let value: i64 = rng.random();
        assert_eq!(value, number(value.to_string()));
    }
}

#[test]
fn exp() {
    let mut rng = rand::rng();

    for _ in 0..1_000 {
        let value: f64 = rng.random_range(-1e6..1e6);
        assert_eq!(value, number(format!("{value:e}")));
    }
}
