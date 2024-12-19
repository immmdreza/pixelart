use std::{fs::File, path::Path};

use image::{codecs::gif::GifEncoder, Frame, ImageResult};

use crate::image::DefaultImageBuffer;
use crate::pixels::{canvas::SharedPixelCanvasExt, color::RgbaInterface};
use crate::{
    pixels::{canvas::PixelCanvasInterface, PixelInterface},
    prelude::PixelCanvas,
};

pub use image::codecs::gif::Repeat;

#[cfg(feature = "viewer")]
use crate::viewer::{view, ViewResult};

pub mod layered;
pub mod simple;

pub struct PixelAnimationBuilder {
    pub(crate) repeat: Repeat,
    pub(crate) scale: usize,
    pub(crate) images: Vec<DefaultImageBuffer>,
}

impl PixelAnimationBuilder {
    pub fn new(
        repeat: Repeat,
        scale: usize,
        images: impl IntoIterator<Item = DefaultImageBuffer>,
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

    #[cfg(feature = "viewer")]
    pub fn view(self) -> ViewResult {
        view([self.images])
    }

    pub(crate) fn get_frame_to_push<
        const H: usize,
        const W: usize,
        P: PixelInterface,
        I: PixelCanvasInterface<H, W, P>,
    >(
        &self,
        value: &I,
    ) -> DefaultImageBuffer
    where
        P::ColorType: RgbaInterface,
    {
        value
            .default_image_builder()
            .with_scale(self.scale)
            .get_image()
    }

    pub fn push_frame_from_canvas<
        const H: usize,
        const W: usize,
        P: PixelInterface,
        I: PixelCanvasInterface<H, W, P>,
    >(
        &mut self,
        value: &I,
    ) where
        P::ColorType: RgbaInterface,
    {
        let frame = self.get_frame_to_push(value);
        self.images.push(frame)
    }
}

pub trait AnimationContext<const H: usize, const W: usize, P: PixelInterface> {
    fn frame_count(&self) -> &Repeat;

    fn builder(&self) -> &PixelAnimationBuilder;
    fn builder_mut(&mut self) -> &mut PixelAnimationBuilder;
    fn canvas(&self) -> &PixelCanvas<H, W, P>;
    fn canvas_mut(&mut self) -> &mut PixelCanvas<H, W, P>;

    fn get_frame_to_capture(&self) -> DefaultImageBuffer
    where
        <P as PixelInterface>::ColorType: RgbaInterface,
    {
        self.builder().get_frame_to_push(self.canvas())
    }

    fn capture(&mut self)
    where
        <P as PixelInterface>::ColorType: RgbaInterface,
    {
        let frame = self.get_frame_to_capture();
        self.builder_mut().images.push(frame);
    }
}

pub trait Animated<const H: usize, const W: usize, P: PixelInterface>
where
    <P as PixelInterface>::ColorType: RgbaInterface,
{
    type ContextType: AnimationContext<H, W, P>;

    fn create_context(&mut self) -> Self::ContextType;

    fn setup(&mut self, ctx: &mut Self::ContextType);
    fn update(&mut self, ctx: &mut Self::ContextType, i: u16) -> bool;

    fn process(&mut self) -> <Self as Animated<H, W, P>>::ContextType {
        let mut ctx = self.create_context();
        self.setup(&mut ctx);
        match ctx.frame_count() {
            Repeat::Finite(frame_count) => {
                for i in 0..*frame_count {
                    if self.update(&mut ctx, i) {
                        ctx.capture();
                    } else {
                        break;
                    }
                }
            }
            Repeat::Infinite => {
                let mut i = 0;
                loop {
                    if self.update(&mut ctx, i) {
                        ctx.capture();
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        ctx
    }
}

#[cfg(test)]
mod tests {
    use simple::SimpleAnimation;

    use crate::prelude::*;

    use super::*;

    #[test]
    fn test_name() {
        let mut animation = SimpleAnimation::<5, 5, 1, 1, _, _>::new(
            TOP_LEFT,
            5,
            Repeat::Infinite,
            Repeat::Infinite,
            |ctx| {
                ctx.update_body_color(YELLOW);
                ctx.update_part_color(BLUE);
            },
            |i, ctx| {
                if let Some(next) = ctx.part.position().next() {
                    ctx.part.copy_to(next);
                    ctx.update_part_color(PixelColor::from_blue(255 - (i as u8 * 10) % 250));
                    true
                } else {
                    false
                }
            },
        );

        animation.process().save("arts/animation_0.gif").unwrap();
    }
}
