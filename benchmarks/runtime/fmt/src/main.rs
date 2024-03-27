#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use test::Bencher;

    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
        age: u8,
    }

    #[bench]
    fn fmt_write_str(b: &mut Bencher) {
        b.iter(|| {
            const CONST_STRING: &str = "foobar";
            let mut buffer = String::with_capacity(256 * 1024 * 1024);
            let iterations = 5000000;
            for i in 0..iterations {
                write!(buffer, "Iteration {i} out of {iterations}: {CONST_STRING}").unwrap();
            }
            buffer
        });
    }

    #[bench]
    fn fmt_debug_derive(b: &mut Bencher) {
        let persons: Vec<_> = (0u64..1000000)
            .map(|i| Person {
                first_name: format!("Jake {i}"),
                last_name: format!("Novak {i}"),
                age: (i % 100) as u8,
            })
            .collect();

        b.iter(|| {
            let mut buffer = String::with_capacity(256 * 1024 * 1024);
            write!(buffer, "{persons:?}").unwrap();
            buffer
        });
    }
}

fn main() {}
