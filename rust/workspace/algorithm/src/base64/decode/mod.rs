use crate::base64::CHARS;

pub fn decode(input: &str) -> Result<Vec<u8>, &str> {
    if input.len() % 4 != 0 {
        return Err("Input length must be a multiple of 4");
    }

    let mut table = [255_u8; 256];
    for (i, &b) in CHARS.iter().enumerate() {
        table[b as usize] = i as u8;
    }

    let mut result = Vec::with_capacity(input.len() / 4 * 3);
    let input = input.as_bytes();
    let len = input.len();
    let mut i = 0;
    while i < len {
        let mut acc: u32 = 0;
        let end = i + 4;
        while i < end {
            if input[i] == b'=' {
                if i < len - 2 {
                    return Err("Padding must be at the end");
                } else if i == len - 2 {
                    if input[len - 1] != b'=' {
                        return Err("Padding must be at the end");
                    }
                    result.push((acc >> 4) as u8);
                    return Ok(result);
                } else {
                    result.push((acc >> 10) as u8);
                    result.push((acc >> 2 & 0xFF) as u8);
                    return Ok(result);
                }
            }

            let v = table[input[i] as usize] as u32;
            if v == 255 {
                return Err("Invalid character in input");
            }

            acc <<= 6;
            acc |= v;
            i += 1;
        }
        result.push((acc >> 16) as u8);
        result.push((acc >> 8 & 0xFF) as u8);
        result.push((acc & 0xFF) as u8);
    }

    Ok(result)
}

#[cfg(test)]
mod tests;
