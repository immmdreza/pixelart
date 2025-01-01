use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub mod table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct IllusionItem<T> {
    value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InnerIllusionArray<const W: usize, P>
where
    P: Default,
{
    inner: HashMap<usize, IllusionItem<P>>,
    default: P,
}

impl<const W: usize, P> InnerIllusionArray<W, P>
where
    P: Default,
{
    pub fn filled_len(&self) -> usize {
        self.inner.len()
    }

    /// Returns actual element if exists, or the default value otherwise. And none if the index is out of bounds.
    fn get(&self, index: usize) -> Option<&P> {
        // Return none if the index is out of bounds
        if index >= W {
            return None;
        }

        self.inner
            .get(&index)
            .map(|item| &item.value)
            .unwrap_or(&self.default)
            .into()
    }
}

/// This kind of array assumes (illusional) that is filled with `W` elements.
/// When iterates, it gives a mut or ref to a handles that returns a ref to actual element if exists, or the
/// default value otherwise.
/// Mutating the handle will mutate the actual element if exists or creating it if not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IllusionArray<const W: usize, P>
where
    P: Default,
{
    inner: InnerIllusionArray<W, P>,
}

impl<const W: usize, P: Default> Default for IllusionArray<W, P> {
    fn default() -> Self {
        Self {
            inner: InnerIllusionArray {
                inner: HashMap::new(),
                default: Default::default(),
            },
        }
    }
}

impl<const W: usize, P> IllusionArray<W, P>
where
    P: Default,
{
    pub fn iter(&self) -> IllusionArrayIter<W, P> {
        IllusionArrayIter {
            inner: &self.inner,
            curr_index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IllusionArrayIterMut<W, P> {
        IllusionArrayIterMut {
            inner: &mut self.inner,
            curr_index: 0,
        }
    }

    pub fn filled_len(&self) -> usize {
        self.inner.filled_len()
    }

    /// Returns actual element if exists, or the default value otherwise. And none if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<IllusionArrayHandle<'_, W, P>> {
        if index >= W {
            return None;
        }

        Some(IllusionArrayHandle {
            inner: &self.inner,
            index,
        })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<IllusionArrayHandleMut<'_, W, P>>
    where
        P: Clone,
        P: PartialEq,
    {
        // Return none if the index is out of bounds
        if index >= W {
            return None;
        }

        let current = P::default().into();
        Some(IllusionArrayHandleMut {
            inner: &mut self.inner,
            index,
            current,
        })
    }

    pub fn try_modify(
        &mut self,
        index: usize,
        f: impl FnOnce(&mut IllusionArrayHandleMut<W, P>),
    ) -> bool
    where
        P: std::cmp::PartialEq + std::clone::Clone,
    {
        if let Some(mut item) = self.get_mut(index) {
            item.modify(f);
            true
        } else {
            false
        }
    }

    pub fn real_items(&self) -> impl Iterator<Item = (&usize, &P)> {
        self.inner
            .inner
            .iter()
            .map(|(index, item)| (index, &item.value))
    }

    pub fn real_items_mut(&mut self) -> impl Iterator<Item = (&usize, &mut P)> {
        self.inner
            .inner
            .iter_mut()
            .map(|(index, item)| (index, &mut item.value))
    }
}

pub struct IllusionArrayIter<'a, const W: usize, P>
where
    P: Default,
{
    inner: &'a InnerIllusionArray<W, P>,
    curr_index: usize,
}

impl<'a, const W: usize, P> Iterator for IllusionArrayIter<'a, W, P>
where
    P: Default,
{
    type Item = IllusionArrayHandle<'a, W, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index < W {
            let handle = IllusionArrayHandle {
                inner: self.inner,
                index: self.curr_index,
            };
            self.curr_index += 1;
            Some(handle)
        } else {
            None
        }
    }
}

pub struct IllusionArrayHandle<'a, const W: usize, P>
where
    P: Default,
{
    inner: &'a InnerIllusionArray<W, P>,
    index: usize,
}

impl<'a, const W: usize, P> Debug for IllusionArrayHandle<'a, W, P>
where
    for<'b> &'b P: Debug,
    P: Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IllusionArrayHandle")
            .field("index", &self.index)
            .field("value", &self.get())
            .finish()
    }
}

