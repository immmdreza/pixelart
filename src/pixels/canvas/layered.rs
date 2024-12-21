use thiserror::Error;

use crate::{
    pixels::{
        position::{IntoPixelStrictPosition, PixelStrictPosition},
        Pixel, PixelInitializer, PixelInterface, PixelMutInterface,
    },
    prelude::{Drawable, MaybePixel, PixelColor},
};

use super::PixelCanvas;

#[derive(Debug, Default)]
pub struct LayerData<const H: usize, const W: usize> {
    layer_tag: Option<String>,
    pub drawing_position: PixelStrictPosition<H, W>,
    pub canvas: PixelCanvas<H, W, MaybePixel>,
}

impl<const H: usize, const W: usize> LayerData<H, W> {
    pub fn new(
        layer_tag: impl Into<Option<String>>,
        canvas: PixelCanvas<H, W, MaybePixel>,
    ) -> Self {
        Self {
            drawing_position: PixelStrictPosition::new(0, 0).unwrap(),
            layer_tag: layer_tag.into(),
            canvas,
        }
    }

    pub fn new_without_tag(canvas: PixelCanvas<H, W, MaybePixel>) -> Self {
        Self::new(None, canvas)
    }

    pub fn build_new(
        layer_tag: impl Into<Option<String>>,
        canvas_builder: impl FnOnce(&mut PixelCanvas<H, W, MaybePixel>),
    ) -> Self {
        let mut canvas = PixelCanvas::default();
        canvas_builder(&mut canvas);
        Self {
            drawing_position: PixelStrictPosition::new(0, 0).unwrap(),
            layer_tag: layer_tag.into(),
            canvas,
        }
    }

    pub fn build_new_without_tag(
        canvas_builder: impl FnOnce(&mut PixelCanvas<H, W, MaybePixel>),
    ) -> Self {
        Self::build_new(None, canvas_builder)
    }

    pub fn layer_tag(&self) -> Option<&String> {
        self.layer_tag.as_ref()
    }

    pub fn with_drawing_position(
        mut self,
        start_position: impl IntoPixelStrictPosition<H, W>,
    ) -> Self {
        self.drawing_position = start_position.into_pixel_strict_position();
        self
    }

    pub fn with_layer_tag(mut self, tag: impl Into<String>) -> Self {
        self.layer_tag = Some(tag.into());
        self
    }

    pub fn with_modified_canvas(
        mut self,
        modifier: impl FnOnce(&mut PixelCanvas<H, W, MaybePixel>),
    ) -> Self {
        modifier(&mut self.canvas);
        self
    }

    pub fn update_drawing_position(
        &mut self,
        updater: impl FnOnce(&PixelStrictPosition<H, W>) -> PixelStrictPosition<H, W>,
    ) {
        self.drawing_position = updater(&self.drawing_position);
    }
}

#[derive(Debug, Error)]
pub enum AddLayerError {
    #[error("This layer tag is already used")]
    LayerTagDuplicated,
}

#[derive(Debug, Clone)]
pub enum TopLayerId {
    Tag(String),
    Index(usize),
}

impl From<usize> for TopLayerId {
    fn from(v: usize) -> Self {
        Self::Index(v)
    }
}

impl From<String> for TopLayerId {
    fn from(v: String) -> Self {
        Self::Tag(v)
    }
}

impl From<&'static str> for TopLayerId {
    fn from(v: &'static str) -> Self {
        Self::Tag(v.to_string())
    }
}

#[derive(Debug)]
pub struct LayeredCanvas<const H: usize, const W: usize = H, P: PixelInterface = Pixel> {
    pub(crate) base_layer: PixelCanvas<H, W, P>,
    pub(crate) top_layers: Vec<LayerData<H, W>>, // Top layers are all using maybe (transparent) pixel
}

