use eframe::egui::{Ui, TextEdit};

use crate::gui::{Element, Events};

#[derive(PartialEq)]
pub enum TableSort {
    All,
    OnlyWithoutCount,
    OnlyCount
}

pub struct Search {
    pub string: String,
    pub sort: TableSort
}

impl Search {
    pub fn new() -> Self {
        Search {
            string: String::new(),
            sort: TableSort::All
        }
    }
}

impl Element for Search {
    fn update(&mut self, ui: &mut Ui, _events: &mut Events) {
        ui.add(TextEdit::singleline(&mut self.string).hint_text("Поиск"));

        ui.menu_button("Сортировка", |ui| {
            ui.selectable_value(&mut self.sort, TableSort::All, "Все");
            ui.selectable_value(&mut self.sort, TableSort::OnlyCount, "С количеством");
            ui.selectable_value(&mut self.sort, TableSort::OnlyWithoutCount, "Без количества");
        });
    }
}