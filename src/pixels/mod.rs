//! Root module for anything related to [`Pixel`]s: color, position, table and etc.
//!

use self::{
    color::{IntoPixelColor, PixelColor},
    position::PixelPosition,
};

pub mod canvas;
pub mod color;
pub mod position;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel {
    color: PixelColor,
    position: PixelPosition,
}

impl Pixel {
    pub fn new(color: impl IntoPixelColor, pos: impl Into<PixelPosition>) -> Self {
        Self {
            color: color.into_pixel_color(),
            position: pos.into(),
        }
    }

    /// Get the current [`PixelColor`] of this [`Pixel`].
    pub fn color(&self) -> &PixelColor {
        &self.color
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
