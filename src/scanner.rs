use crate::Error;

#[inline]
pub fn next_quote(bytes: &[u8]) -> Option<usize> {
    bytes.iter().position(|&b| b == b'"')
}

pub fn scan_object(bytes: &[u8]) -> Result<usize, Error> {
    if bytes.is_empty() || bytes[0] != b'{' {
        return Err(Error::invalid_json());
    }

    let mut pos = 1;
    let mut depth = 1;
    while pos < bytes.len() && depth > 0 {
        match bytes[pos] {
            b'"' => pos += scan_string(&bytes[pos..])?,
            b'[' => pos += scan_array(&bytes[pos..])?,
            b'{' => {
                depth += 1;
                pos += 1;
            }
            b'}' => {
                depth -= 1;
                pos += 1;
            }
            _ => pos += 1,
        }
    }

    if depth == 0 {
        Ok(pos)
    } else {
        Err(Error::unmatched_bracket())
    }
}

pub fn scan_array(bytes: &[u8]) -> Result<usize, Error> {
    if bytes.is_empty() || bytes[0] != b'[' {
        return Err(Error::invalid_json());
    }

    let mut pos = 1;
    let mut depth = 1;
    while pos < bytes.len() && depth > 0 {
        match bytes[pos] {
            b'"' => pos += scan_string(&bytes[pos..])?,
            b'[' => {
                depth += 1;
                pos += 1;
            }
            b']' => {
                depth -= 1;
                pos += 1;
            }
            _ => pos += 1,
        }
    }

    if depth == 0 {
        Ok(pos)
    } else {
        Err(Error::unmatched_bracket())
    }
}

pub fn scan_string(bytes: &[u8]) -> Result<usize, Error> {
    if bytes.is_empty() || bytes[0] != b'"' {
        return Err(Error::invalid_json());
    }

    let mut pos = 1;
    while pos < bytes.len() {
        match bytes[pos] {
            b'"' => return Ok(pos + 1),
            b'\\' => pos += 2, // skip escaped character
            _ => pos += 1,
        }
    }

    Err(Error::unterminated_string())
}

pub fn scan_primitive(bytes: &[u8]) -> Result<usize, Error> {
    let mut pos = 0;
    while pos < bytes.len() {
        match bytes[pos] {
            b',' | b']' | b'}' => break,
            b' ' | b'\n' | b'\r' | b'\t' => {
                let after = pos + skip_whitespace_count(&bytes[pos..]);
                if after < bytes.len() && !matches!(bytes[after], b',' | b']' | b'}') {
                    return Err(Error::invalid_json());
                }
                break;
            }
            _ => pos += 1,
        }
    }
    if pos == 0 {
        Err(Error::invalid_json())
    } else {
        Ok(pos)
    }
}

#[inline]
pub fn skip_whitespace(bytes: &[u8]) -> Result<usize, Error> {
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes[pos].is_ascii_whitespace() {
            pos += 1;
        } else {
            return Ok(pos);
        }
    }

    Err(Error::invalid_json())
}

fn skip_whitespace_count(bytes: &[u8]) -> usize {
    bytes.iter().take_while(|b| b.is_ascii_whitespace()).count()
}
