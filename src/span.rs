#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn get<'a>(&self, data: &'a str) -> &'a str {
        &data[self.start..self.end]
    }

    pub fn find(&self, pattern: &str, data: &str) -> Result<Span, crate::Error> {
        let slice = &data.as_bytes()[self.start..self.end];
        let result = crate::parser::find_path(slice, pattern)?;
        Ok(Span::new(
            self.start + result.start,
            self.start + result.end,
        ))
    }
}
