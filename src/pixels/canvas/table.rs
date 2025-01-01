//! This module contains types related to a [`PixelTable`], subtype of
//! [PixelCanvas](`super::PixelCanvas`)
//!

use pixelart_table_abs::table::{IllusionArray2DHandle, IllusionArray2DHandleMut, IllusionTable};

use crate::pixels::{
    position::{IntoPixelStrictPosition, PixelStrictPositionInterface},
    Pixel, PixelInitializer, PixelInterface, PixelMutInterface,
};
/// Represents a table of [`Pixel`]s. (A collection of [`PixelRow`]s).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixelTable<const H: usize, const W: usize = H, P: PixelInterface + Default = Pixel> {
    pub(crate) inner: IllusionTable<H, W, P>,
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> PixelTable<H, W, P> {
    /// Returns actual existing elements, any other element will be the default value.
    pub fn real_items(&self) -> impl Iterator<Item = ((&usize, &usize), &P)> {
        self.inner.real_items()
    }

    /// Returns actual existing elements, any other element will be the default value.
    pub fn real_items_mut(&mut self) -> impl Iterator<Item = ((&usize, &usize), &mut P)> {
        self.inner.real_items_mut()
    }

    pub fn swap(
        &mut self,
        a: impl IntoPixelStrictPosition<H, W>,
        b: impl IntoPixelStrictPosition<H, W>,
    ) {
        self.inner.swap(
            a.into_pixel_strict_position().expand(),
            b.into_pixel_strict_position().expand(),
        )
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + PixelInitializer + Clone + Default>
    PixelTable<H, W, P>
{
    pub fn filled_len(&self) -> usize {
        self.inner.filled_len()
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + Default> PixelTable<H, W, P> {
    pub fn get_pixel(
        &self,
        pos: impl IntoPixelStrictPosition<H, W>,
    ) -> IllusionArray2DHandle<'_, H, W, P> {
        let (row, column) = pos.into_pixel_strict_position().expand();
        self.inner.get((row, column)).unwrap()
    }

    pub fn get_pixel_mut(
        &mut self,
        pos: impl IntoPixelStrictPosition<H, W>,
    ) -> pixelart_table_abs::table::IllusionArray2DHandleMut<H, W, P>
    where
        P: PartialEq + Clone,
    {
        let (row, column) = pos.into_pixel_strict_position().expand();
        self.inner.get_mut((row, column)).unwrap()
    }

    /// Returns an iterator over all [`Pixel`]s in this table row by row.
    ///
    /// ## Example
    /// ```rust
    /// # use pixelart::pixels::canvas::table::PixelTable;
    /// # use crate::pixelart::pixels::PixelInterface;
    /// # let mut table = PixelTable::<2>::default();
    /// for pix in table.iter_pixels() {
    ///     println!("{:?}", pix)
    /// }
    /// ```
    pub fn iter_pixels(&self) -> impl Iterator<Item = IllusionArray2DHandle<H, W, P>> {
        self.inner.iter()
    }

    /// Use this type to iterate over mutable ref of the pixels.
    ///
    /// ## Example
    /// ```rust
    /// # use pixelart::pixels::canvas::table::PixelTable;
    /// # use crate::pixelart::pixels::PixelInterface;
    /// # let mut table = PixelTable::<2>::default();
    /// for pix in table.iter_pixels_mut() {
    ///     // You can edit pixel here.
    ///     println!("{:?}", pix)
    /// }
    /// ```
    pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = IllusionArray2DHandleMut<H, W, P>>
    where
        P: PartialEq + Clone,
    {
        self.inner.iter_mut()
    }

    /// Calls a closure on each read-only ref pixel of this table.
    pub fn for_each_pixel<F>(&self, f: F)
    where
        F: Fn(IllusionArray2DHandle<H, W, P>) + Copy,
    {
        self.iter_pixels().for_each(f);
    }

    /// Calls a closure on each mutable ref pixel of this table.
    pub fn for_each_pixel_mut<F>(&mut self, f: F)
    where
        P: PixelMutInterface + Clone + PartialEq,
        F: FnMut(IllusionArray2DHandleMut<H, W, P>) + Copy,
    {
        self.iter_pixels_mut().for_each(f);
    }

    pub fn iter(
        &self,
    ) -> pixelart_table_abs::IllusionArrayIter<'_, H, pixelart_table_abs::IllusionArray<W, P>> {
        self.inner.inner().iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> pixelart_table_abs::IllusionArrayIterMut<'_, H, pixelart_table_abs::IllusionArray<W, P>>
    where
        P: PixelMutInterface + Clone + PartialEq,
    {
        self.inner.inner_mut().iter_mut()
    }

    pub fn get_row(
        &self,
        row: usize,
    ) -> Option<
        pixelart_table_abs::IllusionArrayHandle<'_, H, pixelart_table_abs::IllusionArray<W, P>>,
    > {
        self.inner.get_row(row)
    }

    pub fn get_row_mut(
        &mut self,
        row: usize,
    ) -> Option<
        pixelart_table_abs::IllusionArrayHandleMut<'_, H, pixelart_table_abs::IllusionArray<W, P>>,
    >
    where
        P: PixelMutInterface + Clone + PartialEq,
    {
        self.inner.get_row_mut(row)
    }
}

impl<const H: usize, const W: usize, P: Default> Default for PixelTable<H, W, P>
where
    P: PixelInterface + PixelInitializer + Clone + PartialEq,
    P::ColorType: Default + Clone,
{
    fn default() -> Self {
        Self {
            inner: IllusionTable::default(),
        }
    }
}

#[cfg(test)]
mod pixel_table_tests {
    use crate::{
        pixels::{
            canvas::SharedPixelCanvasExt,
            color::{PixelColor, PixelColorExt},
            position::PixelStrictPosition,
        },
        prelude::PixelCanvas,
    };

    use super::*;

    #[test]
    fn test_name() {
        let mut table = PixelTable::<5>::default();

        for row in table.iter() {
            for pixel in row.iter() {
                println!("{pixel:?}")
            }
        }

        let pos = PixelStrictPosition::new(0, 0).unwrap();

        let mut _pixel00 = table.get_pixel_mut(pos);
        _pixel00.update_color(PixelColor::BLUE);
    }

    #[test]
    fn iter_pixels() {
        let mut table = PixelTable::<2>::default();
        for pix in table.iter_pixels() {
            println!("{:?}", pix)
        }

        for pix in table.iter_pixels_mut() {
            println!("{:?}", pix)
        }
    }

    #[test]
    fn test_flip() {
        let mut canvas = PixelCanvas::<5>::default();

        canvas
            .get_row_mut(0)
            .unwrap()
            .iter_mut()
            .for_each(|mut pix| {
                pix.update_color(PixelColor::BLACK);
            });

        canvas.iter_mut().for_each(|mut row| {
            row.iter_mut()
                .last()
                .unwrap()
                .update_color(PixelColor::BLACK);
        });

        canvas
            .flip_x()
            .flip_y()
            .default_image_builder()
            .with_scale(5)
            .save("arts/flipped_0.png")
            .unwrap();
    }
}
