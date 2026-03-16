use crate::{
    scanner::{next_quote, scan_array, scan_object, scan_primitive, scan_string, skip_whitespace},
    Error, Span,
};

pub fn find_path(bytes: &[u8], pattern: &str) -> Result<Span, Error> {
    let mut abs_start = 0;
    let mut abs_end = bytes.len();

    for segment in pattern.split('.') {
        let slice = &bytes[abs_start..abs_end];
        let section = find_segment(slice, segment)?;
        abs_start += section.start;
        abs_end = abs_start + section.end - section.start;
    }

    Ok(Span::new(abs_start, abs_end))
}

fn find_segment(bytes: &[u8], segment: &str) -> Result<Span, Error> {
    if bytes.is_empty() {
        return Err(Error::invalid_json());
    }

    match bytes[0] {
        b'{' => search_object(bytes, segment),
        b'[' => Err(Error::unsupported_array()),
        _ => Err(Error::invalid_json()),
    }
}

#[inline]
fn search_object(bytes: &[u8], pattern: &str) -> Result<Span, Error> {
    let mut pos = 1;

    while pos < bytes.len() {
        let (is_match, field_end) = match_field(&bytes[pos..], pattern)?;
        pos += field_end;

        let (start, len) = find_value(&bytes[pos..])?;
        if is_match {
            return Ok(Span::new(pos + start, pos + start + len));
        } else {
            pos += start + len;
        }
    }

    Err(Error::not_found())
}

fn find_value(bytes: &[u8]) -> Result<(usize, usize), Error> {
    let mut pos = 0;
    while pos < bytes.len() {
        match bytes[pos] {
            b':' => {
                pos += 1;
                pos += skip_whitespace(&bytes[pos..])?;
                let start = pos;
                let len = match bytes[pos] {
                    b'[' => scan_array(&bytes[pos..])?,
                    b'{' => scan_object(&bytes[pos..])?,
                    b'"' => scan_string(&bytes[pos..])?,
                    _ => scan_primitive(&bytes[pos..])?,
                };
                return Ok((start, len));
            }
            _ => pos += 1,
        }
    }

    Err(Error::invalid_json())
}

fn match_field(bytes: &[u8], pattern: &str) -> Result<(bool, usize), Error> {
    let quote_pos = next_quote(bytes).ok_or_else(Error::not_found)?;
    let start = quote_pos + 1;
    let len = scan_string(&bytes[quote_pos..])?;
    let end = quote_pos + len;
    let field_name = &bytes[start..end - 1];

    Ok((field_name == pattern.as_bytes(), end))
}
