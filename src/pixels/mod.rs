//! Root module for anything related to [`Pixel`]s: color, position, table and etc.
//!

use std::fmt::Display;

use self::{color::PixelColor, position::PixelPosition};

pub mod canvas;
pub mod color;
pub mod maybe;
pub mod position;

pub trait PixelInitializer: PixelInterface {
    fn new(color: impl Into<Self::ColorType>, position: impl Into<PixelPosition>) -> Self;
}

pub trait PixelInterface {
    type ColorType;

    /// Indicates if this pixel has a color.
    fn has_color(&self) -> bool;

    /// Get the current [`PixelColor`] of this [`Pixel`].
    fn color(&self) -> &Self::ColorType;

    /// Get the [`PixelPosition`] of this [`Pixel`].
    fn position(&self) -> &PixelPosition;
}

pub trait PixelMutInterface: PixelInterface {
    /// Updates the [`PixelColor`] of this [`Pixel`].
    ///
    /// Returning the previous color.
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType;
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
    type ColorType = PixelColor;

    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }

    fn has_color(&self) -> bool {
        true
    }
}

impl PixelMutInterface for Pixel {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType {
        std::mem::replace(&mut self.color, color.into()).into()
    }
}

impl PixelInterface for &Pixel {
    type ColorType = PixelColor;

    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }

    fn has_color(&self) -> bool {
        true
    }
}

impl PixelInterface for &mut Pixel {
    type ColorType = PixelColor;

    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn position(&self) -> &PixelPosition {
        &self.position
    }

    fn has_color(&self) -> bool {
        true
    }
}

impl PixelMutInterface for &mut Pixel {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType {
        std::mem::replace(&mut self.color, color.into()).into()
    }
}

impl PixelInitializer for Pixel {
    fn new(color: impl Into<PixelColor>, position: impl Into<PixelPosition>) -> Self {
        Self {
            color: color.into(),
            position: position.into(),
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
    fn filter_color(
        self,
        color: impl Into<<Self::Item as PixelInterface>::ColorType>,
    ) -> impl Iterator<Item = Item>
    where
        Self: Sized,
        <Item as PixelInterface>::ColorType: PartialEq,
    {
        let color: <Self::Item as PixelInterface>::ColorType = color.into();
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
pub trait PixelIterMutExt<'p, Item: PixelMutInterface>: Iterator<Item = Item> {
    /// Updates the [`PixelColor`] for all of this iterator members.
    fn update_colors(self, color: impl Into<<Self::Item as PixelInterface>::ColorType>)
    where
        Self: Sized,
        <Self::Item as PixelInterface>::ColorType: Clone,
    {
        let color = color.into();
        self.for_each(move |mut pixel| {
            pixel.update_color(color.clone());
        })
    }
}

impl<'p, T> PixelIterMutExt<'p, &'p mut Pixel> for T where T: Iterator<Item = &'p mut Pixel> {}
