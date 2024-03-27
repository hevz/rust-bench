#![feature(test)]
extern crate test;

mod nbody;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn nbody_5k(b: &mut Bencher) {
        // Calculates the N-body simulation.
        // Code taken from https://github.com/prestontw/rust-nbody
        b.iter(|| {
            let mut nbody = nbody::init(5000);
            for _ in 0..10 {
                nbody = nbody::compute_forces(nbody);
            }
            nbody
        });
    }
}

fn main() {}
