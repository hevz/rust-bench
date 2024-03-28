#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use resvg::tiny_skia;
    use resvg::usvg::{Options, Transform, TreeParsing};

    // ~30 MiB SVG map from Wikipedia (https://upload.wikimedia.org/wikipedia/commons/7/7a/PrimeraFaseCentroExpedici%C3%B3nAlNorte.svg)
    static SVG_DATA: &[u8] = include_bytes!("../data/map.svg");

    #[bench]
    fn svg_parse(b: &mut Bencher) {
        b.iter(|| resvg::usvg::Tree::from_data(SVG_DATA, &Options::default()).unwrap());
    }

    #[bench]
    fn svg_render(b: &mut Bencher) {
        let svg_parsed_map =
            resvg::usvg::Tree::from_data(SVG_DATA, &Options::default()).unwrap();
        let resvg_tree = resvg::Tree::from_usvg(&svg_parsed_map);

        b.iter(|| {
            let width = 1024u32;
            let height = 1024u32;
            let mut buffer =
                vec![0u8; (width * height * tiny_skia::BYTES_PER_PIXEL as u32) as usize];
            let tree = &resvg_tree;

            let mut pixmap =
                tiny_skia::PixmapMut::from_bytes(buffer.as_mut_slice(), width, height).unwrap();
            tree.render(Transform::default(), &mut pixmap);
            buffer
        });
    }
}

fn main() {}
