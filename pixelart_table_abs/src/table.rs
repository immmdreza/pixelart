use std::ops::{Deref, DerefMut};

use crate::{IllusionArray, IllusionItem};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InnerIllusionTable<const H: usize, const W: usize, P>
where
    P: Default,
{
    inner: IllusionArray<H, IllusionArray<W, P>>,
    default: P,
}

impl<const H: usize, const W: usize, P> InnerIllusionTable<H, W, P>
where
    P: Default,
{
    pub fn filled_len(&self) -> usize {
        self.inner.iter().map(|row| row.filled_len()).sum()
    }

    fn get(&self, (row, column): (usize, usize)) -> Option<&P> {
        // Return none if the index is out of bounds
        if column >= W || row >= H {
            return None;
        }

        if let Some(row) = self.inner.inner.inner.get(&row) {
            if let Some(item) = row.value.inner.get(column) {
                return Some(item);
            }
        }

        return Some(&self.default);
    }

    fn find_raw_mut(&mut self, (row, column): (usize, usize)) -> Option<&mut P> {
        if let Some(row) = self.inner.inner.inner.get_mut(&row) {
            if let Some(item) = row.value.inner.inner.get_mut(&column) {
                return Some(&mut item.value);
            }
        }

        return None;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IllusionTable<const H: usize, const W: usize, P>
where
    P: Default,
{
    inner: InnerIllusionTable<H, W, P>,
}

impl<const H: usize, const W: usize, P> IllusionTable<H, W, P>
where
    P: Default,
{
    pub fn filled_len(&self) -> usize {
        self.inner.filled_len()
    }

    pub fn iter(&self) -> IllusionTableIter<H, W, P> {
        IllusionTableIter {
            inner: &self.inner,
            curr_index: (0, 0),
        }
    }

    pub fn iter_mut(&mut self) -> IllusionTableIterMut<H, W, P> {
        IllusionTableIterMut {
            inner: &mut self.inner,
            curr_index: (0, 0),
        }
    }

    pub fn get(&self, (row, column): (usize, usize)) -> Option<IllusionArray2DHandle<'_, H, W, P>> {
        // Return none if the index is out of bounds
        if column >= W || row >= H {
            return None;
        }

        Some(IllusionArray2DHandle {
            inner: &self.inner,
            index: (row, column),
        })
    }

    pub fn get_mut(
        &mut self,
        (row, column): (usize, usize),
    ) -> Option<IllusionArray2DHandleMut<'_, H, W, P>>
    where
        P: Clone,
        P: PartialEq,
    {
        // Return none if the index is out of bounds
        if column >= W || row >= H {
            return None;
        }

        let current = Some(Default::default());
        Some(IllusionArray2DHandleMut {
            inner: &mut self.inner,
            index: (row, column),
            current,
        })
    }

    pub fn try_modify(
        &mut self,
        (row, column): (usize, usize),
        f: impl FnOnce(&mut IllusionArray2DHandleMut<H, W, P>),
    ) -> bool
    where
        P: std::cmp::PartialEq + std::clone::Clone,
    {
        if let Some(mut item) = self.get_mut((row, column)) {
            item.modify(f);
            true
        } else {
            false
        }
    }

    pub fn get_row(
        &self,
        row: usize,
    ) -> Option<crate::IllusionArrayHandle<'_, H, IllusionArray<W, P>>> {
        self.inner.inner.get(row)
    }

    pub fn get_row_mut(
        &mut self,
        row: usize,
    ) -> Option<crate::IllusionArrayHandleMut<'_, H, IllusionArray<W, P>>>
    where
        P: std::cmp::PartialEq + std::clone::Clone,
    {
        self.inner.inner.get_mut(row)
    }

    pub fn inner(&self) -> &IllusionArray<H, IllusionArray<W, P>> {
        &self.inner.inner
    }

    pub fn inner_mut(&mut self) -> &mut IllusionArray<H, IllusionArray<W, P>> {
        &mut self.inner.inner
    }

    pub fn real_items(&self) -> impl Iterator<Item = ((&usize, &usize), &P)> {
        self.inner
            .inner
            .real_items()
            .map(|(row, items)| {
                items
                    .real_items()
                    .map(move |(column, item)| ((row, column), item))
            })
            .flatten()
    }

    pub fn real_items_mut(&mut self) -> impl Iterator<Item = ((&usize, &usize), &mut P)> {
        self.inner
            .inner
            .real_items_mut()
            .map(|(row, items)| {
                items
                    .real_items_mut()
                    .map(move |(column, item)| ((row, column), item))
            })
            .flatten()
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let a_exists = self.inner.find_raw_mut(a).is_some();
        let b_exists = self.inner.find_raw_mut(b).is_some();

        let mut replace = |a: (usize, usize), b: (usize, usize)| {
            let row = self.inner.inner.inner.inner.get_mut(&a.0).unwrap();
            let item = row.value.inner.inner.remove(&a.1).unwrap();
            if row.value.inner.inner.is_empty() {
                self.inner.inner.inner.inner.remove(&a.0);
            }

            let entry = self.inner.inner.inner.inner.entry(b.0);
            entry.or_default().value.inner.inner.insert(b.1, item);
        };

        if a_exists && b_exists {
            let a = self.inner.find_raw_mut(a).unwrap() as *mut P;
            let b = self.inner.find_raw_mut(b).unwrap() as *mut P;

            unsafe {
                std::ptr::swap(a, b);
            }
        } else if a_exists {
            // Take out a and replace to b
            replace(a, b);
        } else {
            // Take out b and replace to a
            replace(b, a);
        }
    }
}

