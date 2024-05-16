use eframe::egui::{self, Color32, RichText, Ui, Visuals};
use std::fs;

use crate::ORIGIN_WIDTH;

use self::table::TableStates;

use super::data_controller::*;

pub mod table;
pub mod control;
pub mod events;

use events::*;
use table::Table;
use control::ControlPanel;

pub trait Element {
    fn update(&mut self, _ui: &mut Ui, _events: &mut Events);

    fn events_handler(&mut self, events: &mut Events) {
        for event in events.iter() {
            match event {
                _ => {}
            }
        }
    }
}

pub struct BarApp {
    events: Events,
    data_base: DataBase,
    table: Table,
    control_panel: ControlPanel
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

        // match fs::create_dir("assets") {
        //     Ok(_) => (),
        //     Err(_) => ()
        // }

        let data_base = DataBase::new("data/barcodes.db").unwrap();
        data_base.init().unwrap();

        let table = Table::new();

        let mut bar_app = BarApp {
            events: Events::new(),
            data_base,
            table,
            control_panel: ControlPanel::new()
        };

        bar_app.table.show_data(bar_app.data_base.get_all().unwrap());
        
        bar_app
    }
}

impl eframe::App for BarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visual_mode = Visuals::dark();
        visual_mode.override_text_color = Some(Color32::WHITE);
        ctx.set_visuals(visual_mode);
        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let win_width = ui.available_width();
                let left = ORIGIN_WIDTH * 0.25;
                let right = win_width - left;

                egui::SidePanel::left("left")
                    .min_width(left)
                    .show_separator_line(false)
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        ui.label(RichText::new("BRC").heading());
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
                
                self.events_handler()
            });
    }
}

impl BarApp {
    fn events_handler(&mut self) {
        let mut read_events = Vec::<usize>::new();
        let mut returned_events = Events::new();

        for (idx, event) in self.events.iter().enumerate() {
            match event {
                BarAppEvents::AddItem(data) => {
                    self.data_base.append(data.clone()).unwrap();
                    match self.table.state {
                        TableStates::Data => self.table.show_data(self.data_base.get_all().unwrap()),
                        TableStates::History => self.table.show_history(self.data_base.get_history(Some(data.clone())).unwrap())
                    }
                    
                    read_events.push(idx);
                }
                BarAppEvents::UpdateItem(data) => {
                    self.data_base.update(data.clone()).unwrap();
                    match self.table.state {
                        TableStates::Data => self.table.show_data(self.data_base.get_all().unwrap()),
                        TableStates::History => self.table.show_history(self.data_base.get_history(Some(data.clone())).unwrap())
                    }

                    read_events.push(idx);
                }
                BarAppEvents::ShowItems => {
                    self.table.show_data(self.data_base.get_all().unwrap());
                    self.table.state = TableStates::Data;

                    read_events.push(idx);
                }
                BarAppEvents::ShowHistory(data) => {
                    self.table.show_history(self.data_base.get_history(data.clone()).unwrap());
                    self.table.state = TableStates::History;

                    read_events.push(idx);
                }
                BarAppEvents::CheckNameItem(name) => {
                    let name = name.clone().trim().to_string().to_lowercase();
                    let db_data = self.data_base.get_all().unwrap();

                    for dt in db_data.iter() {
                        if name == dt.name.to_lowercase().trim().to_string() {
                            returned_events.push(BarAppEvents::ErrorNameItem);
                            break;
                        }
                    }

                    read_events.push(idx);
                }
                _ => {}
            }
        }

        self.events.remove_multiple(read_events);
        self.events.append(&mut returned_events);
    }
}
