#![feature(test)]
extern crate test;

use std::hint::black_box;
use test::Bencher;

#[bench]
fn bench_filter_chain_count(b: &mut Bencher) {
    b.iter(|| (0i64..1000000).chain(0..1000000).map(black_box).filter(|x| x % 3 == 0).count())
}