impl<const H: usize, const W: usize, P: Default> Default for IllusionTable<H, W, P> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

pub struct IllusionTableIter<'a, const H: usize, const W: usize, P>
where
    P: Default,
{
    inner: &'a InnerIllusionTable<H, W, P>,
    curr_index: (usize, usize), // Represents the current index of the 2d array (row, col)
}

impl<'a, const H: usize, const W: usize, P> Iterator for IllusionTableIter<'a, H, W, P>
where
    P: Default,
{
    // Array handle for a 2d array
    type Item = IllusionArray2DHandle<'a, H, W, P>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start from first row and first column (0, 0) to last row and last column (H, W)
        if self.curr_index.0 < H {
            let handle = IllusionArray2DHandle {
                inner: self.inner,
                index: self.curr_index,
            };

            // Increment the column index
            if self.curr_index.1 < W - 1 {
                self.curr_index.1 += 1;
            } else {
                // Increment the row index and reset the column index
                self.curr_index.0 += 1;
                self.curr_index.1 = 0;
            }

            Some(handle)
        } else {
            None
        }
    }
}

pub struct IllusionArray2DHandle<'a, const H: usize, const W: usize, P>
where
    P: Default,
{
    inner: &'a InnerIllusionTable<H, W, P>,
    index: (usize, usize),
}

impl<'a, const H: usize, const W: usize, P> std::fmt::Debug for IllusionArray2DHandle<'a, H, W, P>
where
    for<'b> &'b P: std::fmt::Debug,
    P: Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IllusionArray2DHandle")
            .field("index", &self.index)
            .field("value", &self.get())
            .finish()
    }
}

impl<'a, const H: usize, const W: usize, P> Deref for IllusionArray2DHandle<'a, H, W, P>
where
    P: Default,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, const H: usize, const W: usize, P> IllusionArray2DHandle<'a, H, W, P>
where
    P: Default,
{
    pub fn get(&self) -> &P {
        self.inner
            .get(self.index)
            .expect("IllusionArray2DHandle's Row index should be in bounds.")
    }

    pub fn mapped<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&P) -> T,
    {
        f(self.get())
    }

    pub fn index(&self) -> (usize, usize) {
        self.index
    }
}

pub struct IllusionTableIterMut<'a, const H: usize, const W: usize, P>
where
    P: Default,
{
    inner: &'a mut InnerIllusionTable<H, W, P>,
    curr_index: (usize, usize), // Represents the current index of the 2d array (row, col)
}

