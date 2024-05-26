//! Module contains types related to a [`PixelCanvas`].

use self::{drawable::Drawable, image::PixelImageBuilder, table::PixelTable};

use super::{
    color::{IntoPixelColor, PixelColor},
    position::{IntoPixelStrictPosition, PixelStrictPositionInterface, SingleCycle},
    Pixel, PixelInitializer, PixelInterface, PixelMutInterface,
};

pub mod drawable;
pub mod image;
pub mod row;
pub mod table;
pub mod templates;

/// Interface that any pixel canvas may want to implement.
///
/// Using this we can have access to later extension methods.
pub trait PixelCanvasInterface<const H: usize, const W: usize, P: PixelInterface> {
    /// A read-only reference to underlying [`PixelTable`].
    fn table(&self) -> &PixelTable<H, W, P>;

    /// A mutable reference to underlying [`PixelTable`].
    fn table_mut(&mut self) -> &mut PixelTable<H, W, P>;
}

/// A [`PixelCanvas`], the highest level api to work and clear interact
/// with the underlying [`PixelTable`] and pixels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelCanvas<const H: usize, const W: usize = H, P: PixelInterface = Pixel> {
    table: PixelTable<H, W, P>,
}

impl<const H: usize, const W: usize, P> Default for PixelCanvas<H, W, P>
where
    P: PixelInterface + PixelInitializer,
    <P as PixelInterface>::ColorType: Default + Clone,
{
    fn default() -> Self {
        Self {
            table: Default::default(),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + PixelInitializer> PixelCanvas<H, W, P> {
    pub fn new(fill_color: impl Into<P::ColorType> + Clone) -> Self {
        Self {
            table: PixelTable::new(fill_color),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> std::ops::Deref for PixelCanvas<H, W, P> {
    type Target = PixelTable<H, W, P>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> std::ops::DerefMut
    for PixelCanvas<H, W, P>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> PixelCanvasInterface<H, W, P>
    for PixelCanvas<H, W, P>
{
    fn table(&self) -> &PixelTable<H, W, P> {
        &self.table
    }

    fn table_mut(&mut self) -> &mut PixelTable<H, W, P> {
        &mut self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> PixelCanvasInterface<H, W, P>
    for &mut PixelCanvas<H, W, P>
{
    fn table(&self) -> &PixelTable<H, W, P> {
        &self.table
    }

    fn table_mut(&mut self) -> &mut PixelTable<H, W, P> {
        &mut self.table
    }
}

fn _fill_inside<const H: usize, const W: usize, I: PixelCanvasExt<H, W>>(
    canvas: &mut I,
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
///
/// This trait is implemented for any canvas of [`PixelInterface`].
pub trait SharedPixelCanvasExt<const H: usize, const W: usize, P: PixelInterface>:
    PixelCanvasInterface<H, W, P>
{
    /// Gets the color of a pixel at given position.
    fn color_at<'a>(&'a self, pos: impl PixelStrictPositionInterface<H, W>) -> &P::ColorType
    where
        P: 'a,
    {
        self.table()[pos].color()
    }
}

impl<const H: usize, const W: usize, T, P: PixelInterface> SharedPixelCanvasExt<H, W, P> for T where
    T: PixelCanvasInterface<H, W, P>
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is implemented for any canvas of [`PixelInterface`].
pub trait SharedMutPixelCanvasExt<const H: usize, const W: usize, P: PixelMutInterface>:
    PixelCanvasInterface<H, W, P>
{
    /// Updates every pixel's color to default which is white.
    fn clear(&mut self)
    where
        <P as PixelInterface>::ColorType: Default + Clone,
    {
        self.fill(P::ColorType::default())
    }

    /// Fills all pixels color.
    fn fill(&mut self, color: impl Into<P::ColorType>)
    where
        <P as PixelInterface>::ColorType: Clone,
    {
        let color = color.into();
        self.table_mut().for_each_pixel_mut(|pixel| {
            pixel.update_color(color.clone());
        })
    }

    /// Update color of a pixel at the given position.
    fn update_color_at(
        &mut self,
        pos: impl PixelStrictPositionInterface<H, W>,
        color: impl Into<P::ColorType>,
    ) -> P::ColorType {
        self.table_mut()[pos].update_color(color)
    }

    fn draw<const HD: usize, const WD: usize>(
        &mut self,
        start_pos: impl IntoPixelStrictPosition<H, W>,
        drawable: impl Drawable<HD, WD>,
    ) where
        Self: Sized,
        <P as PixelInterface>::ColorType: From<PixelColor>,
    {
        drawable.draw_on(start_pos, self)
    }

    fn draw_exact(
        &mut self,
        start_pos: impl IntoPixelStrictPosition<H, W>,
        drawable: impl Drawable<H, W>,
    ) where
        Self: Sized,
        <P as PixelInterface>::ColorType: From<PixelColor>,
    {
        drawable.draw_on_exact(start_pos, self)
    }

    fn draw_exact_abs(&mut self, drawable: impl Drawable<H, W>)
    where
        Self: Sized,
        <P as PixelInterface>::ColorType: From<PixelColor>,
    {
        drawable.draw_on_exact_abs(self)
    }
}

impl<const H: usize, const W: usize, T, P: PixelMutInterface> SharedMutPixelCanvasExt<H, W, P> for T where
    T: PixelCanvasInterface<H, W, P>
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is only implemented for canvas of [`Pixel`] type.
pub trait PixelCanvasExt<const H: usize, const W: usize>:
    SharedPixelCanvasExt<H, W, Pixel>
{
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
    T: PixelCanvasInterface<H, W, Pixel>
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
pub trait PixelCanvasMutExt<const H: usize, const W: usize>:
    SharedMutPixelCanvasExt<H, W, Pixel>
{
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

impl<const H: usize, const W: usize, T> PixelCanvasMutExt<H, W> for T where
    T: PixelCanvasInterface<H, W, Pixel>
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
