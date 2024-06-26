//! Generates pixel images from any thing that implements [`PixelCanvasInterface`].
//!

use std::{marker::PhantomData, path::Path};

use image::{ImageBuffer, Rgba};
use imageproc::{
    drawing::{draw_filled_rect_mut, Canvas},
    rect::Rect,
};

use crate::pixels::{
    canvas::PixelCanvasInterface,
    color::{IntoPixelColor, RgbaInterface},
    position::PixelPositionInterface,
    PixelInterface,
};

/// Styles use by [`PixelImageBuilder`].
#[derive(Debug, Clone)]
pub struct PixelImageStyle {
    pixel_width: usize,
    border_width: usize,
    border_color: Rgba<u8>,
}

impl Default for PixelImageStyle {
    fn default() -> Self {
        Self::new(10, 1, 50)
    }
}

impl PixelImageStyle {
    pub fn new(pixel_width: usize, border_width: usize, border_color: impl IntoPixelColor) -> Self {
        Self {
            pixel_width,
            border_width,
            border_color: border_color.into_pixel_color().rgba(),
        }
    }

    /// Scales up each pixel and separator sizes on actual image.
    pub fn with_scale(mut self, scale: usize) -> PixelImageStyle {
        self.pixel_width *= scale;
        self.border_width *= scale;
        self
    }
}

/// A type which can help generating [`ImageBuffer`] from a [`PixelCanvasInterface`].
pub struct PixelImageBuilder<'c, const H: usize, const W: usize, P, I>
where
    I: PixelCanvasInterface<H, W, P>,
    P: PixelInterface,
{
    canvas_ref: &'c I,
    style: PixelImageStyle,
    _phantom: PhantomData<P>,
}

impl<'c, const H: usize, const W: usize, P: PixelInterface, I> PixelImageBuilder<'c, H, W, P, I>
where
    I: PixelCanvasInterface<H, W, P>,
{
    pub fn new(canvas_ref: &'c I, style: PixelImageStyle) -> Self {
        Self {
            canvas_ref,
            style,
            _phantom: PhantomData,
        }
    }

    /// Create a new instance of [`PixelImageBuilder`] with a default style.
    pub fn new_default_style(canvas_ref: &'c I) -> Self {
        Self {
            canvas_ref,
            style: Default::default(),
            _phantom: PhantomData,
        }
    }

    pub fn with_scale(self, scale: usize) -> Self {
        Self {
            style: self.style.with_scale(scale),
            ..self
        }
    }

    fn get_pixel_paper_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let separator_pixel_length = self.style.border_width;

        // How many pixels in height for blocks
        let blocks_pixel_in_height = H * self.style.pixel_width;
        let separators_count_in_height = H + 1;
        // How many pixels in height for separator
        let separators_pixel_in_height = separators_count_in_height * separator_pixel_length;
        let height = blocks_pixel_in_height + separators_pixel_in_height;

        let blocks_pixel_in_width = W * self.style.pixel_width;
        let separators_count_in_width = W + 1;
        let separators_pixel_in_width = separators_count_in_width * separator_pixel_length;
        let width = blocks_pixel_in_width + separators_pixel_in_width;

        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);

        image
    }

    /// Draws a pixel with its border.
    fn draw_pixel_on_image<'p>(&self, pixel: &'p P, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
    where
        'c: 'p,
        P: 'p,
        &'p <P as PixelInterface>::ColorType: RgbaInterface + 'p,
    {
        // Draw pixel border
        let pos = pixel.position();
        let row = pos.row();
        let column = pos.column();

        let start_row = (row * self.style.border_width) + (row * self.style.pixel_width);
        let start_column = (column * self.style.border_width) + (column * self.style.pixel_width);

        let bw = self.style.border_width;
        let pw = self.style.pixel_width;
        let bpw = bw + pw;

        draw_filled_rect_mut(
            image,
            Rect::at(start_column as i32, start_row as i32).of_size((bpw) as u32, bw as u32),
            self.style.border_color,
        );

        draw_filled_rect_mut(
            image,
            Rect::at((start_column + bpw) as i32, start_row as i32)
                .of_size(bw as u32, (bpw) as u32),
            self.style.border_color,
        );

        draw_filled_rect_mut(
            image,
            Rect::at(start_column as i32, (start_row + bw) as i32).of_size(bw as u32, (bpw) as u32),
            self.style.border_color,
        );

        draw_filled_rect_mut(
            image,
            Rect::at((start_column + bw) as i32, (start_row + bpw) as i32)
                .of_size((bpw) as u32, bw as u32),
            self.style.border_color,
        );

        // Draw the pixel
        let start_x_pixel = start_row + bw;
        let start_y_pixel = start_column + bw;

        for i in 0..self.style.pixel_width {
            for j in 0..self.style.pixel_width {
                image.draw_pixel(
                    (i + start_y_pixel) as u32,
                    (j + start_x_pixel) as u32,
                    pixel.color().rgba(),
                )
            }
        }
    }

    /// Draws the associated [`PixelCanvasInterface`] to an image buffer.
    pub fn draw_on_image<'p>(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
    where
        'c: 'p,
        P: 'p,
        &'p <P as PixelInterface>::ColorType: RgbaInterface + 'p,
    {
        let table = self.canvas_ref.table();

        for row in table.iter() {
            for pixel in row.iter().filter(|p| p.has_color()) {
                self.draw_pixel_on_image(pixel, image)
            }
        }
    }

    /// Returns an [`ImageBuffer`] based on the current canvas attached.
    pub fn get_image<'p>(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where
        'c: 'p,
        P: 'p,
        &'p <P as PixelInterface>::ColorType: RgbaInterface + 'p,
    {
        let mut image = self.get_pixel_paper_image();
        self.draw_on_image(&mut image);

        image
    }

    /// Saves the [`ImageBuffer`] to a file at specified path.
    pub fn save<'p, Q>(&self, path: Q) -> Result<(), image::ImageError>
    where
        'c: 'p,
        P: 'p,
        Q: AsRef<Path>,
        &'p <P as PixelInterface>::ColorType: RgbaInterface + 'p,
    {
        let image = self.get_image();
        image.save(path)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pixels::{
            canvas::{MaybePixelCanvas, SharedPixelCanvasExt as _},
            color::PixelColorExt as _,
            position::PixelPositionInterface as _,
            PixelIterExt, PixelIterMutExt as _,
        },
        prelude::{PixelCanvas, PixelColor},
    };

    #[test]
    fn full_pixel_test() {
        let canvas = PixelCanvas::<3>::new(PixelColor::YELLOW);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/image_0.png")
            .unwrap();
    }

    #[test]
    fn partial_pixel_test() {
        let mut canvas = MaybePixelCanvas::<3>::default();

        canvas
            .iter_pixels_mut()
            .filter_position(|p| p.column() == p.row())
            .update_colors(PixelColor::MAGENTA);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/image_1.png")
            .unwrap();
    }
}