impl<'a, const H: usize, const W: usize, P> Iterator for IllusionTableIterMut<'a, H, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    // Array handle for a 2d array
    type Item = IllusionArray2DHandleMut<'a, H, W, P>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start from first row and first column (0, 0) to last row and last column (H, W)
        if self.curr_index.0 < H {
            let handle = IllusionArray2DHandleMut {
                inner: unsafe { &mut *(self.inner as *mut _) },
                index: self.curr_index,
                current: Some(P::default()),
            };

            // Increment the column index
            if self.curr_index.1 < W - 1 {
                self.curr_index.1 += 1;
            } else {
                // Increment the row index and reset the column index
                self.curr_index.0 += 1;
                self.curr_index.1 = 0;
            }

            Some(handle)
        } else {
            None
        }
    }
}

pub struct IllusionArray2DHandleMut<'a, const H: usize, const W: usize, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    inner: &'a mut InnerIllusionTable<H, W, P>,
    index: (usize, usize),
    current: Option<P>,
}

impl<'a, const H: usize, const W: usize, P> std::fmt::Debug
    for IllusionArray2DHandleMut<'a, H, W, P>
where
    for<'b> &'b P: std::fmt::Debug,
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IllusionArray2DHandleMut")
            .field("index", &self.index)
            .field("value", &self.get())
            .finish()
    }
}

impl<'a, const H: usize, const W: usize, P> DerefMut for IllusionArray2DHandleMut<'a, H, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<'a, const H: usize, const W: usize, P> Deref for IllusionArray2DHandleMut<'a, H, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, const H: usize, const W: usize, P> Drop for IllusionArray2DHandleMut<'a, H, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    fn drop(&mut self) {
        let current = self.current.take().unwrap_or(P::default());

        if &current != &self.inner.default {
            if let Some(row) = self.inner.inner.inner.inner.get_mut(&self.index.0) {
                if let Some(item) = row.value.inner.inner.get_mut(&self.index.1) {
                    item.value = current;
                } else {
                    // Row exists but item doesn't
                    // Add a new item
                    row.value
                        .inner
                        .inner
                        .insert(self.index.1, IllusionItem { value: current });
                }
            } else {
                // Row doesn't exist
                // Add a new row and item
                let mut row = IllusionArray::<W, P>::default();
                row.inner
                    .inner
                    .insert(self.index.1, IllusionItem { value: current });
                self.inner
                    .inner
                    .inner
                    .inner
                    .insert(self.index.0, IllusionItem { value: row });
            }
        } else {
            // remove old since it's now default.
            if let Some(row) = self.inner.inner.inner.inner.get_mut(&self.index.0) {
                if row.value.inner.inner.remove(&self.index.1).is_some() {
                    if row.value.inner.inner.is_empty() {
                        // If an entity is remove and there're no others, clean up.
                        self.inner.inner.inner.inner.remove(&self.index.0);
                    }
                }
            }
        }
    }
}

impl<'a, const H: usize, const W: usize, P> IllusionArray2DHandleMut<'a, H, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    pub fn get(&self) -> &P {
        self.inner
            .get(self.index)
            .expect("IllusionArray2DHandle's Row index should be in bounds.")
    }

    pub fn mapped<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&P) -> T,
    {
        f(self.get())
    }

    pub fn get_mut(&mut self) -> &mut P
    where
        P: PartialEq + Clone,
    {
        // What are these inners anyway?
        if let Some(row) = self.inner.inner.inner.inner.get_mut(&self.index.0) {
            let item = row
                .value
                .inner
                .inner
                .get_mut(&self.index.1)
                .map(|item| &mut item.value)
                .unwrap_or(self.current.as_mut().unwrap());
            item
        } else {
            self.current.as_mut().unwrap()
        }
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut Self)) {
        f(self);
    }

    pub fn index(&self) -> (usize, usize) {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut table = IllusionTable::<2, 2, i32>::default();

        println!("Filled len: {:?}", table.filled_len());
        println!("---");

        for mut item in table.iter_mut() {
            let (row, column) = item.index();
            *item += (row * column) as i32;
        }

        for item in table.iter() {
            println!("Row: {:?}", item);
        }

        println!("Filled len: {:?}", table.filled_len());
        println!("---");

        table.try_modify((0, 0), |v| *v.get_mut() = 10);
        table.try_modify((1, 1), |v| *v.get_mut() = 15);

        println!("Value at (0, 0): {:?}", table.get((0, 0)));
        println!("Value at (1, 1): {:?}", table.get((1, 1)));
        println!("Filled len: {:?}", table.filled_len());
        println!("---");
    }
}
