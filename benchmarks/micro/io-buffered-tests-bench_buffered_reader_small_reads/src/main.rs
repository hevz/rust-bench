use std::hint::black_box;
use std::io::{BufReader, Read};

fn main() {
    let data = (0..u8::MAX).cycle().take(1024 * 4).collect::<Vec<_>>();
    for _ in 1..=1000000 {
        let mut reader = BufReader::new(&data[..]);
        let mut buf = [0u8; 4];
        for _ in 0..1024 {
            reader.read_exact(&mut buf).unwrap();
            black_box(&buf);
        }
    }
}
