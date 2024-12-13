use std::{fs::File, path::Path};

use image::{
    codecs::gif::{GifEncoder, Repeat},
    Frame, ImageBuffer, ImageResult, Rgba,
};

use crate::{
    pixels::{
        canvas::{partition::CanvasPartition, PixelCanvasInterface},
        Pixel, PixelInterface,
    },
    prelude::{MaybePixel, PixelCanvas},
};
use crate::{
    pixels::{
        canvas::{SharedMutPixelCanvasExt, SharedPixelCanvasExt},
        color::RgbaInterface,
        position::IntoPixelStrictPosition,
    },
    prelude::PixelColor,
};

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

    pub fn new_empty(repeat: Repeat, scale: usize) -> Self {
        Self::new(repeat, scale, [])
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

pub fn create_animation<C>(
    mut ctx: C,
    frame_count: Repeat,
    beginner: impl FnOnce(&mut C) + Copy,
    frame_body: impl FnOnce(u16, &mut C) -> bool + Copy,
    frame_finisher: impl FnOnce(u16, &mut C) + Copy,
) -> C {
    beginner(&mut ctx);
    match frame_count {
        Repeat::Finite(frame_count) => {
            for i in 0..frame_count {
                if frame_body(i, &mut ctx) {
                    frame_finisher(i, &mut ctx)
                } else {
                    break;
                }
            }
        }
        Repeat::Infinite => {
            let mut i = 0;
            loop {
                if frame_body(i, &mut ctx) {
                    frame_finisher(i, &mut ctx);
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }

    ctx
}

pub struct SimpleAnimationContext<const H: usize, const W: usize, const PH: usize, const PW: usize>
{
    pub part: CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel>,
    pub builder: PixelAnimationBuilder,
}

impl<const H: usize, const W: usize, const PH: usize, const PW: usize>
    SimpleAnimationContext<H, W, PH, PW>
{
    pub fn new(
        part: CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel>,
        builder: PixelAnimationBuilder,
    ) -> Self {
        Self { part, builder }
    }

    pub fn update_part_color(
        &mut self,
        color: impl Into<<Pixel as PixelInterface>::ColorType> + Clone,
    ) -> &mut SimpleAnimationContext<H, W, PH, PW>
    where
        Option<PixelColor>: From<<Pixel as PixelInterface>::ColorType> + Clone,
    {
        self.part
            .update_color(Into::<<Pixel as PixelInterface>::ColorType>::into(color));
        self
    }

    pub fn body_mut(&mut self) -> &mut PixelCanvas<H, W> {
        self.part.source_table_mut()
    }

    pub fn update_body_color(
        &mut self,
        color: impl Into<<Pixel as PixelInterface>::ColorType> + Clone,
    ) -> &mut SimpleAnimationContext<H, W, PH, PW> {
        self.body_mut().fill(color);
        self
    }

    pub fn capture(&mut self) -> &mut SimpleAnimationContext<H, W, PH, PW> {
        self.builder
            .push_frame_from_canvas(self.part.source_table());
        self
    }

    pub fn save<P: AsRef<Path>>(self, path: P) -> Result<(), image::ImageError> {
        self.builder.save(path)
    }
}

pub fn create_simple_animation<const H: usize, const W: usize, const PH: usize, const PW: usize>(
    partition_position: impl IntoPixelStrictPosition<H, W>,
    builder: PixelAnimationBuilder,
    frame_count: Repeat,
    beginner: impl FnOnce(&mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
    frame_body: impl FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) -> bool + Copy,
    frame_finisher: impl FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
) -> SimpleAnimationContext<H, W, PH, PW> {
    create_animation::<SimpleAnimationContext<H, W, PH, PW>>(
        SimpleAnimationContext {
            builder,
            part: CanvasPartition::new(partition_position, PixelCanvas::default()),
        },
        frame_count,
        |ctx| {
            beginner(ctx);
            ctx.capture();
        },
        frame_body,
        |i, ctx| {
            frame_finisher(i, ctx);
            ctx.capture();
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[test]
    fn test_name() {
        create_simple_animation::<5, 5, 1, 1>(
            TOP_LEFT,
            PixelAnimationBuilder::new_empty(Repeat::Infinite, 5),
            Repeat::Infinite,
            |ctx| {
                ctx.update_body_color(YELLOW);
                ctx.update_part_color(BLUE);
            },
            |i, ctx| {
                if let Some(next) = ctx.part.position().next() {
                    ctx.part
                        .replace_to(next, PixelColor::from_blue(255 - (i as u8 * 10) % 250));
                    ctx.update_part_color(PixelColor::from_blue(255 - (i as u8 * 10) % 250));
                    true
                } else {
                    false
                }
            },
            |_, _ctx| {},
        )
        .save("arts/animation_0.gif")
        .unwrap();
    }
}
