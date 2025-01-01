use std::marker::PhantomData;

use crate::{
    pixels::{
        position::{IntoPixelStrictPosition, PixelStrictPosition, PixelStrictPositionInterface},
        PixelInitializer, PixelInterface, PixelMutInterface,
    },
    prelude::{Drawable, MaybePixel},
};

use super::{
    table::PixelTable, PixelCanvasInterface, PixelCanvasMutInterface, SharedMutPixelCanvasExt,
};

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
    const MH: usize,
    const MW: usize,
    const SH: usize,
    const SW: usize,
    I,
    SP,
    MP = MaybePixel,
> where
    SP: PixelInterface + Default,
    MP: PixelInterface + Default,
    I: PixelCanvasInterface<SH, SW, SP>,
{
    position: PixelStrictPosition<SH, SW>,
    source_table: I,
    partition_table: PixelTable<MH, MW, MP>,
    partition_snapshot_table: PixelTable<MH, MW, MP>,
    _phantom: PhantomData<SP>,
}

impl<const MH: usize, const MW: usize, const SH: usize, const SW: usize, SP, MP, I>
    Drawable<MH, MW, MP> for CanvasPartition<MH, MW, SH, SW, I, SP, MP>
where
    MP::ColorType: Clone,
    SP: PixelInterface + Default,
    MP: PixelInterface + Default,
    I: PixelCanvasInterface<SH, SW, SP>,
{
    fn draw_on<const HC: usize, const WC: usize, P, C, E>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface + PartialEq + Clone + Default,
        C: PixelCanvasMutInterface<HC, WC, P>,
        P::ColorType: TryFrom<MP::ColorType, Error = E>,
    {
        for (my_position, source_position) in
            Self::_included_positions::<MH, MW, HC, WC>(start_pos.into_pixel_strict_position())
        {
            let my_color = self.partition_table.get_pixel(my_position).color().clone();
            if let Ok(my_color) = P::ColorType::try_from(my_color) {
                canvas
                    .table_mut()
                    .get_pixel_mut(source_position)
                    .update_color(my_color);
            }
        }
    }
}

impl<const SH: usize, const SW: usize, const MH: usize, const MW: usize, SP, MP, I>
    PixelCanvasMutInterface<MH, MW, MP> for CanvasPartition<MH, MW, SH, SW, I, SP, MP>
where
    SP: PixelInterface + Default,
    MP: PixelInterface + PixelMutInterface + Default,
    I: PixelCanvasInterface<SH, SW, SP>,
{
    fn table_mut(&mut self) -> &mut PixelTable<MH, MW, MP> {
        &mut self.partition_table
    }
}

impl<const SH: usize, const SW: usize, const MH: usize, const MW: usize, SP, MP, I>
    PixelCanvasInterface<MH, MW, MP> for CanvasPartition<MH, MW, SH, SW, I, SP, MP>
where
    SP: PixelInterface + Default,
    MP: PixelInterface + Default,
    I: PixelCanvasInterface<SH, SW, SP>,
{
    fn table(&self) -> &PixelTable<MH, MW, MP> {
        &self.partition_table
    }
}

impl<const SH: usize, const SW: usize, const MH: usize, const MW: usize, SP, MP, I>
    CanvasPartition<MH, MW, SH, SW, I, SP, MP>
