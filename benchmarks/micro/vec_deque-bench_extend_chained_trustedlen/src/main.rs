use std::collections::VecDeque;
use std::hint::black_box;

fn main() {
    let mut ring: VecDeque<u16> = VecDeque::with_capacity(1000);

    for _ in 1..=5000000 {
        ring.clear();
        ring.extend(black_box((0..256).chain(768..1024)));
    }
}
