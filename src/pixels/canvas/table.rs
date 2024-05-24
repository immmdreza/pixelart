//! This module contains types related to a [`PixelTable`], subtype of
//! [PixelCanvas](`super::PixelCanvas`)
//!

use std::{array, fmt::Display};

use crate::pixels::{
    color::{IntoPixelColor, PixelColor},
    position::{PixelPosition, PixelPositionInterface, PixelStrictPositionInterface},
    Pixel, PixelInterface,
};

use super::row::PixelRow;

/// Represents a table of [`Pixel`]s. (A collection of [`PixelRow`]s).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelTable<const H: usize, const W: usize = H, P: PixelInterface = Pixel> {
    pixels: [PixelRow<W, P>; H],
}

impl<const H: usize, const W: usize, P: PixelInterface + Display> Display for PixelTable<H, W, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pix in self.iter() {
            write!(f, "{}\n", pix)?;
        }
        Ok(())
    }
}

impl<const H: usize, const W: usize> PixelTable<H, W, Pixel> {
    pub fn new(fill_color: impl IntoPixelColor + Clone) -> Self {
        Self {
            pixels: array::from_fn(|row| PixelRow::new(row, fill_color.clone())),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> PixelTable<H, W, P> {
    /// Get a pixel ref at a [`PixelPosition`] which can be out ob bound! In that case [`None`] is returned.
    ///
    /// Use indexing syntax and a [PixelStrictPosition](`crate::pixels::position::PixelStrictPosition`)
    /// to ensure the position is inbound.
    ///
    /// ## Example of indexing
    /// ```rust
    /// let table = PixelTable::<5>::default();
    /// // Here the code will panic if the position is unbound,
    /// // You can use .unwrap_or_else(|e| e.adjust()) instead to make it fit in that case.
    /// let pos = PixelStrictPosition::new(0, 0).unwrap();
    /// let pixel = &table[pos]
    /// ```
    pub fn get_pixel(&self, pos: PixelPosition) -> Option<&P> {
        let (row, column) = pos.expand();
        self.get(row)?.get(column)
    }

    /// Get a pixel mutable ref at a [`PixelPosition`] which can be out ob bound! In that case [`None`] is returned.
    ///
    /// Use indexing syntax and a [PixelStrictPosition](`crate::pixels::position::PixelStrictPosition`)
    /// to ensure the position is inbound.
    ///
    /// ## Example of indexing
    /// ```rust
    /// let table = PixelTable::<5>::default();
    /// // Here the code will panic if the position is unbound,
    /// // You can use .unwrap_or_else(|e| e.adjust()) instead to make it fit in that case.
    /// let pos = PixelStrictPosition::new(0, 0).unwrap();
    /// let pixel = &mut table[pos]
    /// ```
    pub fn get_pixel_mut(&mut self, pos: PixelPosition) -> Option<&mut P> {
        let (row, column) = pos.expand();
        self.get_mut(row)?.get_mut(column)
    }

    /// Returns an iterator over all [`Pixel`]s in this table row by row.
    ///
    /// ## Example
    /// ```rust
    /// let mut table = PixelTable::<2>::default();
    /// for pix in table.iter_pixels() {
    ///     println!("{:?}", pix.position)
    /// }
    /// ```
    pub fn iter_pixels(&self) -> impl Iterator<Item = &P> {
        self.iter().map(|f| f.iter()).flatten()
    }

    /// Use this type to iterate over mutable ref of the pixels.
    ///
    /// ## Example
    /// ```rust
    /// let mut table = PixelTable::<2>::default();
    /// for pix in table.iter_pixels_mut() {
    ///     // You can edit pixel here.
    ///     println!("{:?}", pix.position)
    /// }
    /// ```
    pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = &mut P> {
        self.iter_mut().map(|f| f.iter_mut()).flatten()
    }

    /// Calls a closure on each read-only ref pixel of this table.
    pub fn for_each_pixel<F>(&self, f: F)
    where
        F: Fn(&P) + Copy,
    {
        self.iter().for_each(|row| row.iter().for_each(f))
    }

    /// Calls a closure on each mutable ref pixel of this table.
    pub fn for_each_pixel_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut P) + Copy,
    {
        self.iter_mut().for_each(|row| row.iter_mut().for_each(f))
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> IntoIterator for PixelTable<H, W, P> {
    type Item = PixelRow<W, P>;
    type IntoIter = std::array::IntoIter<Self::Item, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> std::ops::DerefMut for PixelTable<H, W, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> std::ops::Deref for PixelTable<H, W, P> {
    type Target = [PixelRow<W, P>; H];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<const H: usize, const W: usize> Default for PixelTable<H, W, Pixel> {
    fn default() -> Self {
        Self::new(PixelColor::default())
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> std::ops::IndexMut<usize>
    for PixelTable<H, W, P>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> std::ops::Index<usize>
    for PixelTable<H, W, P>
{
    type Output = PixelRow<W, P>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl<const H: usize, const W: usize, T: PixelStrictPositionInterface<H, W>, P: PixelInterface>
    std::ops::Index<T> for PixelTable<H, W, P>
{
    type Output = P;

    fn index(&self, index: T) -> &Self::Output {
        let (row, column) = index.expand();
        &self[row][column]
    }
}

impl<const H: usize, const W: usize, T: PixelStrictPositionInterface<H, W>, P: PixelInterface>
    std::ops::IndexMut<T> for PixelTable<H, W, P>
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let (row, column) = index.expand();
        &mut self[row][column]
    }
}

#[cfg(test)]
mod pixel_table_tests {
    use crate::pixels::{
        color::{PixelColor, PixelColorExt},
        position::{PixelStrictPosition, PixelStrictPositionInterface},
        PixelMutInterface,
    };

    use super::*;

    fn _assert_iterator_type<
        'a,
        const W: usize,
        I: Iterator<Item = &'a PixelRow<W, P>>,
        P: PixelInterface + 'static,
    >(
        _row_iter: I,
    ) {
    }

    #[test]
    fn test_name() {
        let mut table = PixelTable::<5>::default();

        let iter = table.iter();
        _assert_iterator_type(iter);

        for row in table.iter() {
            for pixel in row.iter() {
                println!("{pixel:?}")
            }
        }

        let pos = PixelStrictPosition::new(0, 0).unwrap();

        let _pixel00 = &mut table[&pos];
        _pixel00.update_color(PixelColor::BLUE);
        let _pixel00_maybe_invalid = table.get_pixel(pos.unbound());
    }

    #[test]
    fn iter_pixels() {
        let mut table = PixelTable::<2>::default();
        for pix in table.iter_pixels() {
            println!("{:?}", pix.position)
        }

        for pix in table.iter_pixels_mut() {
            println!("{:?}", pix)
        }
    }
}
