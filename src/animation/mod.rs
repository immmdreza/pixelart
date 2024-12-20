use std::marker::PhantomData;
use std::{fs::File, path::Path};

use image::{codecs::gif::GifEncoder, Frame, ImageResult};

use crate::image::DefaultImageBuffer;
use crate::pixels::{canvas::SharedPixelCanvasExt, color::RgbaInterface};
use crate::pixels::{Pixel, PixelInitializer};
use crate::{
    pixels::{canvas::PixelCanvasInterface, PixelInterface},
    prelude::PixelCanvas,
};

pub use image::codecs::gif::Repeat;

#[cfg(feature = "viewer")]
use crate::viewer::{view, ViewResult};

pub mod layered;
pub mod simple;

#[derive(Debug)]
pub struct PixelAnimationBuilder {
    pub(crate) repeat: Repeat,
    pub(crate) scale: usize,
    pub(crate) images: Vec<DefaultImageBuffer>,
}

impl Default for PixelAnimationBuilder {
    fn default() -> Self {
        Self {
            repeat: Repeat::Infinite,
            scale: Default::default(),
            images: Default::default(),
        }
    }
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

pub trait AnimatedContext<const H: usize, const W: usize, P: PixelInterface> {
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

#[derive(Debug)]
pub struct AnimationContext<const H: usize, const W: usize = H, P: PixelInterface = Pixel> {
    frames: Repeat,
    pub builder: PixelAnimationBuilder,
    pub canvas: PixelCanvas<H, W, P>,
}

impl<const H: usize, const W: usize, P: PixelInterface> AnimationContext<H, W, P> {
    pub fn new(repeat: Repeat) -> Self
    where
        <P as PixelInterface>::ColorType: std::default::Default,
        <P as PixelInterface>::ColorType: Clone,
        P: PixelInitializer,
    {
        Self {
            frames: repeat,
            builder: Default::default(),
            canvas: Default::default(),
        }
    }

    pub fn with_scale(mut self, scale: usize) -> Self {
        self.builder.scale = scale;
        self
    }

    pub fn with_gif_repeat(mut self, repeat: Repeat) -> Self {
        self.builder.repeat = repeat;
        self
    }

    pub fn with_modified_canvas(
        mut self,
        modifier: impl FnOnce(&mut PixelCanvas<H, W, P>),
    ) -> Self {
        modifier(&mut self.canvas);
        self
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> AnimatedContext<H, W, P>
    for AnimationContext<H, W, P>
{
    fn frame_count(&self) -> &Repeat {
        &self.frames
    }

    fn builder(&self) -> &PixelAnimationBuilder {
        &self.builder
    }

    fn builder_mut(&mut self) -> &mut PixelAnimationBuilder {
        &mut self.builder
    }

    fn canvas(&self) -> &PixelCanvas<H, W, P> {
        &self.canvas
    }

    fn canvas_mut(&mut self) -> &mut PixelCanvas<H, W, P> {
        &mut self.canvas
    }
}

pub trait Animated<const H: usize, const W: usize, P: PixelInterface>
where
    <P as PixelInterface>::ColorType: RgbaInterface,
{
    type ContextType: AnimatedContext<H, W, P>;

    fn create_context(&mut self) -> Self::ContextType;

    fn setup(&mut self, ctx: &mut Self::ContextType);
    fn update(&mut self, ctx: &mut Self::ContextType, i: u16) -> bool;

    fn create(&mut self) -> <Self as Animated<H, W, P>>::ContextType {
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

pub struct Animation<
    C: AnimatedContext<H, W, P>,
    const H: usize,
    const W: usize,
    P: PixelInterface,
    B: FnOnce() -> C + Copy,
    S: FnOnce(&mut C) + Copy,
    U: FnOnce(&mut C, u16) -> bool + Copy,
> {
    context_builder: B,
    updater: U,
    setup: S,
    _phantom: PhantomData<(C, P)>,
}

impl<
        C: AnimatedContext<H, W, P>,
        const H: usize,
        const W: usize,
        P: PixelInterface,
        B: FnOnce() -> C + Copy,
        S: FnOnce(&mut C) + Copy,
        U: FnOnce(&mut C, u16) -> bool + Copy,
    > Animation<C, H, W, P, B, S, U>
{
    pub fn new(context_builder: B, setup: S, updater: U) -> Self {
        Self {
            context_builder,
            updater,
            setup,
            _phantom: PhantomData,
        }
    }
}

impl<
        C: AnimatedContext<H, W, P>,
        const H: usize,
        const W: usize,
        P: PixelInterface,
        B: FnOnce() -> C + Copy,
        S: FnOnce(&mut C) + Copy,
        U: FnOnce(&mut C, u16) -> bool + Copy,
    > Animated<H, W, P> for Animation<C, H, W, P, B, S, U>
where
    <P as PixelInterface>::ColorType: RgbaInterface,
{
    type ContextType = C;

    fn create_context(&mut self) -> Self::ContextType {
        (self.context_builder)()
    }

    fn setup(&mut self, ctx: &mut Self::ContextType) {
        (self.setup)(ctx)
    }

    fn update(&mut self, ctx: &mut Self::ContextType, i: u16) -> bool {
        (self.updater)(ctx, i)
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

        animation.create().save("arts/animation_0.gif").unwrap();
    }

    #[test]
    fn feature() {
        Animation::new(
            || AnimationContext::<5>::new(Repeat::Finite(10)).with_scale(5),
            |ctx| {
                ctx.canvas_mut().fill(BLACK);
            },
            |ctx, i| {
                let new_color = ctx.canvas()[TOP_LEFT]
                    .color()
                    .map_all(|x| x + (i as u8 * 5));
                ctx.canvas_mut().fill(new_color);
                true
            },
        )
        .create()
        .builder
        .save("arts/animation_3.gif")
        .unwrap();
    }
}
