use ecolor::Color32;
use epaint::COMMON_CHARS;
use epaint::{image::ImageStorage, Fonts};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// cargo r --example gen_default_font_image --no-default-features --features bytemuck


fn main() {
    let pixels_per_point = 1.0;
    let max_texture_side = 1024;
    let text_alpha_from_coverage = Default::default();
    let definitions = Default::default();
    let fonts = Fonts::new(pixels_per_point, max_texture_side, text_alpha_from_coverage, definitions);

    // Dummy pass with desired chars
    fonts.begin_pass(pixels_per_point, max_texture_side, text_alpha_from_coverage);
    let galley = fonts.layout(COMMON_CHARS.to_string(), Default::default(), Color32::WHITE, 1000.0);

    // Save glyph positions
    let mut lck = fonts.lock();
    let font_impl = lck.fonts.font(&Default::default()).fonts[0].clone();
    drop(lck);
    let cache = font_impl.glyph_info_cache.read().clone();

    let file = std::fs::File::create("glyphcache.dat").unwrap();
    let mut file = std::io::BufWriter::new(file);
    let bytes: Vec<u8> = postcard::to_allocvec(&cache).unwrap().to_vec();
    file.write_all(&bytes);

    // Save image
    let width = fonts.image().width() as u32;
    let height = fonts.image().height() as u32;

    let file = std::fs::File::create("fontimage.dat").unwrap();
    let mut file = std::io::BufWriter::new(file);

    file.write_all(&width.to_le_bytes());
    file.write_all(&height.to_le_bytes());
    let mut image = fonts.image().clone();

    /*
    let ImageStorage::Owned(pixels) = &mut image.pixels else { panic!() };
    pixels.iter_mut().skip(1).for_each(|px| {
        if *px != Color32::TRANSPARENT {
            *px = Color32::GREEN
        } else {
            *px = Color32::RED
        }
    });
    */

    file.write_all(image.as_raw());

    save_rgba_pam("image.pam", width as _, height as _, image.as_raw()).unwrap();

    file.flush();
}

/// Saves RGBA pixel data as a Netpbm PAM image (Portable Arbitrary Map).
///
/// The output format is PAM (P7), which supports alpha channels.
///
/// `rgba` must contain exactly width * height * 4 bytes.
///
/// Example:
/// ```
/// save_rgba_pam(
///     "output.pam",
///     256,
///     256,
///     &pixels,
/// )?;
/// ```
pub fn save_rgba_pam<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
    rgba: &[u8],
) -> std::io::Result<()> {
    let expected_len = (width as usize)
        .checked_mul(height as usize)
        .and_then(|v| v.checked_mul(4))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "image dimensions overflow",
            )
        })?;

    if rgba.len() != expected_len {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "RGBA buffer size mismatch: expected {}, got {}",
                expected_len,
                rgba.len()
            ),
        ));
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // PAM header
    write!(
        writer,
        "P7\nWIDTH {}\nHEIGHT {}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n",
        width,
        height
    )?;

    // Raw RGBA pixel data
    writer.write_all(rgba)?;

    writer.flush()?;

    Ok(())
}
