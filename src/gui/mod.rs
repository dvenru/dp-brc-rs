use eframe::egui::{self, Color32, Visuals};
use std::fs;

use crate::ORIGIN_WIDTH;

use super::data_controller::*;

pub mod table;
pub mod control;
pub mod events;

use events::*;
use table::Table;
use control::ControlPanel;


#[derive(PartialEq)]
pub enum AppThemeState {
    Dark,
    Light
}
pub struct BarApp {
    events: Events,
    table: Table,
    control_panel: ControlPanel,
    theme: AppThemeState
}

impl Default for BarApp {
    fn default() -> Self {
        match fs::create_dir("data") {
            Ok(_) => (),
            Err(_) => ()
        }

        match fs::create_dir("svg") {
            Ok(_) => (),
            Err(_) => ()
        }

        DataBase::new().init().unwrap();

        BarApp {
            events: Events::new(),
            table: Table::new(),
            control_panel: ControlPanel::new(),
            theme: AppThemeState::Dark
        }
    }
}

impl eframe::App for BarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let win_width = ui.available_width();
                let left = ORIGIN_WIDTH * 0.25;
                let right = win_width - left;

                egui::SidePanel::left("left")
                    .max_width(left)
                    .show_separator_line(false)
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            match self.theme {
                                AppThemeState::Dark => { ui.add(egui::Image::new(egui::include_image!("../../assets/logo_white.png"))); }
                                AppThemeState::Light => { ui.add(egui::Image::new(egui::include_image!("../../assets/logo_dark.png"))); }
                            }

                            ui.add_space(100.0);

                            ui.menu_button("Параметры", |ui| {
                                if ui.selectable_value(&mut self.theme, AppThemeState::Dark, "Темная").clicked() {
                                    let ctx = ui.ctx();
                                    let mut visual_mode = Visuals::dark();
    
                                    visual_mode.override_text_color = Some(Color32::WHITE);
                                    ctx.set_visuals(visual_mode);
                                }
    
                                if ui.selectable_value(&mut self.theme, AppThemeState::Light, "Светлая").clicked() {
                                    let ctx = ui.ctx();
                                    let mut visual_mode = Visuals::light();
    
                                    visual_mode.override_text_color = Some(Color32::BLACK);
                                    ctx.set_visuals(visual_mode);
                                }
                            });
                        });

                        ui.add_space(10.0);

                        self.control_panel.update(ui, &mut self.events);
                    });

                egui::SidePanel::right("right")
                    .min_width(right)
                    .show_separator_line(false)
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            self.table.update(ui, &mut self.events);
                        });
                    });
            });
    }
}