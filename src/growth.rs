#![cfg(test)]
#![cfg(debug_assertions)]

use std::any::Any;

use image::codecs::gif::Repeat;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    animation::{Animated, Animation, AnimationContext},
    pixels::canvas::templates::square,
    prelude::*,
};

fn create_and_save_canvas_of_size<const H: usize, const W: usize>() {
    let mut px = PixelCanvas::<H, W>::default();

    px.draw(CENTER, square::<1>(BLACK));
    px.default_image_builder().save("arts/growth.png").unwrap();
}

fn create_and_save_maybe_canvas_of_size<const H: usize, const W: usize>() {
    let mut px = PixelCanvas::<H, W, MaybePixel>::default();

    px.draw(CENTER, square::<1>(BLACK));
    px.default_image_builder()
        .save("arts/maybe_growth.png")
        .unwrap();
}

#[test]
fn test_growth() {
    // That's enough ?!
    // A single 313x313 normal canvas (not transparent) will cause the stack to overflow!
    // currently ~97,000 normal [`Pixel`]s can be handled at most.
    // Pretty enough for something that should look pixely, But many things IDK.
    create_and_save_canvas_of_size::<312, 312>();
}

#[test]
fn test_maybe_growth() {
    // That's enough ?!
    // A single 271x271 maybe_canvas will cause the stack to overflow!
    // currently ~73,000 [`MaybePixel`]s can be handled at most.
    // Pretty enough for something that should look pixely, But many things IDK.
    create_and_save_maybe_canvas_of_size::<270, 270>();
}

#[test]
fn random_canvas() {
    let mut rng = ThreadRng::default();
    let mut px = PixelCanvas::<109>::default();

    px.iter_pixels_mut().for_each(|mut pixel| {
        pixel.update_color(PixelColor::new(rng.gen(), rng.gen(), rng.gen()));
    });

    px.default_image_builder().save("arts/random.png").unwrap();
}

#[test]
fn random_animation() {
    let rng: ThreadRng = Default::default();

    Animation::new(
        || AnimationContext::<50>::new_with_extra(Repeat::Finite(10), ThreadRng::default()),
        |ctx| {
            let mut rng = rng.clone();
            ctx.canvas.iter_pixels_mut().for_each(|mut pixel| {
                pixel.update_color(PixelColor::new(rng.gen(), rng.gen(), rng.gen()));
            });
        },
        |ctx, _i| {
            let mut rng = rng.clone();
            ctx.canvas.iter_pixels_mut().for_each(|mut pixel| {
                pixel.update_color(PixelColor::new(rng.gen(), rng.gen(), rng.gen()));
            });

            true
        },
    )
    .create()
    .builder
    .save("arts/random_animation.gif")
    .unwrap();
}

#[test]
fn size() {
    let test: [Box<dyn Any>; 2] = [Box::new("10usize".to_string()), Box::new(10usize)];

    println!("{}", size_of_val(&test))
}
