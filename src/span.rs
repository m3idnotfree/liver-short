use core::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the JSON text this span covers.
    ///
    /// Use [`Self::value`] to read it as a Rust type.
    pub fn get<'a>(&self, data: &'a str) -> &'a str {
        &data[self.start..self.end]
    }

    pub fn find(&self, pattern: &str, data: &str) -> Result<Span, crate::Error> {
        crate::cursor::Cursor::new(data.as_bytes(), self.start).find(pattern)
    }

    /// Converts the text [`Self::get`] returns into a Rust type.
    pub fn value<'a>(&self, data: &'a str) -> Value<'a> {
        let slice = self.get(data);
        Value::classify(slice)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Span")
            .field(&(self.start..self.end))
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value<'a> {
    String(&'a str),
    Number(&'a str),
    Object(&'a str),
    Array(&'a str),
    Bool(bool),
    Null,
}

impl<'a> Value<'a> {
    fn classify(data: &'a str) -> Self {
        match data.as_bytes().first() {
            Some(b'"') => Self::String(&data[1..data.len() - 1]),
            Some(b'{') => Self::Object(data),
            Some(b'[') => Self::Array(data),
            _ => match data {
                "true" => Self::Bool(true),
                "false" => Self::Bool(false),
                "null" => Self::Null,
                _ => Self::Number(data),
            },
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match *self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&'a str> {
        match *self {
            Self::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&'a str> {
        match *self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&'a str> {
        match *self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }
}
