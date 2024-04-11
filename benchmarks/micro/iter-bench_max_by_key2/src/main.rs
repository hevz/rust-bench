use std::hint::black_box;

fn main() {
    fn max_index_iter(array: &[i32]) -> usize {
        array
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap()
            .0
    }

    let mut data = vec![0; 1638];
    data[514] = 9999;

    for _ in 1..=500000 {
        black_box(max_index_iter(&data));
    }
}
