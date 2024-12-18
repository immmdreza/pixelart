use std::{fs::File, path::Path};

use image::{codecs::gif::GifEncoder, Frame, ImageBuffer, ImageResult, Rgba};

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

pub use image::codecs::gif::Repeat;

#[cfg(feature = "viewer")]
use crate::viewer::{view, ViewResult};

pub struct PixelAnimationBuilder {
    pub(crate) repeat: Repeat,
    pub(crate) scale: usize,
    pub(crate) images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
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

    pub(crate) fn get_frame_to_push<
        const H: usize,
        const W: usize,
        P: PixelInterface,
        I: PixelCanvasInterface<H, W, P>,
    >(
        &self,
        value: &I,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>>
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

pub trait AnimationContext<const H: usize, const W: usize, P: PixelInterface> {
    fn builder(&self) -> &PixelAnimationBuilder;
    fn builder_mut(&mut self) -> &mut PixelAnimationBuilder;
    fn canvas(&self) -> &PixelCanvas<H, W, P>;
    fn canvas_mut(&mut self) -> &mut PixelCanvas<H, W, P>;
}

pub trait Animated<const H: usize, const W: usize, P: PixelInterface>
where
    <P as PixelInterface>::ColorType: RgbaInterface,
{
    type ContextType: AnimationContext<H, W, P>;

    fn frame_count(&self) -> &Repeat;

    fn context(&self) -> &Self::ContextType;
    fn context_mut(&mut self) -> &mut Self::ContextType;

    fn setup(&mut self);
    fn update(&mut self, i: u16) -> bool;

    fn capture(ctx: &mut Self::ContextType) {
        let frame = ctx.builder().get_frame_to_push(ctx.canvas());
        ctx.builder_mut().images.push(frame);
    }

    fn process(&mut self) {
        self.setup();
        match self.frame_count() {
            Repeat::Finite(frame_count) => {
                for i in 0..*frame_count {
                    if self.update(i) {
                        Self::capture(self.context_mut());
                    } else {
                        break;
                    }
                }
            }
            Repeat::Infinite => {
                let mut i = 0;
                loop {
                    if self.update(i) {
                        Self::capture(self.context_mut());
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

pub struct SimpleAnimationContext<const H: usize, const W: usize, const PH: usize, const PW: usize>
{
    pub part: CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel>,
    pub builder: PixelAnimationBuilder,
}

impl<const H: usize, const W: usize, const PH: usize, const PW: usize> AnimationContext<H, W, Pixel>
    for SimpleAnimationContext<H, W, PH, PW>
{
    fn builder(&self) -> &PixelAnimationBuilder {
        &self.builder
    }

    fn builder_mut(&mut self) -> &mut PixelAnimationBuilder {
        &mut self.builder
    }

    fn canvas(&self) -> &PixelCanvas<H, W, Pixel> {
        self.body()
    }

    fn canvas_mut(&mut self) -> &mut PixelCanvas<H, W, Pixel> {
        self.body_mut()
    }
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

    pub fn body(&self) -> &PixelCanvas<H, W> {
        self.part.source_table()
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

    #[cfg(feature = "viewer")]
    pub fn view(self) -> ViewResult {
        view([self.take_images()])
    }

    pub fn take_images(self) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        self.builder.images
    }
}

pub struct SimpleAnimation<
    const H: usize,
    const W: usize,
    const PH: usize,
    const PW: usize,
    F1: FnOnce(&mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
    F2: FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) -> bool + Copy,
> {
    frame_count: Repeat,
    ctx: SimpleAnimationContext<H, W, PH, PW>,
    setups: F1,
    updater: F2,
}

impl<
        const H: usize,
        const W: usize,
        const PH: usize,
        const PW: usize,
        F1: FnOnce(&mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
        F2: FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) -> bool + Copy,
    > SimpleAnimation<H, W, PH, PW, F1, F2>
{
    pub fn new(
        partition_position: impl IntoPixelStrictPosition<H, W>,
        builder: PixelAnimationBuilder,
        frame_count: Repeat,
        setups: F1,
        updater: F2,
    ) -> Self {
        Self {
            frame_count,
            ctx: SimpleAnimationContext {
                builder,
                part: CanvasPartition::new(partition_position, PixelCanvas::default()),
            },
            setups,
            updater,
        }
    }

    pub fn ctx(&self) -> &SimpleAnimationContext<H, W, PH, PW> {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut SimpleAnimationContext<H, W, PH, PW> {
        &mut self.ctx
    }

    pub fn take_ctx(self) -> SimpleAnimationContext<H, W, PH, PW> {
        self.ctx
    }

    pub fn frame_count(&self) -> Repeat {
        self.frame_count
    }
}

impl<
        const H: usize,
        const W: usize,
        const PH: usize,
        const PW: usize,
        F1: FnOnce(&mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
        F2: FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) -> bool + Copy,
    > Animated<H, W, Pixel> for SimpleAnimation<H, W, PH, PW, F1, F2>
{
    type ContextType = SimpleAnimationContext<H, W, PH, PW>;

    fn frame_count(&self) -> &Repeat {
        &self.frame_count
    }

    fn context(&self) -> &Self::ContextType {
        &self.ctx
    }

    fn context_mut(&mut self) -> &mut Self::ContextType {
        &mut self.ctx
    }

    fn setup(&mut self) {
        (self.setups)(&mut self.ctx)
    }

    fn update(&mut self, i: u16) -> bool {
        (self.updater)(i, &mut self.ctx)
    }
}

pub fn create_simple_animation<const H: usize, const W: usize, const PH: usize, const PW: usize>(
    partition_position: impl IntoPixelStrictPosition<H, W>,
    builder: PixelAnimationBuilder,
    frame_count: Repeat,
    setups: impl FnOnce(&mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
    updater: impl FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) -> bool + Copy,
) -> SimpleAnimationContext<H, W, PH, PW> {
    let mut animation = SimpleAnimation::<H, W, PH, PW, _, _>::new(
        partition_position,
        builder,
        frame_count,
        setups,
        updater,
    );
    animation.process();
    animation.take_ctx()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[test]
    fn test_name() {
        let mut animation = SimpleAnimation::<5, 5, 1, 1, _, _>::new(
            TOP_LEFT,
            PixelAnimationBuilder::new_empty(Repeat::Infinite, 5),
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

        animation.process();
        animation.take_ctx().save("arts/animation_0.gif").unwrap();
    }
}
