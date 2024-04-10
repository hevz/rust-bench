use std::hint::black_box;

fn main() {
    let text = black_box("kdjsfhlakfhlsghlkvcnljknfqiunvcijqenwodind");
    for _ in 1..=2000000 {
        for _ in 0..1024 {
            black_box(text.starts_with('k'));
        }
    }
}
