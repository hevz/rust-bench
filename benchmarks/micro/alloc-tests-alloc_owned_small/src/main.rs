#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn alloc_owned_small(b: &mut Bencher) {
    b.iter(|| {
        let _: Box<_> = Box::new(10);
    })
}
