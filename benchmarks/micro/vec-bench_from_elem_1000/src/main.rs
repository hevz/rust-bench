use std::hint::black_box;
use std::iter::repeat;

fn main() {
    let src_len = 1000;
    for _ in 1..=5000000 {
        black_box(repeat(5).take(src_len).collect::<Vec<usize>>());
    }
}
