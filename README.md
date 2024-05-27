# (What is) PixelArt👾?

Code to pixel image! This is what this library dose at the moment.

![Alien monster](arts/alien_monster.png)

**Made by pixelart 👾** _([This is how](src/pixels/canvas/templates/alien_monster.rs))_.

## Why?

The very first-stone purpose of this lib is to somehow visualize what your code
dose for maybe learning and understating purposes, and of course having fun!

As I already used a lot in tests, If the generated pixel image seems right, you can confirm that your code is also working right. That's one of the _"learning and understating purposes"_, I mentioned before.

We map a simple **fixed size** 2D array of pixels to pixels on a real image. So you
can generate image and see how it changes based on what you do with that array.

## How?

Simply!

### Quick example

Take a look at the example below:

```rust
// Let the fun begins ...
use pixelart::prelude::*;

pub fn main() {
    // A 5x5 pixel canvas.
    let mut canvas = PixelCanvas::<5>::default();

    // mutably access the pixel at the center.
    let center_pixel = &mut canvas[StrictPositions::Center];
    // Change its color to blue.
    center_pixel.color = PixelColor::BLUE;

    // Create and save image based on the canvas.
    let image_builder = canvas.default_image_builder().with_scale(5);
    image_builder.save("my_first_art.png").unwrap();
}

```

This will be the result.

![Your first art](arts/my_first_art.png)

### Breakdown

Let's break down and see what happened though it must be clear enough.

1. Create a default `PixelCanvas`.

    The `PixelCanvas` is a wrapper over so called _"fixed size 2D array of pixels"_.
    The inner type is a `PixelTable` which is an array of `PixelRow` which is an array what? you guess.

    This _some sort of fish_ syntax `::<5>` indicates the size of our array. A 2D array
    has a height (`H`, rows count) and a width (`W`, columns count).

    In this case we set `H` to `5`, which implicitly sets `W` to `5` if you don't set
    that explicitly. So it's equivalent to `::<5, 5>` (`::<H, W>`).

    You could for example do something like this:

    ```rust
    // ---- sniff ----

    // A 3x5 pixel canvas.
    let mut canvas = PixelCanvas::<3, 5>::default();

    // ---- sniff ----
    ```

    The result will be:

    ![Your mini first art](arts/my_mini_first_art.png)

2. Accessing the pixel at the center.

    You can index into canvas using positions (a combination of row and column of a pixel). In this case we use `StrictPositions` enum which can magically generate
    positions based on canvas size. `StrictPositions::Center` is center position (`2, 2` as we start from zero you know).

    `&mut` means I need mutable access to the pixel to change its color, otherwise i can't.

    And the we can change color property of the pixel to `PixelColor::BLUE`.

3. Generating image.

   The last part in to generate image. We first get the image builder based on canvas
   and then we scale it up a bit (`.with_scale`), to make it more visible.

   And jesus please save me as png (With respect).

You can do many other things like iterating over rows and pixels, creating templates and more ...

### More?

The library aims to provide more method and type to make your life easier, some of these functionalities are:

1. Using `MaybePixel` instead of `Pixel` in canvas allows us to [Create Templates](examples/src/template.rs).

2. In above examples you can review usage of rust iterables and extension methods.

3. Using [Pen](examples/src/pen.rs) to have fun.

## Where?

1. For now we're focusing on creating images and make it as smooth as possible. But there're more to do.
2. Generating pixel animations (More likely).
3. Add some interactivity maybe?! (Not sure about this, must be so hard).

## License

Licensed under the mercy and kindness of GOD.

Please use and help making it more useable (Contribute I mean).

## Next?

Just _Remember to have fun 🍟_.
