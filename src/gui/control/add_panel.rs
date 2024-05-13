use eframe::egui::{show_tooltip_at_pointer, Button, DragValue, Grid, Id, RichText, TextEdit, Ui};

use crate::BarCode;

use super::{Events, RemoveMultiple, BarAppEvents, Element, BarCodeData};

use rand::distributions::{Distribution, Uniform};

pub struct ControlPanelAdd {
    pub edit_name: String,
    pub edit_count: u32,
    pub edit_location: String,
    pub edit_brcode: String,
    is_error_name: bool
}

impl ControlPanelAdd {
    pub fn new() -> Self {
        ControlPanelAdd {
            edit_name: "".to_string(),
            edit_count: 0,
            edit_location: "".to_string(),
            edit_brcode: "".to_string(),
            is_error_name: false
        }
    }

    fn check_edits(&self) -> bool {
        self.edit_name.trim().len() > 0 &&
        self.edit_location.trim().len() > 0 &&
        self.edit_brcode.len() > 0
    }
}

impl Element for ControlPanelAdd {
    fn update(&mut self, ui: &mut Ui, events: &mut Events) {
        Grid::new("grid_add")
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
                                    .clamp_range(0..=9999)
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
                ui.horizontal(|ui| {
                    ui.columns(2, |cols| {
                        cols[0].vertical_centered_justified(|ui| {
                            if ui.button("Создать").clicked() {
                                let bar = BarCode::from_str(
                                    "2".to_string() + &Uniform::new_inclusive(0, 9)
                                        .sample_iter(rand::thread_rng())
                                        .take(11)
                                        .map(|c| c.to_string())
                                        .collect::<String>()
                                ).unwrap();

                                self.edit_brcode = bar.get_str() + &bar.get_checksum().to_string()
                            }
                        });
                        cols[1].vertical_centered_justified(|ui| {
                            if ui.button("Считать").clicked() {

                            }
                        });
                    });
                });

                ui.end_row();
            });

        ui.add_space(10.0);
        ui.add(
            TextEdit::singleline(&mut self.edit_brcode)
                .char_limit(13)
                .interactive(false)
                .horizontal_align(eframe::egui::Align::Center)
        );

        events.push(BarAppEvents::CheckNameItem(self.edit_name.clone()));

        ui.add_space(10.0);
        ui.vertical_centered_justified(|ui| {
            let res = ui.add_enabled(
                self.check_edits() && !self.is_error_name,
                Button::new("Добавить")
            );

            if !res.enabled() && res.contains_pointer() {
                show_tooltip_at_pointer(ui.ctx(), Id::new("add_button_data_check"), |ui| {
                    if !self.check_edits() {
                        ui.label(RichText::new("Не все поля заполнены!"));
                    }
                    if self.is_error_name {
                        ui.label(RichText::new("Элемент с таким именем уже существует!"));
                    }
                });
            }

            if res.clicked() {
                events.push(
                    BarAppEvents::AddItem(
                        BarCodeData {
                            name: self.edit_name.clone(),
                            count: self.edit_count,
                            storage_location: self.edit_location.clone(),
                            brcode: self.edit_brcode.clone()
                        }
                    )
                );

                self.edit_name = "".to_string();
                self.edit_count = 0;
                self.edit_location = "".to_string();
                self.edit_brcode = "".to_string();
            }
        });

        self.events_handler(events);
    }

    fn events_handler(&mut self, events: &mut Events) {
        self.is_error_name = false;
        let mut read_events = Vec::<usize>::new();

        for (idx, event) in events.iter().enumerate() {
            match event {
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