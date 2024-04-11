use std::hint::black_box;

enum Cache {
    L1,
    L2,
    L3,
}

impl Cache {
    fn size(&self) -> usize {
        match self {
            Cache::L1 => 1000,      // 8kb
            Cache::L2 => 10_000,    // 80kb
            Cache::L3 => 1_000_000, // 8Mb
        }
    }
}

fn binary_search<F>(cache: Cache, mapper: F)
where
    F: Fn(usize) -> usize,
{
    let size = cache.size();
    let v = (0..size).map(&mapper).collect::<Vec<_>>();
    let mut r = 0usize;
    for _ in 1..=50000000 {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = mapper(r % size);
        black_box(v.binary_search(&i).is_ok());
    }
}

fn binary_search_worst_case(cache: Cache) {
    let size = cache.size();

    let mut v = vec![0; size];
    let i = 1;
    v[size - 1] = i;
    for _ in 1..=50000000 {
        black_box(v.binary_search(&i).is_ok());
    }
}
fn main() {
    binary_search_worst_case(Cache::L1);
}
