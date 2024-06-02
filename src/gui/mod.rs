use eframe::{Frame, egui::{self, Context}};
use std::fs;

use crate::ORIGIN_WIDTH;

use super::data_controller::*;

pub mod table;
pub mod control;
pub mod events;
pub mod help;

use events::*;
use help::*;
use table::Table;
use control::ControlPanel;

pub struct BarApp {
    events: Events,
    table: Table,
    control_panel: ControlPanel,
    help: HelpMenu
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
            help: HelpMenu::new()
        }
    }
}

impl eframe::App for BarApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
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
                            match self.help.theme {
                                AppThemes::Dark => { ui.add(egui::Image::new(egui::include_image!("../../assets/logo_white.png"))); }
                                AppThemes::Light => { ui.add(egui::Image::new(egui::include_image!("../../assets/logo_dark.png"))); }
                            }

                            ui.add_space(110.0);

                            self.help.update(ui);
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