//! This module contains types related to a [`PixelRow`], subtype of
//! [PixelTable](`super::table::PixelTable`)
//!

use std::{array, fmt::Display};

use crate::pixels::{position::PixelPosition, PixelInitializer, PixelInterface};
/// Represents a row of [`Pixel`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelRow<const W: usize, P: PixelInterface> {
    /// Row number staring from 0. (row index)
    pub(super) row: usize,
    pub(super) pixels: [P; W],
}

impl<const W: usize, P: PixelInterface + Display> Display for PixelRow<W, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for pix in self.iter() {
            write!(f, "{}, ", pix)?;
        }
        f.write_str("]")
    }
}

impl<const W: usize, P: PixelInitializer + PixelInterface> PixelRow<W, P> {
    pub fn new(row: usize, fill_color: impl Into<P::ColorType> + Clone) -> Self {
        Self {
            row,
            pixels: array::from_fn(move |column| {
                P::new(fill_color.clone(), PixelPosition::new(row, column))
            }),
        }
    }
}

impl<const W: usize, P: PixelInterface> std::ops::DerefMut for PixelRow<W, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

impl<const W: usize, P: PixelInterface> std::ops::Deref for PixelRow<W, P> {
    type Target = [P; W];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<const W: usize, P: PixelInterface> IntoIterator for PixelRow<W, P> {
    type Item = P;
    type IntoIter = array::IntoIter<Self::Item, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

impl<const W: usize, P: PixelInterface> std::ops::IndexMut<usize> for PixelRow<W, P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

impl<const W: usize, P: PixelInterface> std::ops::Index<usize> for PixelRow<W, P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

/// A set of extension methods for a read-only iterator over [`PixelRow`].
pub trait PixelRowIterExt<'p, const W: usize, P: PixelInterface + 'static>:
    Iterator<Item = &'p PixelRow<W, P>>
{
    /// Filter rows based on index of the row.
    fn filter_row<F>(self, mut predicate: F) -> impl Iterator<Item = &'p PixelRow<W, P>>
    where
        Self: Sized,
        F: FnMut(usize) -> bool,
    {
        self.filter(move |row| predicate(row.row))
    }
}

impl<'p, const W: usize, T, P: PixelInterface + 'static> PixelRowIterExt<'p, W, P> for T where
    T: Iterator<Item = &'p PixelRow<W, P>>
{
}

/// A set of extension methods for a mutable iterator over [`PixelRow`].
pub trait PixelRowIterMutExt<'p, const W: usize, P: PixelInterface + 'static>:
    Iterator<Item = &'p mut PixelRow<W, P>>
{
    /// Filter rows based on index of the row.
    fn filter_row<R>(self, mut predicate: R) -> impl Iterator<Item = &'p mut PixelRow<W, P>>
    where
        Self: Sized,
        R: FnMut(usize) -> bool,
    {
        self.filter(move |row| predicate(row.row))
    }
}

impl<'p, const W: usize, T, P: PixelInterface + 'static> PixelRowIterMutExt<'p, W, P> for T where
    T: Iterator<Item = &'p mut PixelRow<W, P>>
{
}

#[cfg(test)]
mod tests {

    use crate::pixels::{color::PixelColor, Pixel};

    use super::*;

    #[test]
    fn test_name() {
        let mut r = PixelRow::<2, Pixel>::new(0, PixelColor::default());
        let _s = r.as_mut_slice();
    }
}
