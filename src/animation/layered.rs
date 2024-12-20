use std::path::Path;

use image::codecs::gif::Repeat;

use crate::{
    pixels::{
        canvas::{layered::LayeredCanvas, SharedPixelCanvasExt},
        PixelInterface, PixelMutInterface,
    },
    prelude::PixelColor,
};

use super::{AnimationContext, PixelAnimationBuilder};

#[cfg(feature = "viewer")]
use crate::viewer::ViewResult;

pub struct LayeredAnimationContext<const H: usize, const W: usize, P: PixelInterface> {
    pub(crate) frame_count: Repeat,
    pub(crate) layered_canvas: LayeredCanvas<H, W, P>,
    pub(crate) builder: PixelAnimationBuilder,
}

impl<const H: usize, const W: usize, P: PixelInterface> LayeredAnimationContext<H, W, P> {
    pub fn new(
        layered_canvas: LayeredCanvas<H, W, P>,
        builder: PixelAnimationBuilder,
        frame_count: Repeat,
    ) -> Self {
        Self {
            layered_canvas,
            builder,
            frame_count,
        }
    }

    pub fn layered_canvas(&self) -> &LayeredCanvas<H, W, P> {
        &self.layered_canvas
    }

    pub fn layered_canvas_mut(&mut self) -> &mut LayeredCanvas<H, W, P> {
        &mut self.layered_canvas
    }

    pub fn save<PA: AsRef<Path>>(self, path: PA) -> Result<(), image::ImageError> {
        self.builder.save(path)
    }

    #[cfg(feature = "viewer")]
    pub fn view(self) -> ViewResult {
        self.builder.view()
    }
}

impl<const H: usize, const W: usize, P: PixelInterface> AnimationContext<H, W, P>
    for LayeredAnimationContext<H, W, P>
where
    P: Clone + PixelMutInterface,
    <P as PixelInterface>::ColorType: TryFrom<Option<PixelColor>>,
{
    fn builder(&self) -> &PixelAnimationBuilder {
        &self.builder
    }

    fn builder_mut(&mut self) -> &mut PixelAnimationBuilder {
        &mut self.builder
    }

    fn canvas(&self) -> &crate::prelude::PixelCanvas<H, W, P> {
        self.layered_canvas.base_layer()
    }

    fn canvas_mut(&mut self) -> &mut crate::prelude::PixelCanvas<H, W, P> {
        self.layered_canvas.base_layer_mut()
    }

    fn frame_count(&self) -> &Repeat {
        &self.frame_count
    }

    fn get_frame_to_capture(&self) -> crate::image::DefaultImageBuffer
    where
        <P as PixelInterface>::ColorType: crate::pixels::color::RgbaInterface,
    {
        self.layered_canvas()
            .get_resulting_canvas()
            .default_image_builder()
            .get_image()
    }
}

#[cfg(test)]
mod tests {
    use image::codecs::gif::Repeat;

    use crate::{
        animation::Animated,
        pixels::{
            canvas::{
                layered::LayerData,
                templates::{alien_monster::AlienMonster, heart::Heart},
                SharedMutPixelCanvasExt,
            },
            position::PixelStrictPositionInterface,
            Pixel,
        },
        prelude::*,
    };

    use super::*;

    struct MyLayeredAnimation;

    impl Animated<50, 50, Pixel> for MyLayeredAnimation {
        type ContextType = LayeredAnimationContext<50, 50, Pixel>;

        fn create_context(&mut self) -> Self::ContextType {
            LayeredAnimationContext::new(
                LayeredCanvas::default(),
                PixelAnimationBuilder::new_empty(Repeat::Infinite, 2),
                Repeat::Finite(30),
            )
        }

        fn setup(&mut self, ctx: &mut Self::ContextType) {
            ctx.layered_canvas_mut().base_layer_mut().fill(YELLOW);
            ctx.layered_canvas_mut()
                .new_layer(LayerData::build_new("alien_1".to_string(), |canvas| {
                    canvas.draw(TOP_LEFT, AlienMonster);
                }))
                .unwrap();
            ctx.layered_canvas_mut()
                .new_layer(
                    LayerData::build_new("alien_2".to_string(), |canvas| {
                        canvas.draw(
                            TOP_LEFT.bounding_direction(
                                crate::pixels::position::Direction::DownRight,
                                2,
                            ),
                            AlienMonster,
                        );
                    })
                    .with_drawing_position(CENTER),
                )
                .unwrap();
        }

        fn update(&mut self, ctx: &mut Self::ContextType, i: u16) -> bool {
            if i <= 12 {
                ctx.layered_canvas_mut()
                    .top_layer_mut("alien_1")
                    .unwrap()
                    .update_drawing_position(|curr| curr.bounding_down(1));

                ctx.layered_canvas_mut()
                    .top_layer_mut("alien_2")
                    .unwrap()
                    .update_drawing_position(|curr| curr.bounding_left(1));
            }

            if i == 12 {
                ctx.layered_canvas_mut()
                    .new_layer(
                        LayerData::default()
                            .with_drawing_position(CENTER.bounding_left(4).bounding_up(4))
                            .with_layer_tag("heart")
                            .with_modified_canvas(|canvas| canvas.draw(TOP_LEFT, Heart)),
                    )
                    .unwrap();
            }

            if i > 12 && i <= 15 {
                ctx.layered_canvas_mut()
                    .top_layer_mut("heart")
                    .unwrap()
                    .update_drawing_position(|curr| {
                        curr.bounding_direction(crate::pixels::position::Direction::UpRight, 1)
                    });
            }

            if i > 15 && i <= 18 {
                ctx.layered_canvas_mut()
                    .top_layer_mut("heart")
                    .unwrap()
                    .update_drawing_position(|curr| curr.bounding_up(1));
            }

            true
        }
    }

    #[test]
    fn test_name() {
        MyLayeredAnimation
            .process()
            .save("arts/layered_animation_0.gif")
            .unwrap();
    }
}
