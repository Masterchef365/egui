#cargo r --example gen_default_font_image --no-default-features --features bytemuck && mv fontimage.dat src/text/fontimage.dat
cargo r --example gen_default_font_image --features bytemuck && mv fontimage.dat src/text/fontimage.dat
