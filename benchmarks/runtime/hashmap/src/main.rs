#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use fxhash::FxBuildHasher;
    use hashbrown::HashMap;
    use test::{black_box, Bencher};

    fn create_map_1m_integers() -> HashMap<u64, u64, FxBuildHasher> {
        let mut map: HashMap<u64, u64, _> =
            HashMap::with_capacity_and_hasher(1_000_000, FxBuildHasher::default());
        for index in 0..map.capacity() {
            map.insert(index as u64, index as u64);
        }
        map
    }

    #[bench]
    fn hashmap_insert_1m(b: &mut Bencher) {
        // Measures how long does it take to insert 1 million numbers into a hashmap.
        b.iter(|| {
            let count = 1_000_000;
            let mut map = HashMap::with_capacity_and_hasher(
                // Over allocate the hashmap to avoid reallocations when inserting
                count * 2,
                FxBuildHasher::default(),
            );
            for index in 0..count {
                map.insert(index, index);
            }
        });
    }

    #[bench]
    fn hashmap_remove_1m(b: &mut Bencher) {
        // Measures how long it takes to remove 1 million elements from a hashmap.
        b.iter(|| {
            let mut map = create_map_1m_integers();
            for index in 0..map.capacity() {
                map.remove(&(index as u64));
            }
        });
    }

    #[bench]
    fn hashmap_find_1m(b: &mut Bencher) {
        let map_1m_integers = create_map_1m_integers();
        // Measures how long it takes to find 1 million elements that are in a hashmap.
        b.iter(|| {
            let map = &map_1m_integers;
            for index in 0..map.capacity() {
                black_box(map.get(&(index as u64)));
            }
        });
    }

    #[bench]
    fn hashmap_find_misses_1m(b: &mut Bencher) {
        let map_1m_integers = create_map_1m_integers();
        // Measures how long it takes to find 1 million elements that are not in a hashmap.
        b.iter(|| {
            let map = &map_1m_integers;
            for index in map.capacity()..(map.capacity() * 2) {
                black_box(map.get(&(index as u64)));
            }
        });
    }

    #[bench]
    fn hashmap_iterate_1m(b: &mut Bencher) {
        let map_1m_integers = create_map_1m_integers();
        // Measures how long it takes to iterate through values of a hashmap with 1 million elements.
        b.iter(|| map_1m_integers.values().sum::<u64>());
    }
}

fn main() {}
