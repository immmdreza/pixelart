//! Root module for anything related to [`Pixel`]s: color, position, table and etc.
//!

use std::fmt::Display;

use position::PixelPosition;

use self::color::PixelColor;

pub mod canvas;
pub mod color;
pub mod maybe;
pub mod position;

pub trait PixelInitializer: PixelInterface {
    fn new(color: impl Into<Self::ColorType>) -> Self;
}

pub trait PixelInterface {
    type ColorType;

    /// Indicates if this pixel has a color.
    fn has_color(&self) -> bool;

    /// Get the current [`PixelColor`] of this [`Pixel`].
    fn color(&self) -> &Self::ColorType;
}

pub trait PixelMutInterface: PixelInterface {
    /// Updates the [`PixelColor`] of this [`Pixel`].
    ///
    /// Returning the previous color.
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel {
    pub color: PixelColor,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pixel({})", self.color)
    }
}

impl PixelInterface for Pixel {
    type ColorType = PixelColor;

    fn color(&self) -> &PixelColor {
        &self.color
    }

    fn has_color(&self) -> bool {
        true
    }
}

impl PixelMutInterface for Pixel {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType {
        std::mem::replace(&mut self.color, color.into())
    }
}

impl PixelInterface for &Pixel {
    type ColorType = PixelColor;

    fn color(&self) -> &PixelColor {
        &self.color
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

    fn has_color(&self) -> bool {
        true
    }
}

impl PixelMutInterface for &mut Pixel {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType {
        std::mem::replace(&mut self.color, color.into())
    }
}

impl PixelInitializer for Pixel {
    fn new(color: impl Into<PixelColor>) -> Self {
        Self {
            color: color.into(),
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
pub trait PixelIterExt<P: PixelInterface>: Iterator<Item = PixelData<P>> {
    /// Filter pixels using their [`PixelColor`].
    fn filter_color(
        self,
        color: impl Into<<P as PixelInterface>::ColorType>,
    ) -> impl Iterator<Item = PixelData<P>>
    where
        Self: Sized,
        <P as PixelInterface>::ColorType: PartialEq,
    {
        let color: <P as PixelInterface>::ColorType = color.into();
        self.filter(move |pixel| pixel.color() == &color)
    }

    /// Filter pixels based on their [`PixelPosition`].
    fn filter_position<F>(self, mut predicate: F) -> impl Iterator<Item = PixelData<P>>
    where
        Self: Sized,
        F: FnMut(&PixelPosition) -> bool,
    {
        self.filter(move |pixel| predicate(pixel.position()))
    }
}

// impl<'p, T> PixelIterExt<Pixel> for T where T: Iterator<Item = Pixel> {}
// impl<'p, T> PixelIterExt<&'p Pixel> for T where T: Iterator<Item = &'p Pixel> {}
// impl<'p, T> PixelIterExt<&'p mut Pixel> for T where T: Iterator<Item = &'p mut Pixel> {}

impl<T, P: PixelInterface> PixelIterExt<P> for T where T: Iterator<Item = PixelData<P>> {}

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

impl<T, P: PixelMutInterface> PixelIterMutExt<'_, P> for T where T: Iterator<Item = P> {}

#[derive(Debug)]
pub struct PixelData<P: PixelInterface> {
    pub pixel: P,
    position: PixelPosition,
}

impl<P: PixelMutInterface> PixelMutInterface for PixelData<P> {
    fn update_color(&mut self, color: impl Into<Self::ColorType>) -> Self::ColorType {
        self.pixel.update_color(color)
    }
}

impl<P: PixelInterface> PixelInterface for PixelData<P> {
    type ColorType = P::ColorType;

    fn has_color(&self) -> bool {
        self.pixel.has_color()
    }

    fn color(&self) -> &Self::ColorType {
        self.pixel.color()
    }
}

impl<P: PixelInterface> PixelData<P> {
    pub fn position(&self) -> &PixelPosition {
        &self.position
    }
}
