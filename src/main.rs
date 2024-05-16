#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod barcode;
mod data_controller;
mod gui;

use gui::BarApp;
use barcode::*;
use eframe::egui;

const ORIGIN_WIDTH: f32 = 1000.0;
const ORIGIN_HEIGHT: f32 = 600.0;

fn main() -> Result<(), eframe::Error> {
    let app_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([ORIGIN_WIDTH - 100.0, ORIGIN_HEIGHT])
            .with_inner_size([ORIGIN_WIDTH + 100.0, ORIGIN_HEIGHT])
            .with_title("BRC")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "BRC",
        app_options,
        Box::new(|ctx| {
            egui_extras::install_image_loaders(&ctx.egui_ctx);

            Box::<BarApp>::default()
        })
    )
}
