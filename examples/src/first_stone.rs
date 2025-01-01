use pixelart::prelude::*;

pub fn first_stone() {
    // A 5x5 pixel canvas.
    let mut canvas = PixelCanvas::<5>::default();

    // mutably access the pixel at the center.
    let mut center_pixel = canvas.get_pixel_mut(StrictPositions::Center);
    // Change its color to blue.
    center_pixel.color = PixelColor::BLUE;
    drop(center_pixel);

    // Create and save image based on the canvas.
    let image_builder = canvas.default_image_builder().with_scale(5);
    image_builder.save("arts/my_first_art.png").unwrap();
}
