use super::{
    canvas::{
        PixelCanvasInterface, PixelCanvasMutInterface, SharedMutPixelCanvasExt,
        SharedPixelCanvasExt,
    },
    color::PixelColor,
    position::{PixelPosition, PixelStrictPositionInterface},
    PixelInitializer, PixelInterface, PixelIterExt, PixelIterMutExt, PixelMutInterface,
};

/// A pixel that may not have any effect on the color at this position.
pub struct MaybePixel {
    pub color: Option<PixelColor>,
    position: PixelPosition,
}

impl PixelInitializer for MaybePixel {
    fn new(color: impl Into<Option<PixelColor>>, position: impl Into<PixelPosition>) -> Self {
        Self {
            color: color.into(),
            position: position.into(),
        }
    }
}

// false here should indicate that this type do not support accessing color (at compile time).
impl PixelMutInterface for MaybePixel {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Option<PixelColor> {
        std::mem::replace(&mut self.color, color.into())
    }
}

impl PixelMutInterface for &mut MaybePixel {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Option<PixelColor> {
        std::mem::replace(&mut self.color, color.into())
    }
}

// false here should indicate that this type do not support accessing color (at compile time).
impl PixelInterface for MaybePixel {
    type ColorType = Option<PixelColor>;

    fn has_color(&self) -> bool {
        self.color.is_some()
    }

    fn color(&self) -> &Self::ColorType {
        &self.color
    }

    fn position(&self) -> &super::position::PixelPosition {
        &self.position
    }
}

impl PixelInterface for &MaybePixel {
    type ColorType = Option<PixelColor>;

    fn has_color(&self) -> bool {
        self.color.is_some()
    }

    fn color(&self) -> &Self::ColorType {
        &self.color
    }

    fn position(&self) -> &super::position::PixelPosition {
        &self.position
    }
}

impl PixelInterface for &mut MaybePixel {
    type ColorType = Option<PixelColor>;

    fn has_color(&self) -> bool {
        self.color.is_some()
    }

    fn color(&self) -> &Self::ColorType {
        &self.color
    }

    fn position(&self) -> &super::position::PixelPosition {
        &self.position
    }
}

impl<'p, T> PixelIterExt<MaybePixel> for T where T: Iterator<Item = MaybePixel> {}
impl<'p, T> PixelIterExt<&'p MaybePixel> for T where T: Iterator<Item = &'p MaybePixel> {}
impl<'p, T> PixelIterExt<&'p mut MaybePixel> for T where T: Iterator<Item = &'p mut MaybePixel> {}

impl<'p, T> PixelIterMutExt<'p, &'p mut MaybePixel> for T where
    T: Iterator<Item = &'p mut MaybePixel>
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is only implemented for canvas of [`MaybePixel`] type.
pub trait MaybePixelCanvasExt<const H: usize, const W: usize>:
    SharedPixelCanvasExt<H, W, MaybePixel>
{
    fn iter_existing_pixels(&self) -> impl Iterator<Item = &MaybePixel> {
        self.table().iter_pixels().filter(|p| p.has_color())
    }
}

impl<const H: usize, const W: usize, T> MaybePixelCanvasExt<H, W> for T where
    T: PixelCanvasInterface<H, W, MaybePixel>
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is only implemented for canvas of [`MaybePixel`] type.
pub trait MaybePixelMutCanvasExt<const H: usize, const W: usize>:
    SharedMutPixelCanvasExt<H, W, MaybePixel>
{
    fn clear_pixel(&mut self, pos: impl PixelStrictPositionInterface<H, W>) -> Option<PixelColor> {
        self.update_color_at(pos, Option::None)
    }
}

impl<const H: usize, const W: usize, T> MaybePixelMutCanvasExt<H, W> for T where
    T: PixelCanvasMutInterface<H, W, MaybePixel>
{
}
