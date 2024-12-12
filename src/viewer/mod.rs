#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};

use eframe::egui::IconData;
use eframe::{
    egui::{self, ColorImage, TextureHandle, TextureOptions},
    CreationContext,
};
use image::{ImageBuffer, Rgba};
use uuid::Uuid;

use crate::pixels::canvas::templates::alien_monster::AlienMonster;
use crate::pixels::canvas::{SharedMutPixelCanvasExt, SharedPixelCanvasExt};
use crate::prelude::{MaybePixel, PixelCanvas};

pub type ViewResult = eframe::Result;

const PIXELART_ICON: LazyLock<IconData> = LazyLock::new(|| get_icon());

fn get_icon() -> IconData {
    let mut canvas = PixelCanvas::<20, 20, MaybePixel>::default();
    canvas.draw((1, 0), AlienMonster);
    let img = canvas.default_image_builder().get_image();

    IconData {
        height: img.height(),
        width: img.width(),
        rgba: img.into_raw(),
    }
}

pub(crate) fn view(
    images: impl IntoIterator<Item = ImageBuffer<Rgba<u8>, Vec<u8>>>,
) -> eframe::Result {
    let images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = images.into_iter().collect();
    let first_msg = images.first().expect("At least one image is excepted.");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_icon(PIXELART_ICON.clone())
            .with_inner_size([
                (first_msg.width() as f32 + 20.),
                (first_msg.height() as f32 + 20.),
            ]),
        ..Default::default()
    };
    eframe::run_native(
        "Pixelart",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::<MyApp>::new(MyApp::new(cc, images)))
        }),
    )
}

#[derive(Clone)]
struct ImageTextureInfo {
    image_id: Uuid,
    texture: TextureHandle,
    image_height: f32,
    image_width: f32,
}

#[derive(Default)]
struct ViewPortData {
    show_viewport: Arc<AtomicBool>,
}

struct MyApp {
    textures: Vec<ImageTextureInfo>,

    /// Data for viewports
    viewports_data: Arc<HashMap<Uuid, ViewPortData>>,
}

impl MyApp {
    fn new(cc: &CreationContext, images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) -> Self {
        let textures: Vec<_> = images
            .into_iter()
            .map(|image| ImageTextureInfo {
                image_id: Uuid::new_v4(),
                texture: cc.egui_ctx.load_texture(
                    "screen",
                    ColorImage::from_rgba_unmultiplied(
                        [image.width() as usize, image.height() as usize],
                        &image.clone().into_raw(),
                    ),
                    TextureOptions::default(),
                ),
                image_height: image.height() as f32,
                image_width: image.width() as f32,
            })
            .collect();

        Self {
            viewports_data: Arc::new(
                textures
                    .iter()
                    .map(|f| {
                        (
                            f.image_id,
                            ViewPortData {
                                show_viewport: Arc::new(true.into()),
                            },
                        )
                    })
                    .collect(),
            ),
            textures,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut textures_info_iter = self.textures.clone().into_iter();
                let first_texture_info = textures_info_iter.next().unwrap();

                ui.add(
                    egui::Image::new(&first_texture_info.texture) // ERROR GONE
                        .max_height(first_texture_info.image_height)
                        .max_width(first_texture_info.image_width),
                );

                for texture_info in textures_info_iter {
                    if self.viewports_data[&texture_info.image_id]
                        .show_viewport
                        .load(Ordering::Relaxed)
                    {
                        let show_deferred_viewport = self.viewports_data[&texture_info.image_id]
                            .show_viewport
                            .clone();
                        ctx.show_viewport_deferred(
                            egui::ViewportId::from_hash_of(texture_info.image_id),
                            egui::ViewportBuilder::default()
                                .with_title("Pixelart")
                                .with_resizable(false)
                                .with_maximize_button(false)
                                .with_icon(PIXELART_ICON.clone())
                                .with_inner_size([
                                    (texture_info.image_width + 20.),
                                    (texture_info.image_height + 20.),
                                ]),
                            move |ctx, class| {
                                assert!(
                                    class == egui::ViewportClass::Deferred,
                                    "This egui backend doesn't support multiple viewports"
                                );

                                egui::CentralPanel::default().show(ctx, |ui| {
                                    egui::ScrollArea::both().show(ui, |ui| {
                                        ui.add(
                                        egui::Image::new(&texture_info.texture) // ERROR GONE
                                            .max_height(texture_info.image_height)
                                            .max_width(texture_info.image_width));
                                    })
                                });

                                if ctx.input(|i| i.viewport().close_requested()) {
                                    // Tell parent to close us.
                                    show_deferred_viewport.store(false, Ordering::Relaxed);
                                }
                            },
                        );
                    }
                }
            });
        });
    }
}
