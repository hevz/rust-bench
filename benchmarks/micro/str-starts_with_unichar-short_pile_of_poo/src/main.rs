#![feature(test)]
extern crate test;

macro_rules! make_test_inner {
    ($s:ident, $code:expr, $name:ident, $str:expr, $iters:expr) => {
        #[bench]
        fn $name(bencher: &mut Bencher) {
            let mut $s = $str;
            black_box(&mut $s);
            bencher.iter(|| {
                for _ in 0..$iters {
                    black_box($code);
                }
            });
        }
    };
}

macro_rules! make_test {
    ($name:ident, $s:ident, $code:expr) => {
        make_test!($name, $s, $code, 1);
    };
    ($name:ident, $s:ident, $code:expr, $iters:expr) => {
        mod $name {
            use test::Bencher;
            use test::black_box;

            make_test_inner!($s, $code, short_pile_of_poo,
                "💩💩💩💩💩💩💩💩💩💩💩💩💩💩💩💩!", $iters);
        }
    }
}

make_test!(starts_with_unichar, s, s.starts_with('\u{1F4A4}'), 1024);
