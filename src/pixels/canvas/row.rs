//! This module contains types related to a [`PixelRow`], subtype of
//! [PixelTable](`super::table::PixelTable`)
//!

use std::array::{self};

use crate::pixels::{color::PixelColor, position::PixelPosition, Pixel};
/// Represents a row of [`Pixel`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelRow<const W: usize> {
    /// Row number staring from 0. (row index)
    pub(super) row: usize,
    pub(super) pixels: [Pixel; W],
}

impl<const W: usize> PixelRow<W> {
    pub fn new(row: usize) -> Self {
        Self {
            row,
            pixels: array::from_fn(|column| {
                Pixel::new(PixelColor::default(), PixelPosition::new(row, column))
            }),
        }
    }
}

impl<const W: usize> std::ops::DerefMut for PixelRow<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

impl<const W: usize> std::ops::Deref for PixelRow<W> {
    type Target = [Pixel; W];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<const W: usize> IntoIterator for PixelRow<W> {
    type Item = Pixel;
    type IntoIter = array::IntoIter<Self::Item, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

impl<const W: usize> std::ops::IndexMut<usize> for PixelRow<W> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

impl<const W: usize> std::ops::Index<usize> for PixelRow<W> {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

/// A set of extension methods for a read-only iterator over [`PixelRow`].
pub trait PixelRowIterExt<'p, const W: usize>: Iterator<Item = &'p PixelRow<W>> {
    /// Filter rows based on index of the row.
    fn filter_row<P>(self, mut predicate: P) -> impl Iterator<Item = &'p PixelRow<W>>
    where
        Self: Sized,
        P: FnMut(usize) -> bool,
    {
        self.filter(move |row| predicate(row.row))
    }
}

impl<'p, const W: usize, T> PixelRowIterExt<'p, W> for T where T: Iterator<Item = &'p PixelRow<W>> {}

/// A set of extension methods for a mutable iterator over [`PixelRow`].
pub trait PixelRowIterMutExt<'p, const W: usize>: Iterator<Item = &'p mut PixelRow<W>> {
    /// Filter rows based on index of the row.
    fn filter_row<P>(self, mut predicate: P) -> impl Iterator<Item = &'p mut PixelRow<W>>
    where
        Self: Sized,
        P: FnMut(usize) -> bool,
    {
        self.filter(move |row| predicate(row.row))
    }
}

impl<'p, const W: usize, T> PixelRowIterMutExt<'p, W> for T where
    T: Iterator<Item = &'p mut PixelRow<W>>
{
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name() {
        let mut r = PixelRow::<2>::new(0);
        let _s = r.as_mut_slice();
    }
}
