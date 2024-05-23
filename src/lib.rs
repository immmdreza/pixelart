pub mod pixels;

#[cfg(test)]
mod tests {
    use crate::pixels::{
        canvas::{PixelCanvas, PixelCanvasExt, PixelCanvasInterface},
        color::{PixelColor, PixelColorExt},
        position::{CommonStrictPositions, PixelPositionInterface},
        PixelInterface, PixelIterExt, PixelIterMutExt,
    };

    #[test]
    fn basic_example_test() {
        // A 5x5 pixel canvas.
        let mut canvas = PixelCanvas::<5>::default();
        let table = canvas.table_mut();

        // A common position in a square canvas.
        let pos = CommonStrictPositions::TopRight;

        assert_eq!(table[pos].color(), &PixelColor::WHITE);

        // Update color of a pixel.
        let prev_color: PixelColor = table[pos].update_color(PixelColor::BLACK);
        assert_eq!(prev_color, PixelColor::WHITE);
        assert_eq!(table[pos].color(), &PixelColor::BLACK);

        let black_pixels: usize = table.iter_pixels().filter_color(PixelColor::BLACK).count();
        assert_eq!(black_pixels, 1);

        // Change color of all pixels in main diagonal to blue where pos.row == pos.column.
        table.iter_mut().for_each(|row| {
            row.iter_mut()
                // Filter main diagonal only pixels.
                .filter_position(|p| p.column() == p.row())
                // Update the pixel color for each item in iterator.
                .update_colors(PixelColor::BLUE)
        });

        assert_eq!(
            table[CommonStrictPositions::TopLeft].color(),
            &PixelColor::BLUE
        );
        assert_eq!(
            table[CommonStrictPositions::BottomRight].color(),
            &PixelColor::BLUE
        );

        let image_builder = canvas.image_builder_default_style();
        image_builder.save("arts/basic.png").unwrap();
    }
}
