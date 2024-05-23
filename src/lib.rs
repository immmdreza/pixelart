pub mod pixels;

#[cfg(test)]
mod tests {
    use crate::pixels::{
        canvas::{PixelCanvas, PixelCanvasInterface},
        color::{PixelColor, PixelColorExt},
        position::CommonStrictPositions,
    };

    #[test]
    fn basic_example_test() {
        // A 5x5 pixel canvas.
        let mut canvas = PixelCanvas::<5>::default();
        let table = canvas.table_mut();

        // A common position in a square canvas.
        let pos = CommonStrictPositions::TopLeft;

        assert_eq!(table[pos].color(), &PixelColor::WHITE);

        // Update color of a pixel.
        let prev_color: PixelColor = table[pos].update_color(PixelColor::BLACK);
        assert_eq!(prev_color, PixelColor::WHITE);
        assert_eq!(table[pos].color(), &PixelColor::BLACK);
    }
}
