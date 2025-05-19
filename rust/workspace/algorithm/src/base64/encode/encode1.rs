use crate::base64::CHARS;

pub fn encode(data: &[u8]) -> String {
    let n = data.len();
    let mut result = Vec::with_capacity((n + 2) / 3 * 4);

    let n = n / 3 * 3;
    let mut i = 0;
    while i < n {
        let i0 = data[i] >> 2;
        let i1 = (data[i] << 4 & 0x3F) | data[i + 1] >> 4;
        let i2 = (data[i + 1] << 2 & 0x3F) | data[i + 2] >> 6;
        let i3 = data[i + 2] & 0x3F;
        result.push(CHARS[i0 as usize]);
        result.push(CHARS[i1 as usize]);
        result.push(CHARS[i2 as usize]);
        result.push(CHARS[i3 as usize]);
        i += 3;
    }

    match data.len() - n {
        1 => {
            let i0 = data[n] >> 2;
            let i1 = data[n] << 4 & 0x3F;
            result.push(CHARS[i0 as usize]);
            result.push(CHARS[i1 as usize]);
            result.push(b'=');
            result.push(b'=');
        }
        2 => {
            let i0 = data[n] >> 2;
            let i1 = (data[n] << 4 & 0x3F) | data[n + 1] >> 4;
            let i2 = data[n + 1] << 2 & 0x3F;
            result.push(CHARS[i0 as usize]);
            result.push(CHARS[i1 as usize]);
            result.push(CHARS[i2 as usize]);
            result.push(b'=');
        }
        _ => {}
    }

    unsafe { String::from_utf8_unchecked(result) }
}
