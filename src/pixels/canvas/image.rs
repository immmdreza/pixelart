use std::{marker::PhantomData, path::Path};

use image::{ImageBuffer, Rgba};
use imageproc::{
    drawing::{draw_filled_rect_mut, Canvas},
    rect::Rect,
};

use crate::pixels::{
    color::{IntoPixelColor, PixelColor, PixelColorExt, PixelColorInterface},
    position::PixelPositionInterface,
    PixelInterface,
};

use super::PixelCanvasInterface;

/// Styles use by [`PixelImageBuilder`].
#[derive(Debug, Clone)]
pub struct PixelImageStyle {
    pixel_width: usize,
    separator_width: usize,
    background: Rgba<u8>,
    separator_color: Rgba<u8>,
}

impl Default for PixelImageStyle {
    fn default() -> Self {
        Self::new(10, 1, PixelColor::WHITE, PixelColor::BLACK)
    }
}

impl PixelImageStyle {
    pub fn new(
        pixel_width: usize,
        separator_width: usize,
        background: impl IntoPixelColor,
        separator_color: impl IntoPixelColor,
    ) -> Self {
        Self {
            pixel_width,
            separator_width,
            background: background.into_pixel_color().rgba(),
            separator_color: separator_color.into_pixel_color().rgba(),
        }
    }

    /// Scales up each pixel and separator sizes on actual image.
    pub fn with_scale(mut self, scale: usize) -> PixelImageStyle {
        self.pixel_width *= scale;
        self.separator_width *= scale;
        self
    }
}

/// A type which can help generating [`ImageBuffer`] from a [`PixelCanvasInterface`].
pub struct PixelImageBuilder<'c, const H: usize, const W: usize, I, P: PixelInterface>
where
    I: PixelCanvasInterface<H, W, P>,
{
    canvas_ref: &'c I,
    style: PixelImageStyle,
    _phantom: PhantomData<P>,
}

impl<'c, const H: usize, const W: usize, I, P: PixelInterface> PixelImageBuilder<'c, H, W, I, P>
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

    fn get_pixel_paper_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let separator_pixel_length = self.style.separator_width;

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

        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(width as u32, height as u32);

        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(width as u32, height as u32),
            self.style.background,
        );

        for i in 0..separators_count_in_width as i32 {
            draw_filled_rect_mut(
                &mut image,
                Rect::at(
                    i * ((separator_pixel_length + self.style.pixel_width) as i32),
                    0,
                )
                .of_size(separator_pixel_length as u32, height as u32),
                self.style.separator_color,
            )
        }

        for i in 0..separators_count_in_height as i32 {
            draw_filled_rect_mut(
                &mut image,
                Rect::at(
                    0,
                    i * ((separator_pixel_length + self.style.pixel_width) as i32),
                )
                .of_size(width as u32, separator_pixel_length as u32),
                self.style.separator_color,
            )
        }

        image
    }

    fn draw_to_image_pixels(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        color: Rgba<u8>,
        row: usize,
        column: usize,
    ) {
        // Find out start pixel ...
        // 0, 0 -> 1sp + 0bp, 1sp + 0bp
        // 1, 1 -> 2sp + 1bp, 2sp + 1bp
        // 2, 2 -> 3sp + 2bp, 3sp + 2bp
        // i, j -> (i + 1)sp + ibp, (j + 1)sp + jbp
        let start_x_pixel =
            ((column + 1) * self.style.separator_width) + (column * self.style.pixel_width);
        let start_y_pixel =
            ((row + 1) * self.style.separator_width) + (row * self.style.pixel_width);

        for i in 0..self.style.pixel_width {
            for j in 0..self.style.pixel_width {
                image.draw_pixel(
                    (i + start_y_pixel) as u32,
                    (j + start_x_pixel) as u32,
                    color,
                )
            }
        }
    }

    /// Draws the associated [`PixelCanvasInterface`] to an image buffer.
    pub fn draw_on_image(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let table = self.canvas_ref.table();

        for row in table.iter() {
            for pixel in row.iter() {
                self.draw_to_image_pixels(
                    image,
                    pixel.color().rgba(),
                    pixel.position().column(),
                    pixel.position().row(),
                )
            }
        }
    }

    /// Returns an [`ImageBuffer`] based on the current canvas attached.
    pub fn get_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut image = self.get_pixel_paper_image();
        self.draw_on_image(&mut image);

        image
    }

    /// Saves the [`ImageBuffer`] to a file at specified path.
    pub fn save<Q>(&self, path: Q) -> Result<(), image::ImageError>
    where
        Q: AsRef<Path>,
    {
        let image = self.get_image();
        image.save(path)
    }
}
