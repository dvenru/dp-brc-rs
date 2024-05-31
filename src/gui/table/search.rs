use eframe::egui::{Grid, TextEdit, Ui};
use super::{date::*, TableStates};

#[derive(PartialEq)]
pub enum TableSort {
    All,
    OnlyWithoutCount,
    OnlyCount
}

pub struct Search {
    pub string: String,
    pub sort: TableSort,
    pub date_is_active: bool,
    pub date: (DatePicker, DatePicker),
}

impl Search {
    pub fn new() -> Self {
        Search {
            string: String::new(),
            sort: TableSort::All,
            date_is_active: false,
            date: (
                DatePicker::new(),
                DatePicker::new()
            )
        }
    }

    pub fn update(&mut self, ui: &mut Ui, table_state: &TableStates) {
        ui.add(TextEdit::singleline(&mut self.string).hint_text("Поиск"));

        ui.menu_button("Сортировка", |ui| {
            ui.selectable_value(&mut self.sort, TableSort::All, "Все");
            ui.selectable_value(&mut self.sort, TableSort::OnlyCount, "С количеством");
            ui.selectable_value(&mut self.sort, TableSort::OnlyWithoutCount, "Без количества");

            if let TableStates::History = table_state {
                ui.separator();

                ui.checkbox(&mut self.date_is_active, "Сортировать по дате:");

                Grid::new("grid_search_date")
                    .num_columns(2)
                    .min_col_width(10.0)
                    .show(ui, |ui| {
                        ui.label("C");
                        ui.horizontal(|ui| {
                            self.date.0.update(ui, self.date_is_active);
                        });
                        ui.end_row();

                        ui.label("По");
                        ui.horizontal(|ui| {
                            self.date.1.update(ui, self.date_is_active);
                        });
                        ui.end_row();
                    });

            }
        });
    }
}
