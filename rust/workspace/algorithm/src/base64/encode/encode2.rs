const ENCODE_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &[u8]) -> String {
    let mut result = Vec::with_capacity((data.len() + 2) / 3 * 4);
    let mut acc: usize = 0;
    for (i, &b) in data.iter().enumerate() {
        acc <<= 8;
        acc |= b as usize;
        if i % 3 == 2 {
            result.push(ENCODE_TABLE[acc >> 18]);
            result.push(ENCODE_TABLE[acc >> 12 & 0x3F]);
            result.push(ENCODE_TABLE[acc >> 6 & 0x3F]);
            result.push(ENCODE_TABLE[acc & 0x3F]);
            acc = 0;
        } else if i == data.len() - 1 {
            match i % 3 {
                0 => {
                    result.push(ENCODE_TABLE[acc >> 2]);
                    result.push(ENCODE_TABLE[acc << 4 & 0x3F]);
                    result.push(b'=');
                    result.push(b'=');
                }
                1 => {
                    result.push(ENCODE_TABLE[acc >> 10]);
                    result.push(ENCODE_TABLE[acc >> 4 & 0x3F]);
                    result.push(ENCODE_TABLE[acc << 2 & 0x3F]);
                    result.push(b'=');
                }
                _ => {}
            }
        }
    }

    unsafe { String::from_utf8_unchecked(result) }
}
