pub mod image;
pub mod pixels;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::pixels::{
        canvas::{PixelCanvas, SharedPixelCanvasExt},
        color::{PixelColor, PixelColorExt},
        position::{PixelPositionInterface, StrictPositions},
        PixelInterface, PixelIterExt, PixelIterMutExt, PixelMutInterface,
    };

    #[test]
    fn basic_example_test() {
        // A 5x5 pixel canvas.
        let mut canvas = PixelCanvas::<5>::default();

        // A common position in a square canvas.
        let pos = StrictPositions::TopRight;

        assert_eq!(canvas[pos].color(), &PixelColor::WHITE);

        // Update color of a pixel.
        let prev_color = canvas[pos].update_color(PixelColor::BLACK);
        assert_eq!(prev_color, PixelColor::WHITE);
        assert_eq!(canvas[pos].color(), &PixelColor::BLACK);

        let black_pixels: usize = canvas.iter_pixels().filter_color(PixelColor::BLACK).count();
        assert_eq!(black_pixels, 1);

        // Change color of all pixels in main diagonal to blue where pos.row == pos.column.
        canvas
            // Iterate over all pixels row by row
            .iter_pixels_mut()
            // Filter main diagonal only pixels.
            .filter_position(|p| p.column() == p.row())
            // Update the pixel color for each item in iterator.
            .update_colors(PixelColor::RED);

        assert_eq!(canvas[StrictPositions::TopLeft].color(), &PixelColor::RED);
        assert_eq!(
            canvas[StrictPositions::BottomRight].color(),
            &PixelColor::RED
        );

        let image_builder = canvas.default_image_builder().with_scale(5);
        let image = image_builder.get_image();
        image.save("arts/basic.png").unwrap();
    }
}
