//! Root module for anything related to [`Pixel`]s: color, position, table and etc.
//!

use std::fmt::Display;

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

pub trait PixelMutInterface: PixelInterface {
    /// Updates the [`PixelColor`] of this [`Pixel`].
    ///
    /// Returning the previous color.
    fn update_color(&mut self, color: impl IntoPixelColor) -> PixelColor;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel {
    color: PixelColor,
    /// Can't be changed.
    position: PixelPosition,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pixel({}, {})", self.color, self.position)
    }
}

impl PixelInterface for Pixel {
    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }
}

impl PixelMutInterface for Pixel {
    fn update_color(&mut self, color: impl IntoPixelColor) -> PixelColor {
        std::mem::replace(&mut self.color, color.into_pixel_color())
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

impl PixelMutInterface for &mut Pixel {
    fn update_color(&mut self, color: impl IntoPixelColor) -> PixelColor {
        std::mem::replace(&mut self.color, color.into_pixel_color())
    }
}

impl Pixel {
    pub fn new(color: impl IntoPixelColor, pos: impl Into<PixelPosition>) -> Self {
        Self {
            color: color.into_pixel_color(),
            position: pos.into(),
        }
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

/// A set of extension methods for a shared (owned, ref, mut) iterator over [`Pixel`]s.
pub trait PixelIterExt<Item: PixelInterface>: Iterator<Item = Item> {
    /// Filter pixels using their [`PixelColor`].
    fn filter_color(self, color: impl IntoPixelColor) -> impl Iterator<Item = Item>
    where
        Self: Sized,
    {
        let color = color.into_pixel_color();
        self.filter(move |pixel| pixel.color() == &color)
    }

    /// Filter pixels based on their [`PixelPosition`].
    fn filter_position<P>(self, mut predicate: P) -> impl Iterator<Item = Item>
    where
        Self: Sized,
        P: FnMut(&PixelPosition) -> bool,
    {
        self.filter(move |pixel| predicate(pixel.position()))
    }
}

impl<'p, T> PixelIterExt<Pixel> for T where T: Iterator<Item = Pixel> {}
impl<'p, T> PixelIterExt<&'p Pixel> for T where T: Iterator<Item = &'p Pixel> {}
impl<'p, T> PixelIterExt<&'p mut Pixel> for T where T: Iterator<Item = &'p mut Pixel> {}

/// A set of extension methods for a mutable only iterator over [`Pixel`]s.
pub trait PixelIterMutExt<'p, Item: PixelInterface>: Iterator<Item = &'p mut Pixel> {
    /// Updates the [`PixelColor`] for all of this iterator members.
    fn update_colors(self, color: impl IntoPixelColor)
    where
        Self: Sized,
    {
        let color = color.into_pixel_color();
        self.for_each(move |pixel| {
            pixel.update_color(color.clone());
        })
    }
}

impl<'p, T> PixelIterMutExt<'p, &'p mut Pixel> for T where T: Iterator<Item = &'p mut Pixel> {}
