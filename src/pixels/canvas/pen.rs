use std::marker::PhantomData;

use crate::{
    pixels::{
        position::{
            Direction, IntoPixelStrictPosition, PixelStrictPosition, PixelStrictPositionInterface,
        },
        PixelInterface, PixelMutInterface,
    },
    prelude::PixelColor,
};

use super::PixelCanvasMutInterface;

pub trait CanvasAttachment {
    type CanvasType;
    type ColorType;
}

pub struct CanvasAttachedMarker<
    'c,
    const H: usize,
    const W: usize,
    P: PixelMutInterface,
    C: PixelCanvasMutInterface<H, W, P>,
> {
    current_pos: PixelStrictPosition<H, W>,
    _phantom: PhantomData<&'c (P, C)>,
}

impl<const H: usize, const W: usize, P: PixelMutInterface, C: PixelCanvasMutInterface<H, W, P>>
    CanvasAttachedMarker<'_, H, W, P, C>
{
    pub fn new(current_pos: impl IntoPixelStrictPosition<H, W>) -> Self {
        Self {
            current_pos: current_pos.into_pixel_strict_position(),
            _phantom: PhantomData,
        }
    }
}

impl<
        'c,
        const H: usize,
        const W: usize,
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<H, W, P>,
    > CanvasAttachment for CanvasAttachedMarker<'c, H, W, P, C>
{
    type CanvasType = &'c mut C;
    type ColorType = P::ColorType;
}

pub struct CanvasUnattachedMarker<Co = PixelColor>(PhantomData<Co>);

impl<Co> CanvasAttachment for CanvasUnattachedMarker<Co> {
    type CanvasType = ();
    type ColorType = Co;
}

/**
Pen can be attached to a canvas and do fun things on it.

## Example
```rust
# use pixelart::prelude::PixelCanvas;
# use pixelart::prelude::PixelColor;
# use pixelart::prelude::StrictPositions;
# use crate::pixelart::pixels::canvas::SharedMutPixelCanvasExt;
# use crate::pixelart::pixels::color::PixelColorExt;

let mut canvas = PixelCanvas::<5>::default();

canvas
    .attach_new_pen(PixelColor::CYAN, StrictPositions::BottomCenter)
    .start()
    .up(2)
    // Returns to the position before branching when you're done.
    .branch(|pen| pen.up_left(2))
    .up_right(2);
```
*/
pub struct Pen<M: CanvasAttachment = CanvasUnattachedMarker> {
    canvas: M::CanvasType,
    color: M::ColorType,
    pub drawing: bool,
    attachment: M,
}

impl<M: CanvasAttachment> Pen<M> {
    pub fn stop(&mut self) -> &mut Pen<M> {
        self.drawing = false;
        self
    }
}

impl<Co> Pen<CanvasUnattachedMarker<Co>> {
    pub fn new(color: impl Into<Co>) -> Pen<CanvasUnattachedMarker<Co>> {
        Self {
            canvas: (),
            color: color.into(),
            drawing: false,
            attachment: CanvasUnattachedMarker::<Co>(PhantomData),
        }
    }
}

pub type PixelPen = Pen<CanvasUnattachedMarker<PixelColor>>;

impl<Co> Pen<CanvasUnattachedMarker<Co>> {
    #[must_use = "This function returns a new attached pen."]
    pub fn attach<
        const H: usize,
        const W: usize,
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<H, W, P>,
    >(
        self,
        canvas: &mut C,
        start_pos: impl IntoPixelStrictPosition<H, W>,
    ) -> Pen<CanvasAttachedMarker<'_, H, W, P, C>>
    where
        <P as PixelInterface>::ColorType: From<Co>,
    {
        Pen {
            canvas,
            color: self.color.into(),
            drawing: false,
            attachment: CanvasAttachedMarker::new(start_pos),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelMutInterface, C: PixelCanvasMutInterface<H, W, P>>
    Pen<CanvasAttachedMarker<'_, H, W, P, C>>
{
    #[must_use = "This function returns a new unattached pen."]
    pub fn detach(self) -> Pen<CanvasUnattachedMarker<P::ColorType>> {
        Pen {
            canvas: (),
            color: self.color,
            drawing: self.drawing,
            attachment: CanvasUnattachedMarker(PhantomData),
        }
    }

    fn draw(&mut self) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        if self.drawing {
            self.canvas.table_mut()[self.attachment.current_pos].update_color(self.color.clone());
        }
        self
    }

    pub fn start(&mut self) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.drawing = true;
        self.draw()
    }

    fn go_direction_once(&mut self, dir: Direction) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.attachment.current_pos = self.attachment.current_pos.bounding_direction(dir, 1);
        self.draw()
    }

    pub fn go_direction(&mut self, dir: Direction, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        for _ in 0..how_many {
            self.go_direction_once(dir);
        }
        self
    }

    pub fn up(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::Up, how_many)
    }

    pub fn down(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::Down, how_many)
    }

    pub fn left(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::Left, how_many)
    }

    pub fn right(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::Right, how_many)
    }

    pub fn up_right(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::UpRight, how_many)
    }

    pub fn down_right(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::DownRight, how_many)
    }

    pub fn down_left(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::DownLeft, how_many)
    }

    pub fn up_left(&mut self, how_many: usize) -> &mut Self
    where
        <P as PixelInterface>::ColorType: From<PixelColor> + Clone,
    {
        self.go_direction(Direction::UpLeft, how_many)
    }

    pub fn branch<B: FnMut(&mut Self) -> &mut Self>(&mut self, mut b: B) -> &mut Self {
        let pos_before_branching = self.attachment.current_pos;
        b(self);
        self.attachment.current_pos = pos_before_branching;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::pixels::color::PixelColorExt;
    use crate::{
        pixels::canvas::{SharedMutPixelCanvasExt, SharedPixelCanvasExt},
        prelude::{PixelCanvas, StrictPositions},
    };

    use super::*;

    #[test]
    fn test_attachments() {
        let mut canvas = PixelCanvas::<5>::default();

        let red_pen = PixelPen::new(PixelColor::RED);
        let blue_pen = PixelPen::new(PixelColor::BLUE);

        let attached_pen = red_pen.attach(&mut canvas, StrictPositions::TopLeft);
        let _red_pen = attached_pen.detach();

        let mut attached_pen = blue_pen.attach(&mut canvas, StrictPositions::TopLeft);
        attached_pen
            .start()
            .right(2)
            .down_right(2)
            .down_left(2)
            .left(2);

        canvas.fill_inside(PixelColor::CYAN, StrictPositions::LeftCenter);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/pen_1.png")
            .unwrap();
    }

    #[test]
    fn test_pen_branching() {
        let mut canvas = PixelCanvas::<5>::default();

        canvas
            .attach_new_pen(PixelColor::CYAN, StrictPositions::BottomCenter)
            .start()
            .up(2)
            .branch(|pen| pen.up_left(2))
            .up_right(2);

        canvas
            .default_image_builder()
            .with_scale(5)
            .save("arts/pen_0.png")
            .unwrap();
    }
}
