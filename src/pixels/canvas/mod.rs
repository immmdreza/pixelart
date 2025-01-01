//! Module contains types related to a [`PixelCanvas`].

use std::fmt::Debug;

use partition::CanvasPartition;

use crate::image::{PixelImageBuilder, PixelImageStyle};

use self::{drawable::Drawable, pen::Pen, table::PixelTable};

use super::{
    color::PixelColor,
    maybe::MaybePixel,
    position::{
        IntoPixelStrictPosition, PixelStrictPositionInterface, SingleCycle, MAIN_DIRECTIONS,
    },
    Pixel, PixelInitializer, PixelInterface, PixelMutInterface,
};

pub mod drawable;
pub mod layered;
pub mod partition;
pub mod pen;
pub mod table;
pub mod templates;

/// Interface that any read_only pixel canvas may want to implement.
///
/// Using this we can have access to later extension methods.
pub trait PixelCanvasInterface<const H: usize, const W: usize, P: PixelInterface + Default> {
    /// A read-only reference to underlying [`PixelTable`].
    fn table(&self) -> &PixelTable<H, W, P>;
}

/// Interface that any mutable pixel canvas may want to implement.
pub trait PixelCanvasMutInterface<const H: usize, const W: usize, P: PixelMutInterface + Default>:
    PixelCanvasInterface<H, W, P>
{
    /// A mutable reference to underlying [`PixelTable`].
    fn table_mut(&mut self) -> &mut PixelTable<H, W, P>;
}

/// A [`PixelCanvas`], the highest level api to work and clear interact
/// with the underlying [`PixelTable`] and pixels.
pub struct PixelCanvas<const H: usize, const W: usize = H, P: PixelInterface + Default = Pixel> {
    table: PixelTable<H, W, P>,
}

impl<const H: usize, const W: usize, P: PixelInterface + Default + Clone> Clone
    for PixelCanvas<H, W, P>
where
    <P as PixelInterface>::ColorType: Clone,
{
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default + std::fmt::Debug> std::fmt::Debug
    for PixelCanvas<H, W, P>
where
    <P as PixelInterface>::ColorType: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelCanvas")
            .field("table", &self.table)
            .finish()
    }
}

pub type MaybePixelCanvas<const H: usize, const W: usize = H> = PixelCanvas<H, W, MaybePixel>;

impl<const H: usize, const W: usize, P> PixelCanvas<H, W, P>
where
    P: PixelInterface + Default,
{
    pub fn from_fill_color(color: impl Into<P::ColorType> + Clone) -> Self
    where
        P: Clone,
        P: PartialEq,
        P: PixelInitializer + PixelMutInterface,
        P::ColorType: Default + Clone,
    {
        let mut canvas = Self::default();
        canvas.fill(color);
        canvas
    }

    pub fn flip_x(&mut self) -> &mut PixelCanvas<H, W, P> {
        for row in 0..H {
            for col in 0..W / 2 {
                let opposite_col = W - col - 1;
                self.table.swap((row, col), (row, opposite_col));
            }
        }

        self
    }

    pub fn flipped_x(&self) -> PixelCanvas<H, W, P>
    where
        P: Clone,
        P::ColorType: Clone,
    {
        let mut canvas = self.clone();
        canvas.flip_x();
        canvas
    }

    pub fn flip_y(&mut self) -> &mut PixelCanvas<H, W, P> {
        for row in 0..H / 2 {
            for col in 0..W {
                let opposite_row = H - row - 1;
                self.table.swap((row, col), (opposite_row, col));
            }
        }

        self
    }

    pub fn flipped_y(&self) -> PixelCanvas<H, W, P>
    where
        P: Clone,
        P::ColorType: Clone,
    {
        let mut canvas = self.clone();
        canvas.flip_y();
        canvas
    }
}

