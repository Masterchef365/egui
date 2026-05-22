cargo r --example gen_default_font_image --features bytemuck,serde\
    && mv ./fontimage.dat src/text/\
    && mv ./glyphcache.dat src/text/

