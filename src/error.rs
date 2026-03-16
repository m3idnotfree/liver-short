use core::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

pub struct Error {
    kind: Kind,
}

enum Kind {
    NotFound,
    InvalidJson,
    UnterminatedString,
    UnmatchedBracket,
    UnsupportedArray,
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self.kind, Kind::NotFound)
    }

    pub fn is_invalid_json(&self) -> bool {
        matches!(
            self.kind,
            Kind::InvalidJson | Kind::UnterminatedString | Kind::UnmatchedBracket
        )
    }

    pub fn is_unsupported_array(&self) -> bool {
        matches!(self.kind, Kind::UnsupportedArray)
    }

    pub(crate) fn not_found() -> Self {
        Self {
            kind: Kind::NotFound,
        }
    }

    pub(crate) fn invalid_json() -> Self {
        Self {
            kind: Kind::InvalidJson,
        }
    }

    pub(crate) fn unterminated_string() -> Self {
        Self {
            kind: Kind::UnterminatedString,
        }
    }

    pub(crate) fn unmatched_bracket() -> Self {
        Self {
            kind: Kind::UnmatchedBracket,
        }
    }

    pub(crate) fn unsupported_array() -> Self {
        Self {
            kind: Kind::UnsupportedArray,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[liver-shot] {}", self.kind)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl StdError for Error {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NotFound => f.write_str("field not found"),
            Self::InvalidJson => f.write_str("invalid JSON"),
            Self::UnterminatedString => f.write_str("unterminated string"),
            Self::UnmatchedBracket => f.write_str("unmatched bracket"),
            Self::UnsupportedArray => f.write_str("unsupported array"),
        }
    }
}
