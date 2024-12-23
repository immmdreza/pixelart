use std::{error::Error, fmt::Display};

use image::{Rgb, Rgba};
use thiserror::Error;

pub mod colors {
    use super::PixelColor;

    /// Color **White**.
    pub const WHITE: PixelColor = PixelColor::splat(u8::MAX);

    /// Color **Black**.
    pub const BLACK: PixelColor = PixelColor::splat(u8::MIN);

    /// Color **Red**.
    pub const RED: PixelColor = PixelColor::from_red(u8::MAX);

    /// Color **Green**.
    pub const GREEN: PixelColor = PixelColor::from_green(u8::MAX);

    /// Color **Blue**.
    pub const BLUE: PixelColor = PixelColor::from_blue(u8::MAX);

    /// Color **Yellow**.
    pub const YELLOW: PixelColor = PixelColor::from_red(u8::MAX).green(u8::MAX);

    /// Color **Cyan**.
    pub const CYAN: PixelColor = PixelColor::from_green(u8::MAX).blue(u8::MAX);

    /// Color **Magenta**.
    pub const MAGENTA: PixelColor = PixelColor::from_red(u8::MAX).blue(u8::MAX);
}

pub trait RgbaInterface {
    fn rgba(&self) -> Rgba<u8>;
}

/// An interface for [`PixelColor`].
pub trait PixelColorInterface {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;

    fn rgb(&self) -> Rgb<u8> {
        Rgb([self.r(), self.g(), self.b()])
    }
}

/// Simple RGB color of a pixel.
///
/// The default value is White (`u8::MAX` for all) and not Black (`u8::MIN` for all).
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbaInterface for PixelColor {
    fn rgba(&self) -> Rgba<u8> {
        Rgba([self.r(), self.g(), self.b(), u8::MAX])
    }
}

impl RgbaInterface for &PixelColor {
    fn rgba(&self) -> Rgba<u8> {
        Rgba([self.r(), self.g(), self.b(), u8::MAX])
    }
}

impl RgbaInterface for &mut PixelColor {
    fn rgba(&self) -> Rgba<u8> {
        Rgba([self.r(), self.g(), self.b(), u8::MAX])
    }
}

impl RgbaInterface for Option<PixelColor> {
    fn rgba(&self) -> Rgba<u8> {
        match self {
            Some(color) => color.rgba(),
            None => Rgba([0, 0, 0, 0]),
        }
    }
}

impl RgbaInterface for &Option<PixelColor> {
    fn rgba(&self) -> Rgba<u8> {
        match self {
            Some(color) => color.rgba(),
            None => Rgba([0, 0, 0, 0]),
        }
    }
}

impl RgbaInterface for &mut Option<PixelColor> {
    fn rgba(&self) -> Rgba<u8> {
        match self {
            Some(color) => color.rgba(),
            None => Rgba([0, 0, 0, 0]),
        }
    }
}

impl Display for PixelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.r, self.g, self.b) {
            (0, 0, 0) => f.write_str("black"),
            (255, 255, 255) => f.write_str("white"),
            (255, 0, 0) => f.write_str("red"),
            (0, 255, 0) => f.write_str("green"),
            (0, 0, 255) => f.write_str("blue"),
            (r, g, b) => write!(f, "({r}, {g}, {b})"),
        }
    }
}

impl PixelColorInterface for PixelColor {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

impl Default for PixelColor {
    fn default() -> Self {
        Self {
            r: u8::MAX,
            g: u8::MAX,
            b: u8::MAX,
        }
    }
}

impl PixelColor {
    /// Create a new [`PixelColor`] using rgb values from (0 to 255).
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new [`PixelColor`] using the same value for rgb from (0 to 255).
    pub const fn splat(rgb: u8) -> Self {
        Self {
            r: rgb,
            g: rgb,
            b: rgb,
        }
    }

    /// Create a new [`PixelColor`] using r (red) value only from (0 to 255).
    ///
    /// Others are set to 0.
    pub const fn from_red(r: u8) -> Self {
        Self { r, g: 0, b: 0 }
    }

    /// Create a new [`PixelColor`] using g (green) value only from (0 to 255).
    ///
    /// Others are set to 0.
    pub const fn from_green(g: u8) -> Self {
        Self { r: 0, g, b: 0 }
    }

