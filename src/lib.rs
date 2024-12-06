/*!
(What's) PixelArt 👾?

Using this lib you can (for now) generate pixel images from your rust code.

## Example
```rust
use pixelart::prelude::*;

fn main() {
    let mut canvas = PixelCanvas::<5>::default();

    let pos = StrictPositions::TopRight;

    // Update color of a pixel.
    let prev_color = canvas[pos].update_color(BLACK);
    assert_eq!(prev_color, WHITE);
    assert_eq!(canvas[pos].color(), &BLACK);

    // Change color of all pixels in main diagonal to blue where pos.row == pos.column.
    canvas
        // Iterate over all pixels row by row
        .iter_pixels_mut()
        // Filter main diagonal only pixels.
        .filter_position(|p| p.column() == p.row())
        // Update the pixel color for each item in iterator.
        .update_colors(RED);

    canvas
        .default_image_builder()
        .with_scale(5)
        .save("arts/basic.png")
        .unwrap();
}
```

You can do many other things after you discovered them!
*/

pub mod image;
pub mod pixels;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn basic_example_test() {
        // A 5x5 pixel canvas.
        let mut canvas = PixelCanvas::<5>::default();

        // A common position in a square canvas.
        let pos = TOP_RIGHT;

        assert_eq!(canvas[pos].color(), &WHITE);

        let _part = canvas.partition(TOP_LEFT, BOTTOM_RIGHT);
        let _part_mut = canvas.partition_mut(TOP_LEFT, BOTTOM_RIGHT);

        // Update color of a pixel.
        let prev_color = canvas[pos].update_color(BLACK);
        assert_eq!(prev_color, WHITE);
        assert_eq!(canvas[pos].color(), &BLACK);

        let black_pixels: usize = canvas.iter_pixels().filter_color(BLACK).count();
        assert_eq!(black_pixels, 1);

        // Change color of all pixels in main diagonal to blue where pos.row == pos.column.
        canvas
            // Iterate over all pixels row by row
            .iter_pixels_mut()
            // Filter main diagonal only pixels.
            .filter_position(|p| p.column() == p.row())
            // Update the pixel color for each item in iterator.
            .update_colors(RED);

        assert_eq!(canvas[TOP_LEFT].color(), &RED);
        assert_eq!(canvas[BOTTOM_RIGHT].color(), &RED);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/basic.png")
            .unwrap();
    }
}
