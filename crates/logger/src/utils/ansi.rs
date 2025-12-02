/// Strip ANSI escape codes from bytes
pub fn strip_ansi(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len());
    let mut in_escape = false;

    for &byte in input {
        if byte == 0x1b {
            in_escape = true;
        } else if in_escape {
            if byte == b'm' {
                in_escape = false;
            }
        } else {
            result.push(byte);
        }
    }

    result
}