    /// Create a new [`PixelColor`] using b (blue) value only from (0 to 255).
    ///
    /// Others are set to 0.
    pub const fn from_blue(b: u8) -> Self {
        Self { r: 0, g: 0, b }
    }

    pub const fn red(self, r: u8) -> Self {
        Self { r, ..self }
    }

    pub const fn green(self, g: u8) -> Self {
        Self { g, ..self }
    }

    pub const fn blue(self, b: u8) -> Self {
        Self { b, ..self }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn map(&self, mapper: impl FnOnce(&PixelColor) -> PixelColor) -> PixelColor {
        mapper(self)
    }

    pub fn map_r(&self, mapper: impl FnOnce(u8) -> u8) -> PixelColor {
        Self {
            r: mapper(self.r),
            g: self.g,
            b: self.b,
        }
    }

    pub fn map_g(&self, mapper: impl FnOnce(u8) -> u8) -> PixelColor {
        Self {
            r: self.r,
            g: mapper(self.g),
            b: self.b,
        }
    }

    pub fn map_b(&self, mapper: impl FnOnce(u8) -> u8) -> PixelColor {
        Self {
            r: self.r,
            g: self.g,
            b: mapper(self.b),
        }
    }

    pub fn map_all(&self, mapper: impl FnOnce(u8) -> u8 + Copy) -> PixelColor {
        Self {
            r: mapper(self.r),
            g: mapper(self.g),
            b: mapper(self.b),
        }
    }
}

impl TryFrom<Option<PixelColor>> for PixelColor {
    type Error = ColorConversionError<NoInformation>;

    fn try_from(value: Option<PixelColor>) -> Result<Self, Self::Error> {
        value.ok_or(ColorConversionError(NoInformation))
    }
}

impl From<(u8, u8, u8)> for PixelColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        PixelColor { r, g, b }
    }
}

impl From<[u8; 3]> for PixelColor {
    fn from(rgb: [u8; 3]) -> Self {
        PixelColor {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}

impl From<u8> for PixelColor {
    fn from(rgb: u8) -> Self {
        PixelColor::splat(rgb)
    }
}

pub trait PixelColorExt: PixelColorInterface {
    /// Color **White**.
    const WHITE: PixelColor = PixelColor::splat(u8::MAX);

    /// Color **Black**.
    const BLACK: PixelColor = PixelColor::splat(u8::MIN);

    /// Color **Red**.
    const RED: PixelColor = PixelColor::from_red(u8::MAX);

    /// Color **Green**.
    const GREEN: PixelColor = PixelColor::from_green(u8::MAX);

    /// Color **Blue**.
    const BLUE: PixelColor = PixelColor::from_blue(u8::MAX);

    /// Color **Yellow**.
    const YELLOW: PixelColor = PixelColor::from_red(u8::MAX).green(u8::MAX);

    /// Color **Cyan**.
    const CYAN: PixelColor = PixelColor::from_green(u8::MAX).blue(u8::MAX);

    /// Color **Magenta**.
    const MAGENTA: PixelColor = PixelColor::from_red(u8::MAX).blue(u8::MAX);

    /// Get [`PixelColor`] struct from a type that implements [`PixelColorInterface`].
    fn pixel_color(&self) -> PixelColor {
        PixelColor {
            r: self.r(),
            g: self.g(),
            b: self.b(),
        }
    }
}

impl<T> PixelColorExt for T where T: PixelColorInterface {}

impl From<PixelColor> for Rgba<u8> {
    fn from(val: PixelColor) -> Self {
        val.rgba()
    }
}

impl From<&PixelColor> for Rgba<u8> {
    fn from(val: &PixelColor) -> Self {
        val.rgba()
    }
}

impl From<&mut PixelColor> for Rgba<u8> {
    fn from(val: &mut PixelColor) -> Self {
        val.rgba()
    }
}

#[cfg(test)]
mod pixel_color_tests {
    use super::*;

    #[test]
    fn default_color_should_be_white() {
        assert_eq!(
            PixelColor::default(),
            PixelColor {
                r: 255,
                b: 255,
                g: 255
            }
        );

        assert_eq!(PixelColor::default(), PixelColor::WHITE);
    }
}

#[derive(Debug, Error)]
#[error("Failed converting color ({0}).")]
pub struct ColorConversionError<T: Error>(T);

#[derive(Debug, Error)]
#[error("No information given")]
pub struct NoInformation;

pub type DefaultColorConversionError = ColorConversionError<NoInformation>;
