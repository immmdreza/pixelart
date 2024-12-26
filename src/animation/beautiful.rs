use std::marker::PhantomData;

use image::codecs::gif::Repeat;

use crate::pixels::{color::RgbaInterface, Pixel, PixelInitializer, PixelInterface};

use super::{
    Animated, AnimationContext, AnimationFrameFinisher, AnimationFrameFinisherEmpty,
    AnimationFrameFinisherHolder,
};

pub struct BeautifulAnimation<Extras, const H: usize, const W: usize = H, P: PixelInterface = Pixel>
{
    updater: Box<dyn Fn(&mut AnimationContext<H, W, P>, u16, &mut Extras) -> bool>,
    setup: Box<dyn Fn(&mut AnimationContext<H, W, P>, &mut Extras)>,
    finisher: Box<dyn AnimationFrameFinisher<AnimationContext<H, W, P>>>,
    frame_count: Repeat,
    extras_holder: Option<Extras>,
    _phantom: PhantomData<(P, Extras)>,
}

impl<const H: usize, const W: usize, P: PixelInterface + 'static, Extras>
    BeautifulAnimation<Extras, H, W, P>
{
    pub fn new_with_finisher(
        frame_count: Repeat,
        extras: Extras,
        setup: impl Fn(&mut AnimationContext<H, W, P>, &mut Extras) + 'static,
        updater: impl Fn(&mut AnimationContext<H, W, P>, u16, &mut Extras) -> bool + 'static,
        finisher: impl Fn(&mut AnimationContext<H, W, P>, u16) + 'static,
    ) -> Self {
        Self {
            frame_count,
            extras_holder: Some(extras),
            updater: Box::new(updater),
            setup: Box::new(setup),
            finisher: Box::new(AnimationFrameFinisherHolder::new(finisher)),
            _phantom: PhantomData,
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + 'static, Extras>
    BeautifulAnimation<Extras, H, W, P>
{
    pub fn new(
        frame_count: Repeat,
        extras: Extras,
        setup: impl Fn(&mut AnimationContext<H, W, P>, &mut Extras) + 'static,
        updater: impl Fn(&mut AnimationContext<H, W, P>, u16, &mut Extras) -> bool + 'static,
    ) -> Self {
        Self {
            frame_count,
            extras_holder: Some(extras),
            updater: Box::new(updater),
            setup: Box::new(setup),
            finisher: Box::new(AnimationFrameFinisherEmpty::new()),
            _phantom: PhantomData,
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface, Extras> Animated<H, W, P>
    for BeautifulAnimation<Extras, H, W, P>
where
    P: PixelInitializer,
    <P as PixelInterface>::ColorType: RgbaInterface + Default + Clone,
{
    type ContextType = AnimationContext<H, W, P>;

    fn create_context(&mut self) -> Self::ContextType {
        AnimationContext::<H, W, P>::new(self.frame_count)
    }

    fn setup(&mut self, ctx: &mut Self::ContextType) {
        let mut extra = self
            .extras_holder
            .take()
            .expect("Extra holder should contain something.");
        (self.setup)(ctx, &mut extra);
        self.extras_holder = Some(extra);
    }

    fn update(&mut self, ctx: &mut Self::ContextType, i: u16) -> bool {
        let mut extra = self
            .extras_holder
            .take()
            .expect("Extra holder should contain something.");
        let result = (self.updater)(ctx, i, &mut extra);
        self.extras_holder = Some(extra);
        result
    }

    fn finisher(&mut self, ctx: &mut Self::ContextType, i: u16) {
        self.finisher.run_finisher(ctx, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let _animation = BeautifulAnimation::<_, 5>::new(
            Repeat::Finite(10),
            (10u32, "Hello World".to_string()),
            |_ctx, (_number, _string)| {},
            |_ctx, _i, (_number, _string)| true,
        );
    }
}
