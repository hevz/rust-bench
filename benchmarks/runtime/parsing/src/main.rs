#![feature(test)]
extern crate test;

mod json;

#[cfg(test)]
mod tests {
    use super::json::parse_json;
    use test::Bencher;

    // JSON describing GitHub events, taken from https://api.github.com/events.
    const GITHUB_EVENTS: &str = include_str!("../data/github-events.json");

    #[bench]
    fn nom_json(b: &mut Bencher) {
        // Inflate the data to make the benchmark run for longer
        let github_events = format!(
            "[{}]",
            std::iter::repeat(GITHUB_EVENTS)
                .take(100)
                .collect::<Vec<_>>()
                .join(",")
        );

        b.iter(|| parse_json(&github_events).unwrap());
    }
}

fn main() {}
