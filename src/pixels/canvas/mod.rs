//! Module contains types related to a [`PixelCanvas`].

use self::{image::PixelImageBuilder, table::PixelTable};

use super::{
    color::{IntoPixelColor, PixelColor},
    position::{IntoPixelStrictPosition, PixelStrictPositionInterface, SingleCycle},
    PixelInterface,
};

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

impl<const H: usize, const W: usize> PixelCanvas<H, W> {
    pub fn new(fill_color: impl IntoPixelColor + Clone) -> Self {
        Self {
            table: PixelTable::new(fill_color),
        }
    }
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

impl<const H: usize, const W: usize> PixelCanvasInterface<H, W> for &mut PixelCanvas<H, W> {
    fn table(&self) -> &PixelTable<H, W> {
        &self.table
    }

    fn table_mut(&mut self) -> &mut PixelTable<H, W> {
        &mut self.table
    }
}

fn _fill_inside<const H: usize, const W: usize, P: PixelCanvasExt<H, W>>(
    canvas: &mut P,
    base_color: Option<PixelColor>,
    color: impl IntoPixelColor,
    point_inside: impl IntoPixelStrictPosition<H, W>,
) {
    let new_color = color.into_pixel_color();
    let pos = point_inside.into_pixel_strict_position();
    let base_color = base_color.unwrap_or(*canvas.table()[&pos].color());

    canvas.update_color_at(&pos, new_color);

    for dir in SingleCycle::new(super::position::Direction::Up) {
        if let Ok(new_pos) = pos.checked_direction(dir, 1) {
            if canvas.color_at(&new_pos) == &base_color {
                canvas.update_color_at(&new_pos, new_color);
                _fill_inside(canvas, Some(base_color), new_color, new_pos)
            }
        }
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

    /// Gets the color of a pixel at given position.
    fn color_at(&self, pos: impl PixelStrictPositionInterface<H, W>) -> &PixelColor {
        self.table()[pos].color()
    }

    /// Update color of a pixel at the given position.
    fn update_color_at(
        &mut self,
        pos: impl PixelStrictPositionInterface<H, W>,
        color: impl IntoPixelColor,
    ) -> PixelColor {
        self.table_mut()[pos].update_color(color)
    }

    /// Keep filling pixels with new color until we encounter a new color.
    fn fill_inside(
        &mut self,
        color: impl IntoPixelColor,
        point_inside: impl IntoPixelStrictPosition<H, W>,
    ) where
        Self: Sized,
    {
        _fill_inside::<H, W, _>(self, None, color, point_inside)
    }
}

impl<const H: usize, const W: usize, T> PixelCanvasExt<H, W> for T where
    T: PixelCanvasInterface<H, W>
{
}

#[cfg(test)]
mod tests {
    use crate::pixels::{
        color::PixelColorExt,
        position::{PixelPositionInterface, StrictPositions},
        PixelIterExt, PixelIterMutExt,
    };

    use self::image::PixelImageStyle;

    use super::*;

    #[test]
    fn test_fill_inside() {
        let mut canvas = PixelCanvas::<5>::default();
        canvas
            .iter_pixels_mut()
            .filter_position(|p| p.column() == p.row())
            .update_colors(PixelColor::RED);

        canvas.fill_inside(PixelColor::BLUE, StrictPositions::BottomLeft);

        canvas.update_color_at(StrictPositions::TopRight, PixelColor::BLACK);

        let image_builder = canvas.image_builder(PixelImageStyle::default().with_scale(5));
        image_builder.save("arts/fill_inside.png").unwrap();
    }
}