impl<const H: usize, const W: usize, P: PixelInterface> LayeredCanvas<H, W, P> {
    pub fn new_layer(&mut self, layer_data: LayerData<H, W>) -> Result<usize, AddLayerError> {
        if let Some(tag) = &layer_data.layer_tag {
            if self
                .top_layers
                .iter()
                .any(|x| x.layer_tag.as_ref().is_some_and(|x| x == tag))
            {
                return Err(AddLayerError::LayerTagDuplicated);
            }
        }

        self.top_layers.push(layer_data);

        Ok(self.top_layers.len() - 1)
    }

    pub fn get_resulting_canvas<E>(&self) -> PixelCanvas<H, W, P>
    where
        P: Clone + PixelMutInterface,
        P::ColorType: TryFrom<Option<PixelColor>, Error = E>,
    {
        let mut base = self.base_layer.clone();
        for top in self.top_layers.iter() {
            top.canvas.draw_on_exact(top.drawing_position, &mut base);
        }
        base
    }

    pub fn base_layer(&self) -> &PixelCanvas<H, W, P> {
        &self.base_layer
    }

    pub fn base_layer_mut(&mut self) -> &mut PixelCanvas<H, W, P> {
        &mut self.base_layer
    }

    pub fn top_layer(&self, layer_id: impl Into<TopLayerId>) -> Option<&LayerData<H, W>> {
        let layer_id: TopLayerId = layer_id.into();
        match layer_id {
            TopLayerId::Tag(tag) => self
                .top_layers
                .iter()
                .find(|x| x.layer_tag.as_ref().is_some_and(|x| x == &tag)),
            TopLayerId::Index(index) => self.top_layers.get(index),
        }
    }

    pub fn top_layer_mut(
        &mut self,
        layer_id: impl Into<TopLayerId>,
    ) -> Option<&mut LayerData<H, W>> {
        let layer_id: TopLayerId = layer_id.into();
        match layer_id {
            TopLayerId::Tag(tag) => self
                .top_layers
                .iter_mut()
                .find(|x| x.layer_tag.as_ref().is_some_and(|x| x == &tag)),
            TopLayerId::Index(index) => self.top_layers.get_mut(index),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + PixelInitializer> Default
    for LayeredCanvas<H, W, P>
where
    <P as PixelInterface>::ColorType: std::default::Default + Clone,
{
    fn default() -> Self {
        Self {
            base_layer: PixelCanvas::default(),
            top_layers: Vec::new(),
        }
    }
}

impl<const H: usize, const W: usize, P: PixelInterface + PixelInitializer> LayeredCanvas<H, W, P> {
    pub fn new(color: impl Into<P::ColorType> + Clone) -> Self {
        Self {
            base_layer: PixelCanvas::new(color),
            top_layers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pixels::{
            canvas::{
                templates::alien_monster::AlienMonster, SharedMutPixelCanvasExt,
                SharedPixelCanvasExt,
            },
            position::{Direction, PixelStrictPositionInterface},
        },
        prelude::{CENTER, MAGENTA, TOP_LEFT},
    };

    use super::*;

    #[test]
    fn test_name() {
        let mut layered = LayeredCanvas::<50>::new(MAGENTA);

        layered
            .new_layer(
                LayerData::default()
                    .with_layer_tag("Alien")
                    .with_drawing_position(TOP_LEFT.bounding_direction(Direction::DownRight, 2))
                    .with_modified_canvas(|canvas| {
                        canvas.draw(TOP_LEFT, AlienMonster);
                    }),
            )
            .unwrap();

        layered
            .new_layer(
                LayerData::default()
                    .with_layer_tag("Alien 2")
                    .with_drawing_position(CENTER)
                    .with_modified_canvas(|canvas| {
                        canvas.draw(TOP_LEFT, AlienMonster);
                    }),
            )
            .unwrap();

        layered
            .get_resulting_canvas()
            .default_image_builder()
            .save("arts/layered_0.png")
            .unwrap()
    }
}
