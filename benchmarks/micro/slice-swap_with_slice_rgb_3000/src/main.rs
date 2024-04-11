use std::hint::black_box;

#[derive(Clone)]
struct Rgb(
    #[allow(dead_code)] u8,
    #[allow(dead_code)] u8,
    #[allow(dead_code)] u8,
);

impl Rgb {
    fn gen(i: usize) -> Self {
        Rgb(
            i as u8,
            (i as u8).wrapping_add(7),
            (i as u8).wrapping_add(42),
        )
    }
}

macro_rules! swap_with_slice {
    ($fn:ident, $n:expr, $mapper:expr) => {
        fn $fn() {
            let mut x = (0usize..$n).map(&$mapper).collect::<Vec<_>>();
            let mut y = ($n..($n * 2)).map(&$mapper).collect::<Vec<_>>();
            let mut skip = 0;
            for _ in 1..=10000 {
                for _ in 0..32 {
                    x[skip..].swap_with_slice(&mut y[..($n - skip)]);
                    skip = black_box(skip + 1) % 8;
                }
                black_box((x[$n / 3].clone(), y[$n * 2 / 3].clone()));
            }
        }
    };
}

swap_with_slice!(swap_with_slice_rgb_3000, 3000, Rgb::gen);

fn main() {
    swap_with_slice_rgb_3000();
}
