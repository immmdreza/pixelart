//! Root module for anything related to [`Pixel`]s: color, position, table and etc.
//!

use self::{
    color::{IntoPixelColor, PixelColor},
    position::PixelPosition,
};

pub mod canvas;
pub mod color;
pub mod position;

pub trait PixelInterface {
    /// Get the current [`PixelColor`] of this [`Pixel`].
    fn color(&self) -> &PixelColor;

    /// Get the [`PixelPosition`] of this [`Pixel`].
    fn position(&self) -> &PixelPosition;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel {
    color: PixelColor,
    /// Can't be changed.
    position: PixelPosition,
}

impl PixelInterface for Pixel {
    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }
}

impl PixelInterface for &Pixel {
    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }
}

impl PixelInterface for &mut Pixel {
    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }
}

impl Pixel {
    pub fn new(color: impl IntoPixelColor, pos: impl Into<PixelPosition>) -> Self {
        Self {
            color: color.into_pixel_color(),
            position: pos.into(),
        }
    }

    /// Updates the [`PixelColor`] of this [`Pixel`].
    ///
    /// Returning the previous color.
    pub fn update_color(&mut self, color: impl IntoPixelColor) -> PixelColor {
        std::mem::replace(&mut self.color, color.into_pixel_color())
    }
}

pub trait IntoPixel {
    fn into_pixel(self) -> Pixel;
}

impl<T> IntoPixel for T
where
    T: Into<Pixel>,
{
    fn into_pixel(self) -> Pixel {
        self.into()
    }
}

/// A set of extension methods for a read-only iterator over [`Pixel`]s.
pub trait PixelIterExt<'p, Item: PixelInterface>: Iterator<Item = Item> {
    /// Filter pixels using their [`PixelColor`].
    fn filter_color(self, color: impl IntoPixelColor) -> impl Iterator<Item = Item>
    where
        Self: Sized,
    {
        let color = color.into_pixel_color();
        self.filter(move |pixel| pixel.color() == &color)
    }
}

impl<'p, T> PixelIterExt<'p, Pixel> for T where T: Iterator<Item = Pixel> {}
impl<'p, T> PixelIterExt<'p, &'p Pixel> for T where T: Iterator<Item = &'p Pixel> {}
impl<'p, T> PixelIterExt<'p, &'p mut Pixel> for T where T: Iterator<Item = &'p mut Pixel> {}