impl<const H: usize, const W: usize, P> Default for PixelCanvas<H, W, P>
where
    P: PixelInterface + PixelInitializer + PartialEq + Clone + Default,
    <P as PixelInterface>::ColorType: Default + Clone,
{
    fn default() -> Self {
        Self {
            table: Default::default(),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> std::ops::Deref
    for PixelCanvas<H, W, P>
{
    type Target = PixelTable<H, W, P>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> std::ops::DerefMut
    for PixelCanvas<H, W, P>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> PixelCanvasInterface<H, W, P>
    for PixelCanvas<H, W, P>
{
    fn table(&self) -> &PixelTable<H, W, P> {
        &self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> PixelCanvasInterface<H, W, P>
    for &PixelCanvas<H, W, P>
{
    fn table(&self) -> &PixelTable<H, W, P> {
        &self.table
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> PixelCanvasInterface<H, W, P>
    for &mut PixelCanvas<H, W, P>
{
    fn table(&self) -> &PixelTable<H, W, P> {
        &self.table
    }
}

impl<const H: usize, const W: usize, P: PixelMutInterface + Default>
    PixelCanvasMutInterface<H, W, P> for PixelCanvas<H, W, P>
{
    fn table_mut(&mut self) -> &mut PixelTable<H, W, P> {
        &mut self.table
    }
}

impl<const H: usize, const W: usize, P: PixelMutInterface + Default>
    PixelCanvasMutInterface<H, W, P> for &mut PixelCanvas<H, W, P>
{
    fn table_mut(&mut self) -> &mut PixelTable<H, W, P> {
        &mut self.table
    }
}

fn _fill_inside<
    const H: usize,
    const W: usize,
    P: PixelMutInterface + Default,
    I: SharedMutPixelCanvasExt<H, W, P>,
>(
    canvas: &mut I,
    base_color: Option<P::ColorType>,
    color: impl Into<P::ColorType> + Clone,
    point_inside: impl IntoPixelStrictPosition<H, W>,
) where
    P: PartialEq + Clone,
    P::ColorType: PartialEq + Clone + Default,
{
    let mut stack = vec![point_inside.into_pixel_strict_position()];
    let base_color = base_color.unwrap_or_else(|| canvas.color_at(stack[0]).clone());
    let color = color.into();

    while let Some(pos) = stack.pop() {
        if canvas.color_at(pos) == base_color {
            canvas.update_color_at(pos, color.clone());

            for dir in SingleCycle::new(super::position::Direction::Up)
                .filter(|dir| MAIN_DIRECTIONS.contains(dir))
            {
                if let Ok(new_pos) = pos.checked_direction(dir, 1) {
                    stack.push(new_pos);
                }
            }
        }
    }
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is implemented for any canvas of [`PixelInterface`].
pub trait SharedPixelCanvasExt<const H: usize, const W: usize, P: PixelInterface + Default>:
    PixelCanvasInterface<H, W, P>
{
    /// Get an [`PixelImageBuilder`] based on this canvas with [`PixelImageStyle`] specified.
    fn image_builder(&self, style: PixelImageStyle) -> PixelImageBuilder<H, W, P, Self>
    where
        Self: Sized,
    {
        PixelImageBuilder::new(self, style)
    }

    /// Get an [`PixelImageBuilder`] based on this canvas with default [`PixelImageStyle`].
    fn default_image_builder(&self) -> PixelImageBuilder<H, W, P, Self>
    where
        Self: Sized,
    {
        PixelImageBuilder::new_default_style(self)
    }

    /// Gets the color of a pixel at given position.
    fn color_at(&self, pos: impl PixelStrictPositionInterface<H, W>) -> P::ColorType
    where
        P::ColorType: Clone,
    {
        self.table().get_pixel(pos).color().clone()
    }

    fn any_partition<'a, const MH: usize, const MW: usize, MP>(
        &'a self,
        top_left: impl IntoPixelStrictPosition<H, W>,
    ) -> CanvasPartition<MH, MW, H, W, &'a Self, P, MP>
    where
        Self: Sized,
        &'a Self: PixelCanvasInterface<H, W, P>,
        MP: PixelMutInterface + PixelInitializer + PartialEq + Clone + Default,
        P::ColorType: Clone + Default,
        <MP as PixelInterface>::ColorType: Clone,
        <MP as PixelInterface>::ColorType: std::default::Default,
        <MP as PixelInterface>::ColorType: From<<P as PixelInterface>::ColorType>,
    {
        CanvasPartition::<MH, MW, H, W, &Self, P, MP>::new(top_left, self)
    }

    fn partition<'a, const MH: usize, const MW: usize>(
        &'a self,
        top_left: impl IntoPixelStrictPosition<H, W>,
    ) -> CanvasPartition<MH, MW, H, W, &'a Self, P, P>
    where
        Self: Sized,
        &'a Self: PixelCanvasInterface<H, W, P>,
        P: PixelMutInterface + PixelInitializer + PartialEq + Clone,
        P::ColorType: Clone + Default,
    {
        self.any_partition::<MH, MW, P>(top_left)
    }

    fn maybe_partition<'a, const MH: usize, const MW: usize>(
        &'a self,
        top_left: impl IntoPixelStrictPosition<H, W>,
    ) -> CanvasPartition<MH, MW, H, W, &'a Self, P, MaybePixel>
    where
        Self: Sized,
        &'a Self: PixelCanvasInterface<H, W, P>,
        P: PixelMutInterface + PixelInitializer,
        P::ColorType: Clone + Default,
        Option<PixelColor>: From<P::ColorType>,
    {
        self.any_partition::<MH, MW, MaybePixel>(top_left)
    }
}

impl<const H: usize, const W: usize, T, P: PixelInterface + Default> SharedPixelCanvasExt<H, W, P>
    for T
where
    T: PixelCanvasInterface<H, W, P>,
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is implemented for any canvas of [`PixelInterface`].
pub trait SharedMutPixelCanvasExt<const H: usize, const W: usize, P: PixelMutInterface + Default>:
    PixelCanvasMutInterface<H, W, P>
{
    fn attach_new_pen(
        &mut self,
        color: impl Into<P::ColorType>,
        start_pos: impl IntoPixelStrictPosition<H, W>,
    ) -> Pen<pen::CanvasAttachedMarker<H, W, P, Self>>
    where
        Self: Sized,
        <P as PixelInterface>::ColorType: From<PixelColor>,
    {
        let pen = Pen::new(color);
        pen.attach(self, start_pos)
    }

    /// Updates every pixel's color to default which is white.
    fn clear(&mut self)
    where
        P: PartialEq + Clone,
        <P as PixelInterface>::ColorType: Default + Clone,
    {
        self.fill(P::ColorType::default())
    }

    fn draw<const HD: usize, const WD: usize, MP: PixelInterface, E>(
        &mut self,
        start_pos: impl IntoPixelStrictPosition<H, W>,
        drawable: impl Drawable<HD, WD, MP>,
    ) where
        Self: Sized,
        P: PartialEq + Clone,
        <MP as PixelInterface>::ColorType: Clone,
        P::ColorType: TryFrom<MP::ColorType, Error = E>,
    {
        drawable.draw_on(start_pos, self)
    }

    fn draw_exact<MP: PixelInterface, E>(
        &mut self,
        start_pos: impl IntoPixelStrictPosition<H, W>,
        drawable: impl Drawable<H, W, MP>,
    ) where
        Self: Sized,
        P: PartialEq + Clone,
        <MP as PixelInterface>::ColorType: Clone,
        P::ColorType: TryFrom<MP::ColorType, Error = E>,
    {
        drawable.draw_on_exact(start_pos, self)
    }

    fn draw_exact_abs<MP: PixelInterface, E>(&mut self, drawable: impl Drawable<H, W, MP>)
    where
        Self: Sized,
        P: PartialEq + Clone,
        <MP as PixelInterface>::ColorType: Clone,
        P::ColorType: TryFrom<MP::ColorType, Error = E>,
    {
        drawable.draw_on_exact_abs(self)
    }

    /// Fills all pixels color.
    fn fill(&mut self, color: impl Into<P::ColorType>)
    where
        P: PartialEq + Clone,
        <P as PixelInterface>::ColorType: Clone,
    {
        let color = color.into();
        self.table_mut().for_each_pixel_mut(|mut pixel| {
            pixel.update_color(color.clone());
        })
    }

    /// Keep filling pixels with new color until we encounter a new color.
    fn fill_inside(
        &mut self,
        color: impl Into<P::ColorType> + std::clone::Clone,
        point_inside: impl IntoPixelStrictPosition<H, W>,
    ) where
        Self: Sized,
        P: PartialEq + Clone + Default,
        <P as PixelInterface>::ColorType: PartialEq + Clone + Default,
    {
        _fill_inside::<H, W, P, _>(self, None, color, point_inside)
    }

    /// Update color of a pixel at the given position.
    fn update_color_at(
        &mut self,
        pos: impl PixelStrictPositionInterface<H, W>,
        color: impl Into<P::ColorType>,
    ) -> P::ColorType
    where
        P: PartialEq + Clone,
    {
        self.table_mut().get_pixel_mut(pos).update_color(color)
    }

    fn any_partition_mut<'a, const MH: usize, const MW: usize, MP>(
        &'a mut self,
        top_left: impl IntoPixelStrictPosition<H, W>,
    ) -> CanvasPartition<MH, MW, H, W, &'a mut Self, P, MP>
    where
        Self: Sized,
        &'a mut Self: PixelCanvasInterface<H, W, P>,
        MP: PixelMutInterface + PixelInitializer + Clone + PartialEq + Default,
        P::ColorType: Clone + Default,
        <MP as PixelInterface>::ColorType: Clone,
        <MP as PixelInterface>::ColorType: std::default::Default,
        <MP as PixelInterface>::ColorType: From<<P as PixelInterface>::ColorType>,
    {
        CanvasPartition::<MH, MW, H, W, &mut Self, P, MP>::new(top_left, self)
    }

    fn partition_mut<'a, const MH: usize, const MW: usize>(
        &'a mut self,
        top_left: impl IntoPixelStrictPosition<H, W>,
    ) -> CanvasPartition<MH, MW, H, W, &'a mut Self, P, P>
    where
        Self: Sized,
        &'a mut Self: PixelCanvasInterface<H, W, P>,
        P: PixelMutInterface + PixelInitializer + PartialEq + Clone,
        P::ColorType: Clone + Default,
    {
        self.any_partition_mut::<MH, MW, P>(top_left)
    }

    fn maybe_partition_mut<'a, const MH: usize, const MW: usize>(
        &'a mut self,
        top_left: impl IntoPixelStrictPosition<H, W>,
    ) -> CanvasPartition<MH, MW, H, W, &'a mut Self, P, MaybePixel>
    where
        Self: Sized,
        &'a mut Self: PixelCanvasInterface<H, W, P>,
        P: PixelMutInterface + PixelInitializer,
        P::ColorType: Clone + Default,
        Option<PixelColor>: From<P::ColorType>,
    {
        self.any_partition_mut::<MH, MW, MaybePixel>(top_left)
    }
}

impl<const H: usize, const W: usize, T, P: PixelMutInterface + Default>
    SharedMutPixelCanvasExt<H, W, P> for T
where
    T: PixelCanvasMutInterface<H, W, P>,
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
///
/// This trait is only implemented for canvas of [`Pixel`] type.
pub trait PixelCanvasExt<const H: usize, const W: usize>:
    SharedPixelCanvasExt<H, W, Pixel>
{
}

impl<const H: usize, const W: usize, T> PixelCanvasExt<H, W> for T where
    T: PixelCanvasInterface<H, W, Pixel>
{
}

/// Extensions for any type that implements [`PixelCanvasInterface`].
pub trait PixelCanvasMutExt<const H: usize, const W: usize>:
    SharedMutPixelCanvasExt<H, W, Pixel>
{
    // /// Keep filling pixels with new color until we encounter a new color.
    // fn fill_inside(
    //     &mut self,
    //     color: impl IntoPixelColor,
    //     point_inside: impl IntoPixelStrictPosition<H, W>,
    // ) where
    //     Self: Sized,
    // {
    //     _fill_inside::<H, W, _>(self, None, color, point_inside)
    // }
}

impl<const H: usize, const W: usize, T> PixelCanvasMutExt<H, W> for T where
    T: PixelCanvasMutInterface<H, W, Pixel>
{
}

#[cfg(test)]
mod tests {

    use crate::{
        pixels::{color::PixelColorExt, position::StrictPositions, PixelIterMutExt},
        prelude::*,
    };

    use super::*;

    #[test]
    fn test_fill_inside() {
        let mut canvas = PixelCanvas::<5>::default();
        canvas
            .iter_pixels_mut()
            .filter_position(|(row, column)| row == column)
            .update_colors(PixelColor::RED);

        canvas.fill_inside(PixelColor::BLUE, StrictPositions::BottomLeft);

        canvas.update_color_at(StrictPositions::TopRight, PixelColor::BLACK);

        let image_builder = canvas.default_image_builder().with_scale(5);
        image_builder.save("arts/fill_inside.png").unwrap();
    }

    #[test]
    fn test_swap() {
        let mut canvas = PixelCanvas::<5>::default();

        canvas.get_pixel_mut(TOP_LEFT).update_color(BLUE);
        canvas.get_pixel_mut(BOTTOM_LEFT).update_color(RED);
        canvas.get_pixel_mut(BOTTOM_RIGHT).update_color(YELLOW);
        canvas.get_pixel_mut(TOP_RIGHT).update_color(GREEN);

        canvas.swap(TOP_LEFT, CENTER);
        canvas.swap(BOTTOM_LEFT, BOTTOM_RIGHT);
        canvas.swap(TOP_RIGHT, RIGHT_CENTER);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/swap_0.png")
            .unwrap();
    }
}
