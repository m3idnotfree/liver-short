use crate::{Error, Span};

#[derive(Clone, Copy)]
pub struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(bytes: &'a [u8], pos: usize) -> Self {
        Self { bytes, pos }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn peek(&self) -> Option<u8> {
        if self.pos < self.bytes.len() {
            Some(self.bytes[self.pos])
        } else {
            None
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn eat(&mut self, b: u8) -> bool {
        let found = self.peek() == Some(b);
        if found {
            self.advance();
        }
        found
    }

    pub fn slice(&self, start: usize) -> &'a [u8] {
        &self.bytes[start..self.pos]
    }
}

impl Cursor<'_> {
    pub fn skip_value(&mut self) -> Result<(), Error> {
        match self.peek() {
            Some(b'"') => self.skip_string(),
            Some(b'{') => self.skip_object(),
            Some(b'[') => self.skip_array(),
            Some(b't') => self.skip_literal(b"true"),
            Some(b'f') => self.skip_literal(b"false"),
            Some(b'n') => self.skip_literal(b"null"),
            Some(_) => self.skip_primitive(),
            None => Err(Error::invalid_json()),
        }
    }

    fn skip_object(&mut self) -> Result<(), Error> {
        // `{`
        self.advance();
        let mut depth = 1;
        while depth > 0 {
            match self.peek() {
                Some(b'"') => self.skip_string()?,
                Some(b'[') => self.skip_array()?,
                Some(b'{') => {
                    depth += 1;
                    self.advance();
                }
                Some(b'}') => {
                    depth -= 1;
                    self.advance();
                }
                Some(_) => self.advance(),
                None => return Err(Error::unmatched_bracket()),
            }
        }
        Ok(())
    }

    fn skip_array(&mut self) -> Result<(), Error> {
        // `[`
        self.advance();
        let mut depth = 1;
        while depth > 0 {
            match self.peek() {
                Some(b'"') => self.skip_string()?,
                Some(b'[') => {
                    depth += 1;
                    self.advance();
                }
                Some(b']') => {
                    depth -= 1;
                    self.advance();
                }
                Some(_) => self.advance(),
                None => return Err(Error::unmatched_bracket()),
            }
        }
        Ok(())
    }

    pub fn skip_string(&mut self) -> Result<(), Error> {
        // '"'
        self.advance();
        while let Some(b) = self.peek() {
            match b {
                b'"' => {
                    self.advance();
                    return Ok(());
                }
                b'\\' => self.pos += 2, // skip escaped character
                _ => self.advance(),
            }
        }
        Err(Error::unterminated_string())
    }

    fn skip_literal(&mut self, word: &[u8]) -> Result<(), Error> {
        let end = self.pos + word.len();
        if self.bytes.get(self.pos..end) == Some(word) {
            self.pos = end;
            Ok(())
        } else {
            Err(Error::invalid_json())
        }
    }

    fn skip_primitive(&mut self) -> Result<(), Error> {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if matches!(b, b',' | b']' | b'}') || is_whitespace(b) {
                break;
            }
            self.advance();
        }

        if self.pos == start {
            Err(Error::invalid_json())
        } else {
            Ok(())
        }
    }

    pub fn skip_whitespace(&mut self) -> Option<u8> {
        while let Some(b) = self.peek() {
            if !is_whitespace(b) {
                return Some(b);
            }
            self.advance();
        }
        None
    }
}

impl Cursor<'_> {
    pub fn find(&mut self, pattern: &str) -> Result<Span, Error> {
        for key in pattern.split('.') {
            match self.peek() {
                Some(b'{') => self.find_value(key)?,
                Some(b'[') => return Err(Error::unsupported_array()),
                Some(_) => return Err(Error::not_found()),
                None => return Err(Error::invalid_json()),
            }
        }

        let start = self.pos();
        self.skip_value()?;
        let end = self.pos();

        match self.skip_whitespace() {
            Some(b',' | b'}') => Ok(Span::new(start, end)),
            _ => Err(Error::invalid_json()),
        }
    }

    fn find_value(&mut self, key: &str) -> Result<(), Error> {
        // `{`
        self.advance();
        if self.skip_whitespace() == Some(b'}') {
            return Err(Error::not_found());
        }

        loop {
            // key
            if self.skip_whitespace() != Some(b'"') {
                return Err(Error::invalid_json());
            }

            let start = self.pos();
            self.skip_string()?;
            let field = self.slice(start); // quotes included
            let is_match = &field[1..field.len() - 1] == key.as_bytes();

            // `:`
            self.skip_whitespace();
            if !self.eat(b':') {
                return Err(Error::invalid_json());
            }

            // move to the value
            self.skip_whitespace();
            if is_match {
                return Ok(());
            }

            // not this key; try the next one
            self.skip_value()?;

            // `,` or `}`
            match self.skip_whitespace() {
                Some(b',') => self.advance(),
                Some(b'}') => return Err(Error::not_found()),
                _ => return Err(Error::invalid_json()),
            }
        }
    }
}

fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\r' | b'\t')
}
