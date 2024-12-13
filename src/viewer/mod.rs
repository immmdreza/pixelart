#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};

use atomic_time::AtomicInstant;
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

pub fn view<T: IntoIterator<Item = ImageBuffer<Rgba<u8>, Vec<u8>>>>(
    images: impl IntoIterator<Item = T>,
) -> eframe::Result {
    let images: Vec<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> = images
        .into_iter()
        .map(|f| f.into_iter().collect())
        .collect();
    let first_msg = images
        .first()
        .expect("At least one image is excepted.")
        .first()
        .expect("At least one frame is excepted.");
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
    texture: Arc<Mutex<TextureHandle>>,
    image_height: f32,
    image_width: f32,

    /// In case of a gif
    images_series: Option<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>>,
}

#[derive(Clone)]
struct ViewPortData {
    show_viewport: Arc<AtomicBool>,
    last_shown_image_index: Arc<AtomicUsize>,
    instant: Arc<AtomicInstant>,
}

struct MyApp {
    textures: Vec<ImageTextureInfo>,

    /// Data for viewports
    viewports_data: HashMap<Uuid, ViewPortData>,
}

impl MyApp {
    fn new(cc: &CreationContext, images: Vec<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>>) -> Self {
        let textures: Vec<_> = images
            .into_iter()
            .map(|image| {
                let first_image = image.first().expect("At least one frame is expected");
                ImageTextureInfo {
                    image_id: Uuid::new_v4(),
                    texture: Arc::new(
                        cc.egui_ctx
                            .load_texture(
                                "screen",
                                ColorImage::from_rgba_unmultiplied(
                                    [first_image.width() as usize, first_image.height() as usize],
                                    &first_image.clone().into_raw(),
                                ),
                                TextureOptions::default(),
                            )
                            .into(),
                    ),
                    image_height: first_image.height() as f32,
                    image_width: first_image.width() as f32,
                    images_series: Some(image),
                }
            })
            .collect();

        Self {
            viewports_data: textures
                .iter()
                .map(|f| {
                    (
                        f.image_id,
                        ViewPortData {
                            show_viewport: Arc::new(true.into()),
                            last_shown_image_index: Arc::new(0.into()),
                            instant: Arc::new(AtomicInstant::now()),
                        },
                    )
                })
                .collect(),
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

                if let Some(images_series) = first_texture_info.images_series {
                    ui.add(
                        egui::Image::new(&*first_texture_info.texture.lock().unwrap())
                            .max_height(first_texture_info.image_height)
                            .max_width(first_texture_info.image_width),
                    );

                    let view_data = &self.viewports_data[&first_texture_info.image_id];

                    let last_shown_image_index =
                        view_data.last_shown_image_index.load(Ordering::Relaxed);

                    if view_data.instant.load(Ordering::Relaxed).elapsed()
                        >= Duration::from_millis(100)
                    {
                        first_texture_info.texture.lock().unwrap().set(
                            ColorImage::from_rgba_unmultiplied(
                                [
                                    images_series[last_shown_image_index].width() as usize,
                                    images_series[last_shown_image_index].height() as usize,
                                ],
                                &images_series[last_shown_image_index].clone().into_raw(),
                            ),
                            TextureOptions::default(),
                        );

                        view_data.last_shown_image_index.store(
                            if last_shown_image_index + 1 >= images_series.len() {
                                0
                            } else {
                                last_shown_image_index + 1
                            },
                            Ordering::Relaxed,
                        );
                        view_data.instant.store(Instant::now(), Ordering::Relaxed);
                    }
                    ctx.request_repaint();
                } else {
                    ui.add(
                        egui::Image::new(&*first_texture_info.texture.lock().unwrap())
                            .max_height(first_texture_info.image_height)
                            .max_width(first_texture_info.image_width),
                    );
                }

                for texture_info in textures_info_iter {
                    if self.viewports_data[&texture_info.image_id]
                        .show_viewport
                        .load(Ordering::Relaxed)
                    {
                        let view_data = self.viewports_data[&texture_info.image_id].clone();
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
                                        let view_data = view_data.clone();
                                        if let Some(images_series) = &texture_info.images_series {
                                            ui.add(
                                                egui::Image::new(
                                                    &*texture_info.texture.lock().unwrap(),
                                                )
                                                .max_height(texture_info.image_height)
                                                .max_width(texture_info.image_width),
                                            );

                                            let last_shown_image_index = view_data
                                                .last_shown_image_index
                                                .load(Ordering::Relaxed);

                                            if view_data.instant.load(Ordering::Relaxed).elapsed()
                                                >= Duration::from_millis(100)
                                            {
                                                texture_info.texture.lock().unwrap().set(
                                                    ColorImage::from_rgba_unmultiplied(
                                                        [
                                                            images_series[last_shown_image_index]
                                                                .width()
                                                                as usize,
                                                            images_series[last_shown_image_index]
                                                                .height()
                                                                as usize,
                                                        ],
                                                        &images_series[last_shown_image_index]
                                                            .clone()
                                                            .into_raw(),
                                                    ),
                                                    TextureOptions::default(),
                                                );

                                                view_data.last_shown_image_index.store(
                                                    if last_shown_image_index + 1
                                                        >= images_series.len()
                                                    {
                                                        0
                                                    } else {
                                                        last_shown_image_index + 1
                                                    },
                                                    Ordering::Relaxed,
                                                );
                                                view_data
                                                    .instant
                                                    .store(Instant::now(), Ordering::Relaxed);
                                            }
                                            ctx.request_repaint();
                                        } else {
                                            ui.add(
                                                egui::Image::new(
                                                    &*texture_info.texture.lock().unwrap(),
                                                )
                                                .max_height(texture_info.image_height)
                                                .max_width(texture_info.image_width),
                                            );
                                        }
                                    });
                                });

                                if ctx.input(|i| i.viewport().close_requested()) {
                                    // Tell parent to close us.
                                    view_data.show_viewport.store(false, Ordering::Relaxed);
                                }
                            },
                        );
                    }
                }
            });
        });
    }
}