where
    SP: PixelInterface + Default,
    MP: PixelInterface + Default,
    I: PixelCanvasInterface<SH, SW, SP>,
{
    fn _included_positions<
        const MMH: usize,
        const MMW: usize,
        const SSH: usize,
        const SSW: usize,
    >(
        start_position: PixelStrictPosition<SSH, SSW>,
    ) -> impl Iterator<Item = (PixelStrictPosition<MMH, MMW>, PixelStrictPosition<SSH, SSW>)> {
        (0..MMH).flat_map(move |row_offset| {
            (0..MMW).filter_map(move |column_offset| {
                start_position
                    .checked_right(row_offset)
                    .ok()
                    .and_then(|f| f.checked_down(column_offset).ok())
                    .map(|f| {
                        (
                            PixelStrictPosition::<MMH, MMW>::new(row_offset, column_offset)
                                .unwrap(),
                            f,
                        )
                    })
            })
        })
    }

    fn _read_source<const MMH: usize, const MMW: usize>(
        source_table: &I,
        position: PixelStrictPosition<SH, SW>,
    ) -> PixelTable<MMH, MMW, MP>
    where
        MP: PixelMutInterface + PixelInitializer + Clone + PartialEq,
        MP::ColorType: Clone + Default,
        SP::ColorType: Clone,
        MP::ColorType: From<SP::ColorType>,
    {
        let mut partition_table = PixelTable::<MMH, MMW, MP>::default();
        for (my_position, source_position) in Self::_included_positions(position) {
            let source_color = source_table
                .table()
                .get_pixel(source_position)
                .color()
                .clone();
            partition_table
                .get_pixel_mut(my_position)
                .update_color(source_color);
        }
        partition_table
    }

    fn read_source(&mut self) -> PixelTable<MH, MW, MP>
    where
        MP: PixelMutInterface + PixelInitializer + PartialEq + Clone,
        MP::ColorType: Clone + Default,
        SP::ColorType: Clone,
        MP::ColorType: From<SP::ColorType>,
    {
        Self::_read_source(&self.source_table, self.position)
    }

    fn set_source_color<E>(&mut self, color: impl Into<Option<SP::ColorType>>)
    where
        SP: PixelMutInterface + PartialEq + Clone,
        I: PixelCanvasMutInterface<SH, SW, SP>,
        MP::ColorType: Clone,
        SP::ColorType: TryFrom<MP::ColorType, Error = E> + Clone,
    {
        let chosen_color: Option<SP::ColorType> = color.into();
        for (part_position, source_position) in self.included_positions() {
            if self.partition_table.get_pixel(part_position).has_color() {
                if let Some(color) = &chosen_color {
                    self.source_table
                        .table_mut()
                        .get_pixel_mut(source_position)
                        .update_color(color.clone());
                } else if let Ok(color) = SP::ColorType::try_from(
                    self.partition_snapshot_table
                        .get_pixel(part_position)
                        .color()
                        .clone(),
                ) {
                    self.source_table
                        .table_mut()
                        .get_pixel_mut(source_position)
                        .update_color(color);
                }
            }
        }
    }

    pub fn write_source<E>(&mut self)
    where
        I: PixelCanvasMutInterface<SH, SW, SP>,
        SP: PixelMutInterface + PartialEq + Clone,
        SP::ColorType: TryFrom<MP::ColorType, Error = E> + Clone,
        MP: PixelMutInterface + PartialEq + Clone,
        MP::ColorType: From<SP::ColorType> + Clone,
    {
        for (my_position, source_position) in self.included_positions() {
            if self.partition_table.get_pixel(my_position).has_color() {
                let new_color = self.partition_table.get_pixel(my_position).color().clone();
                let source_current_color = self
                    .source_table
                    .table()
                    .get_pixel(source_position)
                    .color()
                    .clone();

                if let Ok(new_color) = SP::ColorType::try_from(new_color) {
                    self.partition_snapshot_table
                        .get_pixel_mut(my_position)
                        .update_color(source_current_color);
                    self.source_table
                        .table_mut()
                        .get_pixel_mut(source_position)
                        .update_color(new_color);
                }
            }
        }
    }

    pub fn new(
        position: impl IntoPixelStrictPosition<SH, SW>,
        source_table: I,
    ) -> CanvasPartition<MH, MW, SH, SW, I, SP, MP>
    where
        MP: PixelMutInterface + PixelInitializer + PartialEq + Clone,
        MP::ColorType: Clone + Default + From<SP::ColorType>,
        SP::ColorType: Clone,
    {
        let start_position = position.into_pixel_strict_position();
        CanvasPartition::<MH, MW, SH, SW, I, SP, MP> {
            partition_table: Self::_read_source(&source_table, start_position),
            position: start_position,
            source_table,
            partition_snapshot_table: Default::default(),
            _phantom: PhantomData,
        }
    }

    pub fn included_positions(
        &self,
    ) -> impl Iterator<Item = (PixelStrictPosition<MH, MW>, PixelStrictPosition<SH, SW>)> {
        Self::_included_positions(self.position)
    }

    pub fn update_position(&mut self, new_position: impl IntoPixelStrictPosition<SH, SW>)
    where
        MP: PixelMutInterface + PixelInitializer + PartialEq + Clone,
        MP::ColorType: Clone + Default,
        SP::ColorType: Clone,
        MP::ColorType: From<SP::ColorType>,
    {
        self.position = new_position.into_pixel_strict_position();
        self.read_source();
    }

    pub fn update_color<E>(&mut self, color: impl Into<MP::ColorType> + Clone)
    where
        MP: PixelMutInterface + PartialEq + Clone,
        MP::ColorType: From<SP::ColorType> + Clone,
        SP: PixelMutInterface + PartialEq + Clone,
        I: PixelCanvasMutInterface<SH, SW, SP>,
        SP::ColorType: TryFrom<MP::ColorType, Error = E> + Clone,
    {
        SharedMutPixelCanvasExt::fill(self, color);
        self.write_source();
    }

    /// .
    pub fn crop_to<E>(&mut self, new_position: impl IntoPixelStrictPosition<SH, SW>)
    where
        MP: PixelMutInterface + PartialEq + Clone,
        MP::ColorType: From<SP::ColorType> + Clone,
        SP::ColorType: TryFrom<MP::ColorType, Error = E> + Clone + Default,
        SP: PixelMutInterface + PartialEq + Clone,
        I: PixelCanvasMutInterface<SH, SW, SP>,
    {
        self.set_source_color(None);
        self.position = new_position.into_pixel_strict_position();
        self.write_source();
    }

    /// .
    pub fn copy_to<E>(&mut self, new_position: impl IntoPixelStrictPosition<SH, SW>)
    where
        MP: PixelMutInterface + PartialEq + Clone,
        MP::ColorType: Clone + From<SP::ColorType>,
        SP: PixelMutInterface + PartialEq + Clone,
        I: PixelCanvasMutInterface<SH, SW, SP>,
        SP::ColorType: TryFrom<MP::ColorType, Error = E> + Clone,
    {
        self.position = new_position.into_pixel_strict_position();
        self.write_source();
    }

    /// Returns a mutable reference to the partition table of this [`CanvasPartition<SH, SW, MH, MW, I, SP, MP>`].
    pub fn partition_table_mut(&mut self) -> &mut PixelTable<MH, MW, MP> {
        &mut self.partition_table
    }

    /// Returns a reference to the partition table of this [`CanvasPartition<SH, SW, MH, MW, I, SP, MP>`].
    pub fn partition_table(&self) -> &PixelTable<MH, MW, MP> {
        &self.partition_table
    }

    pub fn source_table(&self) -> &I {
        &self.source_table
    }

    pub fn position(&self) -> PixelStrictPosition<SH, SW> {
        self.position
    }

    pub fn source_table_mut(&mut self) -> &mut I {
        &mut self.source_table
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::CanvasPartition;

    #[test]
    fn feature_1() {
        let mut canvas = PixelCanvas::<5>::default();
        // Captures a 2x2 partition from top left of the canvas
        let mut part = canvas.maybe_partition_mut::<2, 2>(TOP_LEFT);

        part.update_color(RED);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/partition_0.png")
            .unwrap()
    }

    #[test]
    fn feature_2() {
        let mut canvas = PixelCanvas::<5>::default();
        let mut part = CanvasPartition::<2, 2, 5, 5, _, _, MaybePixel>::new(TOP_LEFT, &mut canvas);

        part.update_color(RED);

        part.crop_to(TOP_CENTER);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/partition_crop.png")
            .unwrap()
    }

    #[test]
    fn feature_3() {
        let mut canvas = PixelCanvas::<5>::default();
        let mut part = CanvasPartition::<2, 2, 5, 5, _, _, MaybePixel>::new(TOP_LEFT, &mut canvas);

        part.update_color(RED);

        part.copy_to(TOP_CENTER);
        part.copy_to(CENTER);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/partition_crop_1.png")
            .unwrap()
    }

    #[test]
    fn feature_4() {
        let mut canvas = PixelCanvas::<5>::default();
        let mut part = CanvasPartition::<2, 2, 5, 5, _, _, MaybePixel>::new(TOP_LEFT, &mut canvas);

        part.update_color(RED);

        let mut canvas2 = PixelCanvas::<5>::default();

        part.draw_on(LEFT_CENTER, &mut canvas2);

        canvas2
            .default_image_builder()
            .with_scale(5)
            .save("arts/partition_crop_2.png")
            .unwrap()
    }
}
