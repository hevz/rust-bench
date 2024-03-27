#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use lightningcss::stylesheet::{ParserOptions, StyleSheet};

    static FB_CSS: &str = include_str!("../data/fb.css");

    #[bench]
    fn css_parse_fb(b: &mut Bencher) {
        // Inflate the CSS data a bit
        let fb_css_minified = FB_CSS.repeat(10);

        b.iter(|| {
            StyleSheet::parse(&fb_css_minified, ParserOptions::default()).unwrap()
        });
    }
}

fn main() {}
