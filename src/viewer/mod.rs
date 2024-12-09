#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;

use eframe::egui::{IconData, ImageData};
use eframe::{
    egui::{self, Color32, ColorImage, TextureHandle, TextureOptions},
    CreationContext,
};
use image::{ImageBuffer, Rgba};

use crate::pixels::canvas::templates::alien_monster::AlienMonster;
use crate::pixels::canvas::{SharedMutPixelCanvasExt, SharedPixelCanvasExt};
use crate::prelude::{MaybePixel, PixelCanvas};

pub type ViewResult = eframe::Result;

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

pub(crate) fn view(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_icon(get_icon())
            .with_inner_size([(image.width() as f32 + 20.), (image.height() as f32 + 20.)]),
        ..Default::default()
    };
    eframe::run_native(
        "Pixelart",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::<MyApp>::new(MyApp::new(cc, image)))
        }),
    )
}

struct MyApp {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    screen_texture: TextureHandle,
}

impl MyApp {
    fn new(cc: &CreationContext, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        let screen_texture = cc.egui_ctx.load_texture(
            "screen",
            ImageData::Color(Arc::new(ColorImage::new(
                [image.width() as usize, image.height() as usize],
                Color32::TRANSPARENT,
            ))),
            TextureOptions::default(),
        );
        Self {
            screen_texture,
            image,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                // This should obviously not be here, but it's just a test
                self.screen_texture.set(
                    ColorImage::from_rgba_unmultiplied(
                        [self.image.width() as usize, self.image.height() as usize],
                        &self.image.clone().into_raw(),
                    ),
                    TextureOptions::default(),
                );
                ui.add(
                    egui::Image::new(&self.screen_texture) // ERROR GONE
                        .max_height(self.image.height() as f32)
                        .max_width(self.image.width() as f32),
                );
            });
        });
    }
}
