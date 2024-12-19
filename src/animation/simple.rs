use std::path::Path;

use image::codecs::gif::Repeat;

use crate::{
    image::DefaultImageBuffer,
    pixels::{
        canvas::{partition::CanvasPartition, SharedMutPixelCanvasExt},
        position::{IntoPixelStrictPosition, PixelStrictPosition},
        Pixel, PixelInterface,
    },
    prelude::{MaybePixel, PixelCanvas, PixelColor},
};

use super::{Animated, AnimationContext, PixelAnimationBuilder};

#[cfg(feature = "viewer")]
use crate::viewer::ViewResult;

pub struct SimpleAnimationContext<const H: usize, const W: usize, const PH: usize, const PW: usize>
{
    pub(crate) frame_count: Repeat,
    pub(crate) part: CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel>,
    pub(crate) builder: PixelAnimationBuilder,
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

    fn frame_count(&self) -> &Repeat {
        &self.frame_count
    }
}

impl<const H: usize, const W: usize, const PH: usize, const PW: usize>
    SimpleAnimationContext<H, W, PH, PW>
{
    pub fn new(
        frame_count: Repeat,
        part: CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel>,
        builder: PixelAnimationBuilder,
    ) -> Self {
        Self {
            part,
            builder,
            frame_count,
        }
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
        self.builder.view()
    }

    pub fn take_images(self) -> Vec<DefaultImageBuffer> {
        self.builder.images
    }

    pub fn part(
        &self,
    ) -> &CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel> {
        &self.part
    }

    pub fn part_mut(
        &mut self,
    ) -> &mut CanvasPartition<PH, PW, H, W, PixelCanvas<H, W, Pixel>, Pixel, MaybePixel> {
        &mut self.part
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
    partition_position: PixelStrictPosition<H, W>,
    scale: usize,
    gif_repeat: Repeat,
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
        scale: usize,
        gif_repeat: Repeat,
        frame_count: Repeat,
        setups: F1,
        updater: F2,
    ) -> Self {
        Self {
            partition_position: partition_position.into_pixel_strict_position(),
            frame_count,
            scale,
            gif_repeat,
            setups,
            updater,
        }
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

    fn create_context(&mut self) -> Self::ContextType {
        Self::ContextType {
            frame_count: self.frame_count,
            part: CanvasPartition::new(self.partition_position.clone(), PixelCanvas::default()),
            builder: PixelAnimationBuilder::new_empty(self.gif_repeat, self.scale),
        }
    }

    fn update(&mut self, ctx: &mut Self::ContextType, i: u16) -> bool {
        (self.updater)(i, ctx)
    }

    fn setup(&mut self, ctx: &mut Self::ContextType) {
        (self.setups)(ctx)
    }
}

pub fn create_simple_animation<const H: usize, const W: usize, const PH: usize, const PW: usize>(
    partition_position: impl IntoPixelStrictPosition<H, W>,
    scale: usize,
    gif_repeat: Repeat,
    frame_count: Repeat,
    setups: impl FnOnce(&mut SimpleAnimationContext<H, W, PH, PW>) + Copy,
    updater: impl FnOnce(u16, &mut SimpleAnimationContext<H, W, PH, PW>) -> bool + Copy,
) -> SimpleAnimationContext<H, W, PH, PW> {
    let mut animation = SimpleAnimation::<H, W, PH, PW, _, _>::new(
        partition_position,
        scale,
        gif_repeat,
        frame_count,
        setups,
        updater,
    );
    animation.process()
}
