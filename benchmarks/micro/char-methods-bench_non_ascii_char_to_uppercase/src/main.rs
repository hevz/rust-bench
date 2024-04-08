use std::hint::black_box;

fn main() {
    for _ in 1..=5000 {
        (128..=255)
            .cycle()
            .take(10_000)
            .map(|b| black_box(char::from(b)).to_uppercase())
            .count();
    }
}
