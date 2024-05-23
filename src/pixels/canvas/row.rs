//! This module contains types related to a [`PixelRow`], subtype of
//! [PixelTable](`super::table::PixelTable`)
//!

use std::array::{self};

use crate::pixels::Pixel;
/// Represents a row of [`Pixel`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelRow<const W: usize> {
    /// Row number staring from 0. (row index)
    pub(super) row: usize,
    pub(super) pixels: [Pixel; W],
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
