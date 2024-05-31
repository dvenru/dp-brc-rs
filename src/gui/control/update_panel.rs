use eframe::egui::{DragValue, Grid, RichText, TextEdit, Ui, Button, show_tooltip_at_pointer, Id};
use egui_file_dialog::{DialogState, FileDialog};
use std::path::PathBuf;

use crate::{image::Svg, BarCode};
use super::{BarAppEvents, BarCodeData, Element, Events, PanelEdits, RemoveMultiple};

struct SaveFile {
    dialog: FileDialog,
    path: Option<PathBuf>
}

pub struct ControlPanelUpdate {
    pub origin_name: String,
    pub edit: PanelEdits,
    pub is_active: bool,
    is_error_name: bool,
    file: SaveFile
}

impl ControlPanelUpdate {
    pub fn new() -> Self {
        ControlPanelUpdate {
            origin_name: String::new(),
            edit: PanelEdits::new(),
            is_active: false,
            is_error_name: false,
            file: SaveFile {
                dialog: FileDialog::new().title("Сохранить как..."),
                path: None
            }
        }
    }
}

impl Element for ControlPanelUpdate {
    fn update(&mut self, ui: &mut Ui, events: &mut Events) {
        Grid::new("grid_update")
                .num_columns(2)
                .spacing([10.0, 12.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Название:"));
                    ui.text_edit_singleline(&mut self.edit.name);
                    ui.end_row();
    
                    ui.label(RichText::new("Количество:"));
                    ui.horizontal(|ui| {
                        ui.columns(3, |cols| {

                            cols[0].vertical_centered_justified(|ui| {
                                if ui.button("+").clicked() {
                                    self.edit.count += 1;
                                }
                            });

                            cols[1].vertical_centered_justified(|ui| {
                                ui.add(
                                    DragValue::new(&mut self.edit.count)
                                        .clamp_range(0..=999)
                                        .speed(0.2)
                                );
                            });

                            cols[2].vertical_centered_justified(|ui| {
                                if ui.button("-").clicked() {
                                    self.edit.count = self.edit.count.saturating_sub(1);
                                }
                            });

                        });
                    });
                    ui.end_row();
    
                    ui.label(RichText::new("Нахождение:"));
                    ui.text_edit_singleline(&mut self.edit.location);
                    ui.end_row();
    
                    ui.label(RichText::new("Штрих-код:"));
                    ui.add(
                        TextEdit::singleline(&mut self.edit.brcode)
                            .char_limit(13)
                            .interactive(false)
                            .horizontal_align(eframe::egui::Align::Center)
                    );
                    ui.end_row();
                });
            
            events.push(BarAppEvents::CheckNameItem(self.edit.name.clone()));

            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                let res = ui.add_enabled(
                    self.edit.check() && 
                    (!self.is_error_name || self.edit.name == self.origin_name),
                    Button::new(RichText::new("Обновить"))
                );
                
                if !res.enabled() && res.contains_pointer() {
                    show_tooltip_at_pointer(ui.ctx(), Id::new("add_button_data_check"), |ui| {
                        if !self.edit.check() {
                            ui.label(RichText::new("Не все поля заполнены!"));
                        }
                        if self.is_error_name && self.edit.name != self.origin_name {
                            ui.label(RichText::new("Элемент с таким именем уже существует!"));
                        }
                    });
                }

                if res.clicked() {
                    events.push(BarAppEvents::UpdateItem(BarCodeData::from(&self.edit)));
                }

                if ui.button(RichText::new("История изменений")).clicked() {
                    events.push(BarAppEvents::ShowHistory(
                        Some(BarCodeData::from(&self.edit))
                    ));
                }
                
                if ui.button(RichText::new("Сохранить Штрих-код...")).clicked() {
                    self.file.dialog.save_file();
                }

                if let DialogState::Open = self.file.dialog.state() {
                    if let Some(path) = self.file.dialog.update(ui.ctx()).selected() {
                        self.file.path = Some(path.to_path_buf());

                        let svg = Svg::new(200, 7);
                        let encode = svg.generate(BarCode::from_str(self.edit.brcode.clone()).unwrap().encode()).unwrap();

                        svg.save_to(encode, self.file.path.as_mut().unwrap()).unwrap();
                    }
                }
            });

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
                    self.edit.name = data.name.clone();
                    self.edit.count = data.count;
                    self.edit.location = data.storage_location.clone();
                    self.edit.brcode = data.brcode.clone();

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