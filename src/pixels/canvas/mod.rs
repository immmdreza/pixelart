//! Module contains types related to a [`PixelCanvas`].

use self::{image::PixelImageBuilder, table::PixelTable};

use super::color::{IntoPixelColor, PixelColor};

pub mod image;
pub mod row;
pub mod table;

/// Interface that any pixel canvas may want to implement.
///
/// Using this we can have access to later extension methods.
pub trait PixelCanvasInterface<const H: usize, const W: usize> {
    /// A read-only reference to underlying [`PixelTable`].
    fn table(&self) -> &PixelTable<H, W>;

    /// A mutable reference to underlying [`PixelTable`].
    fn table_mut(&mut self) -> &mut PixelTable<H, W>;
}

/// A [`PixelCanvas`], the highest level api to work and clear interact
/// with the underlying [`PixelTable`] and pixels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PixelCanvas<const H: usize, const W: usize = H> {
    table: PixelTable<H, W>,
}

impl<const H: usize, const W: usize> std::ops::Deref for PixelCanvas<H, W> {
    type Target = PixelTable<H, W>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<const H: usize, const W: usize> std::ops::DerefMut for PixelCanvas<H, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<const H: usize, const W: usize> PixelCanvasInterface<H, W> for PixelCanvas<H, W> {
    fn table(&self) -> &PixelTable<H, W> {
        &self.table
    }

    fn table_mut(&mut self) -> &mut PixelTable<H, W> {
        &mut self.table
    }
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
pub trait PixelCanvasExt<const H: usize, const W: usize>: PixelCanvasInterface<H, W> {
    /// Updates every pixel's color to default which is white.
    fn clear(&mut self) {
        self.fill(PixelColor::default())
    }

    /// Fills all pixels color.
    fn fill(&mut self, color: impl IntoPixelColor) {
        let color = color.into_pixel_color();
        self.table_mut().for_each_pixel_mut(|pixel| {
            pixel.update_color(color.clone());
        })
    }

    fn image_builder(&self, style: image::PixelImageStyle) -> PixelImageBuilder<H, W, Self>
    where
        Self: Sized,
    {
        PixelImageBuilder::new(self, style)
    }

    fn image_builder_default(&self) -> PixelImageBuilder<H, W, Self>
    where
        Self: Sized,
    {
        PixelImageBuilder::new_default_style(self)
    }
}

impl<const H: usize, const W: usize, T> PixelCanvasExt<H, W> for T where
    T: PixelCanvasInterface<H, W>
{
}
