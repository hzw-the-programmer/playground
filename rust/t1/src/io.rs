use std::io::{self, Seek, SeekFrom, Write};

fn write_ten_bytes_at_end<W: Write + Seek>(mut writer: W) -> io::Result<()> {
    writer.seek(SeekFrom::End(-10))?;
    for i in 0..10 {
        writer.write(&[i])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_bytes() {
        use std::io::Cursor;
        let mut buf = Cursor::new(vec![0; 15]);
        write_ten_bytes_at_end(&mut buf).unwrap();
        assert_eq!(&buf.get_ref()[5..15], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
