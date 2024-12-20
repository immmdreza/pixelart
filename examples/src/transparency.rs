use pixelart::{
    pixels::canvas::{templates::heart::Heart, SharedMutPixelCanvasExt, SharedPixelCanvasExt},
    prelude::{MaybePixelCanvas, PixelCanvas},
};

pub fn transparency() {
    let mut normal = PixelCanvas::<6, 7>::default();
    normal.draw_exact_abs(Heart);

    // PixelCanvas::<6, 7, MaybePixel>
    let mut transparent = MaybePixelCanvas::<6, 7>::default();
    transparent.draw_exact_abs(Heart);

    normal
        .default_image_builder()
        .with_scale(3)
        .view_with_others([transparent
            .default_image_builder()
            .with_scale(3)
            .get_image()])
        .unwrap();
}
