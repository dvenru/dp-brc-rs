use eframe::egui::{Color32, DragValue, Grid, RichText, TextEdit, Ui, Button, show_tooltip_at_pointer, Id};
use egui_file_dialog::{DialogState, FileDialog};
use std::path::PathBuf;

use crate::{image::Svg, BarCode};

use super::{Events, RemoveMultiple, BarAppEvents, Element, BarCodeData};

pub struct ControlPanelUpdate {
    pub origin_name: String,
    pub edit_name: String,
    pub edit_count: u32,
    pub edit_location: String,
    pub edit_brcode: String,
    is_active: bool,
    is_error_name: bool,
    file_dialog: FileDialog,
    file_path: Option<PathBuf>
}

impl ControlPanelUpdate {
    pub fn new() -> Self {
        ControlPanelUpdate {
            origin_name: "".to_string(),
            edit_name: "".to_string(),
            edit_count: 0,
            edit_location: "".to_string(),
            edit_brcode: "".to_string(),
            is_active: false,
            is_error_name: false,
            file_dialog: FileDialog::new().title("Сохранить как..."),
            file_path: None
        }
    }

    fn check_edits(&self) -> bool {
        self.edit_name.trim().len() > 0 &&
        self.edit_location.trim().len() > 0 &&
        self.edit_brcode.len() > 0
    }
}

impl Element for ControlPanelUpdate {
    fn update(&mut self, ui: &mut Ui, events: &mut Events) {
        if self.is_active {
            Grid::new("grid_update")
                .num_columns(2)
                .spacing([10.0, 12.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Название:"));
                    ui.text_edit_singleline(&mut self.edit_name);
                    ui.end_row();
    
                    ui.label(RichText::new("Количество:"));
                    ui.horizontal(|ui| {
                        ui.columns(3, |cols| {
                            cols[0].vertical_centered_justified(|ui| {
                                if ui.button("+").clicked() {
                                    self.edit_count += 1;
                                }
                            });
                            cols[1].vertical_centered_justified(|ui| {
                                ui.add(
                                    DragValue::new(&mut self.edit_count)
                                        .clamp_range(0..=999)
                                        .speed(0.2)
                                );
                            });
                            cols[2].vertical_centered_justified(|ui| {
                                if ui.button("-").clicked() {
                                    self.edit_count = self.edit_count.saturating_sub(1);
                                }
                            });
                        });
                    });
                    ui.end_row();
    
                    ui.label(RichText::new("Нахождение:"));
                    ui.text_edit_singleline(&mut self.edit_location);
                    ui.end_row();
    
                    ui.label(RichText::new("Штрих-код:"));
                    ui.add(
                        TextEdit::singleline(&mut self.edit_brcode)
                            .char_limit(13)
                            .interactive(false)
                            .horizontal_align(eframe::egui::Align::Center)
                    );
                    ui.end_row();
                });
            
            events.push(BarAppEvents::CheckNameItem(self.edit_name.clone()));

            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                let res = ui.add_enabled(
                    self.check_edits() && 
                    (!self.is_error_name || self.edit_name == self.origin_name),
                    Button::new(RichText::new("Обновить"))
                );
                
                if !res.enabled() && res.contains_pointer() {
                    show_tooltip_at_pointer(ui.ctx(), Id::new("add_button_data_check"), |ui| {
                        if !self.check_edits() {
                            ui.label(RichText::new("Не все поля заполнены!"));
                        }
                        if self.is_error_name && self.edit_name != self.origin_name {
                            ui.label(RichText::new("Элемент с таким именем уже существует!"));
                        }
                    });
                }

                if res.clicked() {
                    events.push(BarAppEvents::UpdateItem(
                        BarCodeData {
                            name: self.edit_name.clone(),
                            count: self.edit_count,
                            storage_location: self.edit_location.clone(),
                            brcode: self.edit_brcode.clone()
                        }
                    ));
                }

                if ui.button(RichText::new("История изменений")).clicked() {
                    events.push(BarAppEvents::ShowHistory(
                        Some(
                            BarCodeData {
                                name: self.edit_name.clone(),
                                count: self.edit_count,
                                storage_location: self.edit_location.clone(),
                                brcode: self.edit_brcode.clone()
                            }
                        )
                    ));
                }
                
                if ui.button(RichText::new("Сохранить Штрих-код...")).clicked() {
                    self.file_dialog.save_file();
                }

                if let DialogState::Open = self.file_dialog.state() {
                    if let Some(path) = self.file_dialog.update(ui.ctx()).selected() {
                        self.file_path = Some(path.to_path_buf());
                        let svg = Svg::new(200, 7);
                        let encode = svg.generate(BarCode::from_str(self.edit_brcode.clone()).unwrap().encode()).unwrap();
                        svg.save_to(encode, self.file_path.as_mut().unwrap()).unwrap();
                    }
                }
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Элемент не выбран").color(Color32::RED));
            });
        }

        self.events_handler(events);
    }

    fn events_handler(&mut self, events: &mut Events) {
        self.is_error_name = false;
        let mut read_events = Vec::<usize>::new();

        for (idx, event) in events.iter().enumerate() {
            match event {
                BarAppEvents::ItemSelected(data) => {
                    self.is_active = true;

                    self.origin_name = data.name.clone();
                    self.edit_name = data.name.clone();
                    self.edit_count = data.count;
                    self.edit_location = data.storage_location.clone();
                    self.edit_brcode = data.brcode.clone();

                    read_events.push(idx)
                }
                BarAppEvents::ErrorNameItem => {
                    self.is_error_name = true;

                    read_events.push(idx);
                }
                _ => {}
            }
        }

        events.remove_multiple(read_events);
    }

}