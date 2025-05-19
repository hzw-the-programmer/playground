const ENCODE_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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
        result.push(ENCODE_TABLE[i0 as usize]);
        result.push(ENCODE_TABLE[i1 as usize]);
        result.push(ENCODE_TABLE[i2 as usize]);
        result.push(ENCODE_TABLE[i3 as usize]);
        i += 3;
    }

    match data.len() - n {
        1 => {
            let i0 = data[n] >> 2;
            let i1 = data[n] << 4 & 0x3F;
            result.push(ENCODE_TABLE[i0 as usize]);
            result.push(ENCODE_TABLE[i1 as usize]);
            result.push(b'=');
            result.push(b'=');
        }
        2 => {
            let i0 = data[n] >> 2;
            let i1 = (data[n] << 4 & 0x3F) | data[n + 1] >> 4;
            let i2 = data[n + 1] << 2 & 0x3F;
            result.push(ENCODE_TABLE[i0 as usize]);
            result.push(ENCODE_TABLE[i1 as usize]);
            result.push(ENCODE_TABLE[i2 as usize]);
            result.push(b'=');
        }
        _ => {}
    }

    unsafe { String::from_utf8_unchecked(result) }
}
