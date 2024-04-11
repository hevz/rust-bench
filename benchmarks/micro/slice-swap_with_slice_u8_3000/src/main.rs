use std::hint::black_box;

macro_rules! swap_with_slice {
    ($fn:ident, $n:expr, $mapper:expr) => {
        fn $fn() {
            let mut x = (0usize..$n).map(&$mapper).collect::<Vec<_>>();
            let mut y = ($n..($n * 2)).map(&$mapper).collect::<Vec<_>>();
            let mut skip = 0;
            for _ in 1..=20000 {
                for _ in 0..32 {
                    x[skip..].swap_with_slice(&mut y[..($n - skip)]);
                    skip = black_box(skip + 1) % 8;
                }
                black_box((x[$n / 3].clone(), y[$n * 2 / 3].clone()));
            }
        }
    };
}

swap_with_slice!(swap_with_slice_u8_3000, 3000, |i| i as u8);

fn main() {
    swap_with_slice_u8_3000();
}