impl<'a, const W: usize, P> IllusionArrayHandle<'a, W, P>
where
    P: Default,
{
    pub fn get(&self) -> &P {
        self.inner
            .get(self.index)
            .expect("IllusionArrayHandle should be in bounds.")
    }

    pub fn mapped<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&P) -> T,
    {
        f(self.get())
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<'a, const W: usize, P> Deref for IllusionArrayHandle<'a, W, P>
where
    P: Default,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

pub struct IllusionArrayIterMut<'a, const W: usize, P>
where
    P: Default,
{
    inner: &'a mut InnerIllusionArray<W, P>,
    curr_index: usize,
}

impl<'a, const W: usize, P> Debug for IllusionArrayHandleMut<'a, W, P>
where
    for<'b> &'b P: Debug,
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IllusionArrayHandleMut")
            .field("index", &self.index)
            .field("value", &self.get())
            .finish()
    }
}

impl<'a, const W: usize, P> Iterator for IllusionArrayIterMut<'a, W, P>
where
    P: Clone + std::cmp::PartialEq + Default,
{
    type Item = IllusionArrayHandleMut<'a, W, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index < W {
            let handle = IllusionArrayHandleMut {
                inner: unsafe { &mut *(self.inner as *mut _) },
                index: self.curr_index,
                current: P::default().into(),
            };
            self.curr_index += 1;
            Some(handle)
        } else {
            None
        }
    }
}

pub struct IllusionArrayHandleMut<'a, const W: usize, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    inner: &'a mut InnerIllusionArray<W, P>,
    index: usize,
    current: Option<P>,
}

impl<'a, const W: usize, P> Drop for IllusionArrayHandleMut<'a, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + std::default::Default + Default,
{
    fn drop(&mut self) {
        let current = self.current.take().unwrap_or(P::default());

        if &current != &self.inner.default {
            if let Some(item) = self.inner.inner.get_mut(&self.index) {
                item.value = current;
            } else {
                self.inner
                    .inner
                    .insert(self.index, IllusionItem { value: current });
            }
        } else {
            // Item is now default! clean up
            self.inner.inner.remove(&self.index);
        }
    }
}

impl<'a, const W: usize, P> DerefMut for IllusionArrayHandleMut<'a, W, P>
where
    P: std::clone::Clone + std::cmp::PartialEq + Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<'a, const W: usize, P> Deref for IllusionArrayHandleMut<'a, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, const W: usize, P> IllusionArrayHandleMut<'a, W, P>
where
    P: std::cmp::PartialEq + std::clone::Clone + Default,
{
    pub fn get(&self) -> &P {
        self.inner
            .get(self.index)
            .expect("IllusionArrayHandleMut should be in bounds.")
    }

    pub fn mapped<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&P) -> T,
    {
        f(self.get())
    }

    pub fn get_mut(&mut self) -> &mut P
    where
        P: Clone,
    {
        self.inner
            .inner
            .get_mut(&self.index)
            .map(|item| &mut item.value)
            .unwrap_or(self.current.as_mut().unwrap())
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut Self)) {
        f(self);
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_1() {
        let mut arr = IllusionArray::<3, i32>::default();

        for item in arr.iter() {
            println!("{:?}", item);
        }
        println!("Filled len: {:?}", arr.filled_len());
        println!("---");

        for (i, mut item) in arr.iter_mut().enumerate() {
            *item += (i as i32) * 2;
        }

        for item in arr.iter() {
            println!("{:?}", item);
        }
        println!("Filled len: {:?}", arr.filled_len());
        println!("---");
    }
}
