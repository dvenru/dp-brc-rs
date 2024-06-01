use eframe::egui::{show_tooltip_at_pointer, Button, DragValue, Grid, Id, RichText, TextEdit, Ui};
use rand::distributions::{Distribution, Uniform};

use super::{BarAppEvents, BarCodeData, Events, PanelEdits};
use crate::{data_controller::DataBase, BarCode};

pub struct ControlPanelAdd {
    edit: PanelEdits,
    name_is_unique: bool
}

impl ControlPanelAdd {
    pub fn new() -> Self {
        ControlPanelAdd {
            edit: PanelEdits::new(),
            name_is_unique: true
        }
    }

    pub fn update(&mut self, ui: &mut Ui, events: &mut Events) {
        Grid::new("grid_add")
            .num_columns(2)
            .spacing([10.0, 12.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Название:"));

                let old_name = self.edit.name.clone();
                ui.text_edit_singleline(&mut self.edit.name);
                self.check_name(old_name);

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
                                    .clamp_range(0..=9999)
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
                ui.horizontal(|ui| {
                    ui.columns(1, |cols| {
                        cols[0].vertical_centered_justified(|ui| {
                            if ui.button("Создать").clicked() {
                                let bar = BarCode::from_str(
                                    "2".to_string() + &Uniform::new_inclusive(0, 9)
                                        .sample_iter(rand::thread_rng())
                                        .take(11)
                                        .map(|c| c.to_string())
                                        .collect::<String>()
                                ).unwrap();

                                self.edit.brcode = bar.get_str() + &bar.get_checksum().to_string()
                            }
                        });
                    });
                });

                ui.end_row();
            });

        ui.add_space(10.0);
        ui.add(
            TextEdit::singleline(&mut self.edit.brcode)
                .char_limit(13)
                .interactive(false)
                .horizontal_align(eframe::egui::Align::Center)
        );

        ui.add_space(10.0);
        ui.vertical_centered_justified(|ui| {
            let res = ui.add_enabled(
                self.edit.check() && self.name_is_unique,
                Button::new("Добавить")
            );

            if !res.enabled() && res.contains_pointer() {
                show_tooltip_at_pointer(ui.ctx(), Id::new("add_button_data_check"), |ui| {
                    if !self.edit.check() {
                        ui.label(RichText::new("Не все поля заполнены!"));
                    }
                    if !self.name_is_unique {
                        ui.label(RichText::new("Элемент с таким именем уже существует!"));
                    }
                });
            }

            if res.clicked() {
                DataBase::new().append(BarCodeData::from(&self.edit)).unwrap();
                events.push(BarAppEvents::UpdateTable);

                self.edit = PanelEdits::new();
            }
        });
    }

    fn check_name(&mut self, old_name: String) {
        if old_name != self.edit.name {
            self.name_is_unique = DataBase::new().name_is_unique(self.edit.name.clone())
        }
    }
}