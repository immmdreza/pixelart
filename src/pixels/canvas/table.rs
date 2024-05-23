//! This module contains types related to a [`PixelTable`], subtype of
//! [PixelCanvas](`super::PixelCanvas`)
//!

use std::array;

use crate::pixels::{
    position::{
        PixelPosition, PixelPositionInterface, PixelStrictPosition, PixelStrictPositionInterface,
    },
    Pixel,
};

use super::row::PixelRow;

/// Represents a table of [`Pixel`]s. (A collection of [`PixelRow`]s).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelTable<const H: usize, const W: usize = H> {
    pixels: [PixelRow<W>; H],
}

impl<const H: usize, const W: usize> PixelTable<H, W> {
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
    pub fn get_pixel(&self, pos: PixelPosition) -> Option<&Pixel> {
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
    pub fn get_pixel_mut(&mut self, pos: PixelPosition) -> Option<&mut Pixel> {
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
    pub fn iter_pixels(&self) -> PixelsIter<H, W> {
        PixelsIter::new(self)
    }

    /// Use this type to iterate over mutable ref of the pixels.
    ///
    /// Note that this is not an iterator and you should call `.advance()` to get next pixel.
    /// ## Example
    /// ```rust
    /// let mut table = PixelTable::<2>::default();
    /// let mut iterator = table.iter_pixels_mut();
    /// while let Some(pix) = iterator.advance() {
    ///     println!("{:?}", pix.position);
    /// }
    /// ```
    pub fn iter_pixels_mut(&mut self) -> PixelsIterMut<H, W> {
        PixelsIterMut::new(self)
    }
}

impl<const H: usize, const W: usize> IntoIterator for PixelTable<H, W> {
    type Item = PixelRow<W>;
    type IntoIter = std::array::IntoIter<Self::Item, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

impl<const H: usize, const W: usize> std::ops::DerefMut for PixelTable<H, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

impl<const H: usize, const W: usize> std::ops::Deref for PixelTable<H, W> {
    type Target = [PixelRow<W>; H];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<const H: usize, const W: usize> Default for PixelTable<H, W> {
    fn default() -> Self {
        Self {
            pixels: array::from_fn(|row| PixelRow {
                row,
                pixels: array::from_fn(|column| Pixel {
                    color: Default::default(),
                    position: PixelPosition::new(row, column),
                }),
            }),
        }
    }
}

impl<const H: usize, const W: usize> std::ops::IndexMut<usize> for PixelTable<H, W> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

impl<const H: usize, const W: usize> std::ops::Index<usize> for PixelTable<H, W> {
    type Output = PixelRow<W>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl<const H: usize, const W: usize, T: PixelStrictPositionInterface<H, W>> std::ops::Index<T>
    for PixelTable<H, W>
{
    type Output = Pixel;

    fn index(&self, index: T) -> &Self::Output {
        let (row, column) = index.expand();
        &self[row][column]
    }
}

impl<const H: usize, const W: usize, T: PixelStrictPositionInterface<H, W>> std::ops::IndexMut<T>
    for PixelTable<H, W>
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let (row, column) = index.expand();
        &mut self[row][column]
    }
}

pub struct PixelsIter<'t, const H: usize, const W: usize> {
    table: &'t PixelTable<H, W>,
    consumed: bool,
    current_pos: PixelStrictPosition<H, W>,
}

impl<'t, const H: usize, const W: usize> PixelsIter<'t, H, W> {
    pub fn new(table: &'t PixelTable<H, W>) -> Self {
        Self {
            table,
            consumed: H == 0 || W == 0,
            current_pos: PixelStrictPosition::new(0, 0).unwrap(),
        }
    }
}

impl<'t, const H: usize, const W: usize> Iterator for PixelsIter<'t, H, W> {
    type Item = &'t Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }

        let next_pixel = self
            .table
            .get(self.current_pos.row())?
            .get(self.current_pos.column());
        let next_pos = self.current_pos.next();
        self.consumed = next_pos.is_none();
        next_pixel
    }
}

pub struct PixelsIterMut<'t, const H: usize, const W: usize> {
    table: &'t mut PixelTable<H, W>,
    consumed: bool,
    current_pos: PixelStrictPosition<H, W>,
}

impl<'t, const H: usize, const W: usize> PixelsIterMut<'t, H, W> {
    pub fn new(table: &'t mut PixelTable<H, W>) -> Self {
        Self {
            table,
            consumed: H == 0 || W == 0,
            current_pos: PixelStrictPosition::new(0, 0).unwrap(),
        }
    }

    pub fn advance(&mut self) -> Option<&mut Pixel> {
        if self.consumed {
            return None;
        }

        let next_pixel = self
            .table
            .get_mut(self.current_pos.row())?
            .get_mut(self.current_pos.column());
        let next_pos = self.current_pos.next();
        self.consumed = next_pos.is_none();
        next_pixel
    }
}

#[cfg(test)]
mod pixel_table_tests {
    use crate::pixels::{
        color::{PixelColor, PixelColorExt},
        position::{PixelStrictPosition, PixelStrictPositionInterface},
    };

    use super::*;

    fn _assert_iterator_type<'a, const W: usize, I: Iterator<Item = &'a PixelRow<W>>>(
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

        let mut iterator = table.iter_pixels_mut();
        while let Some(pixel) = iterator.advance() {
            println!("{:?}", pixel.position);
        }
    }
}
