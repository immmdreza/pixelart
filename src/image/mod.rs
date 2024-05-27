use std::{marker::PhantomData, path::Path};

use image::{ImageBuffer, Rgba};
use imageproc::{
    drawing::{draw_filled_rect_mut, Canvas},
    rect::Rect,
};

use crate::pixels::{
    canvas::PixelCanvasInterface,
    color::{IntoPixelColor, PixelColor, PixelColorExt, PixelColorInterface},
    position::PixelPositionInterface,
    PixelInterface,
};

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
        Self::new(10, 1, PixelColor::WHITE, 50)
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

    fn draw_pixel_on_image<'p>(&self, pixel: &'p P, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
    where
        'c: 'p,
        P: 'p,
        &'p <P as PixelInterface>::ColorType: Into<Rgba<u8>> + 'p,
    {
        // Draw pixel border
        let pos = pixel.position();
        let row = pos.row();
        let column = pos.column();

        let start_row = (row * self.style.separator_width) + (row * self.style.pixel_width);
        let start_column =
            (column * self.style.separator_width) + (column * self.style.pixel_width);

        let bw = self.style.separator_width;
        let pw = self.style.pixel_width;

        let the_1 = (start_row, start_column);
        let h1 = bw;
        let w1 = bw + pw;
        draw_filled_rect_mut(
            image,
            Rect::at(the_1.1 as i32, the_1.0 as i32).of_size(w1 as u32, h1 as u32),
            self.style.separator_color,
        );

        let the_2 = (start_row, start_column + pw + bw);
        let h2 = bw + pw;
        let w2 = bw;
        draw_filled_rect_mut(
            image,
            Rect::at(the_2.1 as i32, the_2.0 as i32).of_size(w2 as u32, h2 as u32),
            self.style.separator_color,
        );

        let the_3 = (start_row + bw, start_column);
        let h3 = bw + pw;
        let w3 = bw;
        draw_filled_rect_mut(
            image,
            Rect::at(the_3.1 as i32, the_3.0 as i32).of_size(w3 as u32, h3 as u32),
            self.style.separator_color,
        );

        let the_4 = (start_row + bw + pw, start_column + bw);
        let h4 = bw;
        let w4 = bw + pw;
        draw_filled_rect_mut(
            image,
            Rect::at(the_4.1 as i32, the_4.0 as i32).of_size(w4 as u32, h4 as u32),
            self.style.separator_color,
        );

        let start_x_pixel = start_row + bw;
        let start_y_pixel = start_column + bw;

        for i in 0..self.style.pixel_width {
            for j in 0..self.style.pixel_width {
                image.draw_pixel(
                    (i + start_y_pixel) as u32,
                    (j + start_x_pixel) as u32,
                    pixel.color().into(),
                )
            }
        }
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
    pub fn draw_on_image<'p>(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
    where
        'c: 'p,
        P: 'p,
        &'p <P as PixelInterface>::ColorType: Into<Rgba<u8>> + 'p,
    {
        let table = self.canvas_ref.table();

        for row in table.iter() {
            for pixel in row.iter() {
                self.draw_to_image_pixels(
                    image,
                    pixel.color().into(),
                    pixel.position().column(),
                    pixel.position().row(),
                )
            }
        }
    }

    /// Returns an [`ImageBuffer`] based on the current canvas attached.
    pub fn get_image<'p>(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>>
    where
        'c: 'p,
        P: 'p,
        &'p <P as PixelInterface>::ColorType: Into<Rgba<u8>> + 'p,
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
        &'p <P as PixelInterface>::ColorType: Into<Rgba<u8>> + 'p,
    {
        let image = self.get_image();
        image.save(path)
    }
}
