fn main() {
    let mut v = vec![0; 100];

    for _ in 1..=500000000 {
        let mut i = 0;
        for x in &mut v {
            *x = i;
            i += 1;
        }
    }
}
