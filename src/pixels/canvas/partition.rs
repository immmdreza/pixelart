use std::marker::PhantomData;

use crate::{
    pixels::{
        position::{
            IntoPixelStrictPosition, PixelPositionInterface, PixelStrictPosition,
            PixelStrictPositionInterface,
        },
        Pixel, PixelInterface, PixelMutInterface,
    },
    prelude::{Drawable, MaybePixel},
};

use super::{drawable::draw_canvas_on, PixelCanvasInterface, PixelCanvasMutInterface};

#[derive(Debug, Clone)]
pub struct BoxIndicator<const H: usize, const W: usize = H> {
    top_left: PixelStrictPosition<H, W>,
    bottom_right: PixelStrictPosition<H, W>,
}

impl<const H: usize, const W: usize> BoxIndicator<H, W> {
    pub fn new(
        top_left: impl IntoPixelStrictPosition<H, W>,
        bottom_right: impl IntoPixelStrictPosition<H, W>,
    ) -> Self {
        Self {
            top_left: top_left.into_pixel_strict_position(),
            bottom_right: bottom_right.into_pixel_strict_position(),
        }
    }
}

impl<const H: usize, const W: usize> IntoIterator for BoxIndicator<H, W> {
    type Item = PixelStrictPosition<H, W>;
    type IntoIter = BoxIndicatorIter<H, W>;

    fn into_iter(self) -> Self::IntoIter {
        BoxIndicatorIter {
            indicator: self,
            current: None,
        }
    }
}

pub struct BoxIndicatorIter<const H: usize, const W: usize> {
    indicator: BoxIndicator<H, W>,
    current: Option<PixelStrictPosition<H, W>>,
}

impl<const H: usize, const W: usize> Iterator for BoxIndicatorIter<H, W> {
    type Item = PixelStrictPosition<H, W>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &self.current {
            if current.column() == self.indicator.bottom_right.column() {
                // Need to go to the next line
                if current.row() == self.indicator.bottom_right.row() {
                    // This is the end!
                    return None;
                } else {
                    self.current = Some(
                        PixelStrictPosition::new(
                            current.row() + 1,
                            self.indicator.top_left.column(),
                        )
                        .unwrap(),
                    );
                }
            } else {
                self.current = Some(current.checked_right(1).unwrap());
            }
        } else {
            self.current = Some(self.indicator.top_left)
        }
        self.current
    }
}

pub struct CanvasPartition<
    const H: usize,
    const W: usize,
    P: PixelInterface,
    I: PixelCanvasInterface<H, W, P>,
> {
    table: I,
    positions: Vec<PixelStrictPosition<H, W>>,
    _phantom: PhantomData<P>,
}

impl<const H: usize, const W: usize, P: PixelInterface, I: PixelCanvasInterface<H, W, P>>
    PixelCanvasInterface<H, W, P> for CanvasPartition<H, W, P, I>
{
    fn table(&self) -> &super::table::PixelTable<H, W, P> {
        self.table.table()
    }
}

impl<const H: usize, const W: usize, P: PixelMutInterface, I: PixelCanvasMutInterface<H, W, P>>
    PixelCanvasMutInterface<H, W, P> for CanvasPartition<H, W, P, I>
{
    fn table_mut(&mut self) -> &mut super::table::PixelTable<H, W, P> {
        self.table.table_mut()
    }
}

impl<const H: usize, const W: usize, P: PixelInterface, I: PixelCanvasInterface<H, W, P>>
    CanvasPartition<H, W, P, I>
{
    pub fn new(
        table: I,
        positions: impl IntoIterator<Item = impl IntoPixelStrictPosition<H, W>>,
    ) -> Self {
        Self {
            table,
            positions: positions
                .into_iter()
                .map(|f| f.into_pixel_strict_position())
                .collect(),
            _phantom: PhantomData,
        }
    }
}

impl<const H: usize, const W: usize, P: PixelMutInterface, I: PixelCanvasMutInterface<H, W, P>>
    CanvasPartition<H, W, P, I>
{
    pub fn update_color(&mut self, color: impl Into<P::ColorType> + Clone) {
        for pixel in self.table.table_mut().iter_pixels_mut().filter(|p| {
            self.positions.iter().any(|q| {
                let pos = p.position();
                pos.row() == q.row() && pos.column() == q.column()
            })
        }) {
            pixel.update_color(color.clone());
        }
    }
}

// Make partition drawable
impl<const H: usize, const W: usize, I: PixelCanvasInterface<H, W, MaybePixel>> Drawable<H, W>
    for CanvasPartition<H, W, MaybePixel, I>
{
    fn draw_on<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<HC, WC, P>,
        <P as PixelInterface>::ColorType: From<crate::prelude::PixelColor>,
    {
        self.table().draw_on(start_pos, canvas);
    }
}

impl<const H: usize, const W: usize, I: PixelCanvasInterface<H, W, Pixel>> Drawable<H, W>
    for CanvasPartition<H, W, Pixel, I>
{
    fn draw_on<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<HC, WC, P>,
        <P as PixelInterface>::ColorType: From<crate::prelude::PixelColor>,
    {
        draw_canvas_on(self.table(), start_pos, canvas)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_name() {
        let mut canvas = PixelCanvas::<5>::default();

        let mut partition = canvas.partition_mut(TOP_LEFT, CENTER);

        partition.update_color(RED);

        let mut other_canvas = PixelCanvas::<5>::default();

        // Copy from canvas Paste on other_canvas
        partition.draw_on(CENTER, &mut other_canvas);

        other_canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/partition.png")
            .unwrap();
    }
}
