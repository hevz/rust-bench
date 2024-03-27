#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use snap::{read::FrameDecoder, write::FrameEncoder};
    use std::io::{BufRead, BufReader, Write};
    use test::{black_box, Bencher};

    const BYTES: usize = 64 * 1024 * 1024;

    #[bench]
    fn bufreader_snappy(b: &mut Bencher) {
        let data = vec![0u8; BYTES];
        b.iter(|| {
            let mut compressed = Vec::new();
            FrameEncoder::new(&mut compressed)
                .write_all(data.as_slice())
                .unwrap();
            let mut reader = BufReader::with_capacity(BYTES, FrameDecoder::new(&compressed[..]));

            while let Ok(buf) = reader.fill_buf() {
                if buf.is_empty() {
                    break;
                }
                black_box(buf);
                let len = buf.len();
                reader.consume(len);
            }
            compressed
        });
    }
}

fn main() {}
