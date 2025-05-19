use crate::base64::CHARS;

pub fn encode(data: &[u8]) -> String {
    let mut result = Vec::with_capacity((data.len() + 2) / 3 * 4);
    let len = data.len() / 3 * 3;
    let mut i = 0;
    while i < len {
        let acc = (data[i] as usize) << 16 | (data[i + 1] as usize) << 8 | data[i + 2] as usize;
        result.push(CHARS[acc >> 18]);
        result.push(CHARS[acc >> 12 & 0x3F]);
        result.push(CHARS[acc >> 6 & 0x3F]);
        result.push(CHARS[acc & 0x3F]);
        i += 3;
    }

    match data.len() % 3 {
        1 => {
            let acc = data[i] as usize;
            result.push(CHARS[acc >> 2]);
            result.push(CHARS[acc << 4 & 0x3F]);
            result.push(b'=');
            result.push(b'=');
        }
        2 => {
            let acc = (data[i] as usize) << 8 | data[i + 1] as usize;
            result.push(CHARS[acc >> 10]);
            result.push(CHARS[acc >> 4 & 0x3F]);
            result.push(CHARS[acc << 2 & 0x3F]);
            result.push(b'=');
        }
        _ => {}
    }

    unsafe { String::from_utf8_unchecked(result) }
}
