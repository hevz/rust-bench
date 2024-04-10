use std::hint::black_box;

fn scatter(x: i32) -> i32 {
    (x * 31) % 127
}

fn main() {
    for _ in 1..=5000000 {
        let it = 0..100;
        black_box(it.map(black_box).map(scatter).max());
    }
}
