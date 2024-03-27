#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use brotli::enc::BrotliEncoderParams;
    use std::io::Write;
    use test::Bencher;

    const TEXT_SHERLOCK: &[u8] = include_bytes!("../data/sherlock.txt");

    fn compress(data: &str) -> Vec<u8> {
        let mut target: Vec<u8> = Vec::with_capacity(1024 * 1024);

        let mut writer = brotli::CompressorWriter::with_params(
            &mut target,
            4096,
            &BrotliEncoderParams::default(),
        );
        std::io::copy(&mut data.as_bytes(), &mut writer).unwrap();
        writer.flush().unwrap();
        drop(writer);
        target
    }

    #[bench]
    fn brotli_compress(b: &mut Bencher) {
        let sherlock_text = String::from_utf8(TEXT_SHERLOCK.to_vec()).expect("Invalid UTF-8");

        b.iter(|| {
            let mut target_buffer: Vec<u8> = Vec::with_capacity(1024 * 1024);
            let mut params = BrotliEncoderParams::default();
            params.quality = 10;

            let mut text = sherlock_text.as_bytes();

            let mut writer =
                brotli::CompressorWriter::with_params(&mut target_buffer, 4096, &params);
            std::io::copy(&mut text, &mut writer).unwrap();
            writer.flush().unwrap();
            drop(writer);
            target_buffer
        });
    }

    #[bench]
    fn brotli_decompress(b: &mut Bencher) {
        let sherlock_text = String::from_utf8(TEXT_SHERLOCK.to_vec()).expect("Invalid UTF-8");
        let compressed_text = compress(&sherlock_text.repeat(20));

        b.iter(|| {
            let mut buffer: Vec<u8> = Vec::with_capacity(TEXT_SHERLOCK.len() * 2);
            let mut reader = brotli::Decompressor::new(compressed_text.as_slice(), 4096);
            std::io::copy(&mut reader, &mut buffer).unwrap();
            buffer
        });
    }
}

fn main() {}
