#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use regex::Regex;
    use test::Bencher;

    const TEXT_SHERLOCK: &[u8] = include_bytes!("../data/sherlock.txt");

    #[bench]
    fn regex_search(b: &mut Bencher) {
        let sherlock_text = String::from_utf8(TEXT_SHERLOCK.to_vec()).expect("Invalid UTF-8");

        let regex1 = Regex::new(r"[a-zA-Z]+ing").unwrap();

        b.iter(|| regex1.find_iter(&sherlock_text).count());
    }

    #[bench]
    fn regex_capture(b: &mut Bencher) {
        let sherlock_text = String::from_utf8(TEXT_SHERLOCK.to_vec()).expect("Invalid UTF-8");

        let regex2 = Regex::new(r"(Sherlock|Holmes|Watson|Irene|Adler|John|Baker)").unwrap();

        b.iter(|| regex2.captures_iter(&sherlock_text).count());
    }
}

fn main() {}
