use pixelart::prelude::*;

pub fn drawing_template_on_canvas() {
    let mut canvas = PixelCanvas::<5>::default();
    let mut my_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::default();

    my_5x5_diagonal_line_template
        .iter_pixels_mut()
        .filter_position(|p| p.0 == p.1)
        .update_colors(PixelColor::BLACK);

    my_5x5_diagonal_line_template.draw_on(StrictPositions::TopLeft, &mut canvas);

    canvas
        .default_image_builder()
        .with_scale(5)
        .save("arts/drawing_0.png")
        .unwrap();
}

pub fn drawing_template_on_template_on_canvas() {
    let mut my_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::default();
    my_5x5_diagonal_line_template
        .iter_pixels_mut()
        .filter_position(|p| p.1 == p.0)
        .update_colors(PixelColor::RED);

    let mut my_other_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::default();
    my_other_5x5_diagonal_line_template
        .iter_pixels_mut()
        .filter_position(|p| p.1 + p.0 == 4)
        .update_colors(PixelColor::BLUE);

    my_other_5x5_diagonal_line_template
        .draw_on(StrictPositions::TopLeft, &mut my_5x5_diagonal_line_template);

    let mut canvas = PixelCanvas::<5>::default();
    my_5x5_diagonal_line_template.draw_on(StrictPositions::TopLeft, &mut canvas);

    canvas
        .default_image_builder()
        .with_scale(5)
        .save("arts/drawing_1.png")
        .unwrap();
}
