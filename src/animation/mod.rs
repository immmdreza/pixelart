use std::{fs::File, path::Path};

use image::{
    codecs::gif::{GifEncoder, Repeat},
    Frame, ImageBuffer, ImageResult, Rgba,
};

use crate::pixels::{canvas::PixelCanvasInterface, PixelInterface};
use crate::pixels::{canvas::SharedPixelCanvasExt, color::RgbaInterface};

pub struct PixelAnimationBuilder {
    repeat: Repeat,
    scale: usize,
    images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl PixelAnimationBuilder {
    pub fn new(
        repeat: Repeat,
        scale: usize,
        images: impl IntoIterator<Item = ImageBuffer<Rgba<u8>, Vec<u8>>>,
    ) -> Self {
        Self {
            repeat,
            scale,
            images: images.into_iter().collect(),
        }
    }

    pub fn save<P>(self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        let mut encoder = GifEncoder::new(File::create(path).unwrap());
        encoder.set_repeat(self.repeat)?;
        let frames = self.images.into_iter().map(|f| Frame::new(f));
        encoder.encode_frames(frames)?;
        Ok(())
    }

    pub fn push_frame_from_canvas<
        'a,
        const H: usize,
        const W: usize,
        P: PixelInterface + 'a,
        I: PixelCanvasInterface<H, W, P>,
    >(
        &mut self,
        value: &'a I,
    ) where
        &'a <P as PixelInterface>::ColorType: RgbaInterface,
        <P as PixelInterface>::ColorType: 'a,
    {
        self.images.push(
            value
                .default_image_builder()
                .with_scale(self.scale)
                .get_image(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{pixels::canvas::partition::CanvasPartition, prelude::*};

    use super::*;

    #[test]
    fn test_name() {
        let mut canvas = PixelCanvas::<5>::default();
        let mut builder = PixelAnimationBuilder::new(Repeat::Infinite, 5, []);

        let mut part = CanvasPartition::<1, 1, 5, 5, _, _, MaybePixel>::new(TOP_LEFT, &mut canvas);

        part.update_color(RED);

        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(TOP_CENTER);
        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(TOP_RIGHT);
        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(RIGHT_CENTER);
        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(BOTTOM_RIGHT);
        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(BOTTOM_CENTER);
        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(BOTTOM_LEFT);
        builder.push_frame_from_canvas(part.source_table());

        part.crop_to(LEFT_CENTER);
        builder.push_frame_from_canvas(part.source_table());

        builder.save("arts/animation_1.gif").unwrap();
    }
}
