use pixelart::{pixels::canvas::pen::PixelPen, prelude::*};

pub fn create_attach_detach() {
    let mut canvas = PixelCanvas::<5>::default();

    let red_pen = PixelPen::new(PixelColor::RED);
    let blue_pen = PixelPen::new(PixelColor::BLUE);

    let attached_pen = red_pen.attach(&mut canvas, StrictPositions::TopLeft);
    let _red_pen = attached_pen.detach();

    let mut attached_pen = blue_pen.attach(&mut canvas, StrictPositions::TopLeft);
    attached_pen
        .start_drawing()
        .go_right(2)
        .go_down_right(2)
        .go_down_left(2)
        .go_left(2);

    canvas.fill_inside(PixelColor::CYAN, StrictPositions::LeftCenter);

    canvas
        .default_image_builder()
        .with_scale(5)
        .save("arts/pen_1.png")
        .unwrap();
}

pub fn branching() {
    let mut canvas = PixelCanvas::<5>::default();

    canvas
        .attach_new_pen(PixelColor::CYAN, StrictPositions::BottomCenter)
        .start_drawing()
        .go_up(2)
        .branch(|pen| pen.go_up_left(2))
        .go_up_right(2);

    canvas
        .default_image_builder()
        .with_scale(5)
        .save("arts/pen_0.png")
        .unwrap();
}
