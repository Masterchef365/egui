use std::io::Write;

use ecolor::Color32;
use epaint::Fonts;

// cargo r --example gen_default_font_image --no-default-features --features bytemuck

fn main() {
    let pixels_per_point = 1.0;
    let max_texture_side = 1024;
    let text_alpha_from_coverage = Default::default();
    let definitions = Default::default();
    let imp = Fonts::new(pixels_per_point, max_texture_side, text_alpha_from_coverage, definitions);
    imp.begin_pass(pixels_per_point, max_texture_side, text_alpha_from_coverage);

    let galley = imp.layout("Hello, world!".into(), Default::default(), Color32::WHITE, 1000.0);

    let width = imp.image().width() as u32;
    let height = imp.image().height() as u32;

    let file = std::fs::File::create("fontimage.dat").unwrap();
    let mut file = std::io::BufWriter::new(file);

    file.write_all(&width.to_le_bytes());
    file.write_all(&height.to_le_bytes());
    file.write_all(imp.image().as_raw());

    file.flush();
}
