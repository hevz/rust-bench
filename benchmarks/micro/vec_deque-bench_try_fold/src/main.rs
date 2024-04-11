use std::collections::VecDeque;
use std::hint::black_box;

fn main() {
    let ring: VecDeque<_> = (0..1000).collect();

    for _ in 1..=2500000 {
        black_box(ring.iter().try_fold(0, |a, b| Some(a + b)));
    }
}
