use std::path::Path;

use image::{ImageBuffer, Rgba};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

use crate::pixels::{
    color::{IntoPixelColor, PixelColor, PixelColorExt, PixelColorInterface},
    position::PixelPositionInterface,
};

use super::PixelCanvasInterface;

#[derive(Debug, Clone)]
pub struct PixelImageStyle {
    block_width: usize,
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
        block_width: usize,
        separator_width: usize,
        background: impl IntoPixelColor,
        separator_color: impl IntoPixelColor,
    ) -> Self {
        Self {
            block_width,
            separator_width,
            background: background.into_pixel_color().rgba(),
            separator_color: separator_color.into_pixel_color().rgba(),
        }
    }

    // pub fn from_image_size<const H: usize, const W: usize>(height: u32, width: u32) {
    //     let separator_size = 1;
    //     let separators_in_height = H as u32 + 1;
    //     let block_width = (height - (separators_in_height * separator_size)) / H as u32;
    // }
}

pub struct PixelImageBuilder<'c, const H: usize, const W: usize, I>
where
    I: PixelCanvasInterface<H, W>,
{
    canvas_ref: &'c I,
    style: PixelImageStyle,
}

impl<'c, const H: usize, const W: usize, I> PixelImageBuilder<'c, H, W, I>
where
    I: PixelCanvasInterface<H, W>,
{
    pub fn new(canvas_ref: &'c I, style: PixelImageStyle) -> Self {
        Self { canvas_ref, style }
    }

    pub fn new_default_style(canvas_ref: &'c I) -> Self {
        Self {
            canvas_ref,
            style: Default::default(),
        }
    }

    fn get_pixel_paper_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let separator_pixel_length = self.style.separator_width;

        // How many pixels in height for blocks
        let blocks_pixel_in_height = H * self.style.block_width;
        let separators_count_in_height = H + 1;
        // How many pixels in height for separator
        let separators_pixel_in_height = separators_count_in_height * separator_pixel_length;
        let height = blocks_pixel_in_height + separators_pixel_in_height;

        let blocks_pixel_in_width = W * self.style.block_width;
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
                    i * ((separator_pixel_length + self.style.block_width) as i32),
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
                    i * ((separator_pixel_length + self.style.block_width) as i32),
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
            ((column + 1) * self.style.separator_width) + (column * self.style.block_width);
        let start_y_pixel =
            ((row + 1) * self.style.separator_width) + (row * self.style.block_width);

        for i in 0..self.style.block_width {
            for j in 0..self.style.block_width {
                image.put_pixel(
                    (i + start_y_pixel) as u32,
                    (j + start_x_pixel) as u32,
                    color,
                )
            }
        }
    }

    fn draw_pixels_table(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let table = self.canvas_ref.table();

        for row in table.iter() {
            for pixel in row.iter() {
                self.draw_to_image_pixels(
                    image,
                    pixel.color.rgba(),
                    pixel.position.row(),
                    pixel.position.column(),
                )
            }
        }
    }

    pub(crate) fn get_image_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut image = self.get_pixel_paper_image();
        self.draw_pixels_table(&mut image);

        image
    }

    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        let image = self.get_image_buffer();
        image.save(Path::new(path))
    }
}
