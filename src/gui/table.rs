use eframe::egui::{self, RichText, TextEdit, Ui};
use egui_extras::{Column, TableBuilder};

use super::{BarCodeData, BarCodeHistoryData};
use super::{Events, BarAppEvents, Element};

#[derive(PartialEq)]
pub enum TableStates {
    Data,
    History
}

pub struct TableCell {
    pub data: String
}

impl Into<TableCell> for String {
    fn into(self) -> TableCell {
        TableCell { data: self }
    }
}

pub struct TableRow {
    pub data: Vec<TableCell>
}

impl TableRow {
    pub fn draw(&self, row: &mut egui_extras::TableRow, num: usize) {
        row.col(|ui| {
            ui.label(RichText::new(num.to_string()));
        });

        for col in self.data.iter() {
            row.col(|ui| {
                ui.label(RichText::new(col.data.clone()));
            });
        }
    }
}

impl From<BarCodeData> for TableRow {
    fn from(value: BarCodeData) -> Self {
        TableRow { data: vec![
            TableCell { data: value.name },
            TableCell { data: value.count.to_string() },
            TableCell { data: value.storage_location },
            TableCell { data: value.brcode }
        ] }
    }
}

impl From<BarCodeHistoryData> for TableRow {
    fn from(value: BarCodeHistoryData) -> Self {
        TableRow { data: vec![
            TableCell { data: value.name },
            TableCell { data: value.count.to_string() },
            TableCell { data: value.storage_location },
            TableCell { data: value.brcode },
            TableCell { data: value.time_change }
        ] }
    }
}

impl From<&TableRow> for BarCodeData {
    fn from(value: &TableRow) -> Self {
        BarCodeData {
            name: value.data[0].data.clone(),
            count: value.data[1].data.parse::<u32>().unwrap(),
            storage_location: value.data[2].data.clone(),
            brcode: value.data[3].data.clone()
        }
    }
}

pub struct Table {
    header: TableRow,
    rows: Vec<TableRow>,
    selected: Option<usize>,
    search_string: String,
    pub state: TableStates,
    only_quantity: bool
}

impl Table {
    pub fn new() -> Self {
        Table {
            header: TableRow {
                data: Vec::new()
            },
            rows: Vec::new(),
            selected: None,
            search_string: "".to_string(),
            state: TableStates::Data,
            only_quantity: false
        }
    }

    pub fn show_data(&mut self, dt: Vec<BarCodeData>) {
        self.rows.clear();
        self.header.data = vec![
            "Название".to_string().into(),
            "Количество".to_string().into(),
            "Хранение".to_string().into(),
            "Штрих-код".to_string().into()
        ];

        for bar in dt {
            self.rows.push(
                TableRow::from(bar)
            );
        }
    }

    pub fn show_history(&mut self, dt: Vec<BarCodeHistoryData>) {
        self.rows.clear();
        self.header.data = vec![
            "Название".to_string().into(),
            "Количество".to_string().into(),
            "Хранение".to_string().into(),
            "Штрих-код".to_string().into(),
            "Дата изменения".to_string().into()
        ];

        for bar in dt {
            self.rows.push(
                TableRow::from(bar)
            );
        }
    }
}

impl Element for Table {
    fn update(&mut self, ui: &mut Ui, events: &mut Events) {
        ui.horizontal(|ui| {
            
            if ui.selectable_value(&mut self.state, TableStates::Data, "Таблица данных").clicked() {
                events.push(BarAppEvents::ShowItems);
            };
            
            if ui.selectable_value(&mut self.state, TableStates::History, "История данных").clicked() {
                events.push(BarAppEvents::ShowHistory(
                    None
                ));
            };

            ui.separator();
            ui.add(TextEdit::singleline(&mut self.search_string).hint_text("Поиск"));
            // ui.checkbox(&mut self.only_quantity, "Только с количеством");
        });

        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .columns(Column::initial(120.0).range(50.0..=300.0).clip(true), self.header.data.len() - 1)
            .column(Column::remainder())
            .sense(egui::Sense::click())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        table.header(30.0, |mut header| {
            header.col(|ui| {
                ui.strong("№");
            });

            for head in self.header.data.iter() {
                header.col(|ui| {
                    ui.strong(head.data.clone());
                });
            }

        }).body(|mut body| {
            for (num, table_row) in self.rows.iter().enumerate() {
                body.row(25.0, |mut row| {
                    if let TableStates::Data = self.state {
                        match self.selected {
                            Some(n) if n == num => row.set_selected(true),
                            _ => row.set_selected(false)
                        };
                    };
                    
                    if !self.search_string.is_empty() {
                        for col in table_row.data.iter() {
                            if col.data.contains(&self.search_string) {
                                table_row.draw(&mut row, num + 1);

                                if let TableStates::Data = self.state {
                                    let res = &row.response();
                                    if res.clicked() {
                                        self.selected = Some(num);
    
                                        events.push(BarAppEvents::ItemSelected(
                                            BarCodeData::from(&self.rows[self.selected.unwrap()])
                                        ));
                                        events.push(BarAppEvents::SwitchTabToUpdate);
                                    }
                                }

                                break;
                            }
                        }
                    } else {
                        table_row.draw(&mut row, num + 1);

                        if let TableStates::Data = self.state {
                            let res = &row.response();
                            if res.clicked() {
                                self.selected = Some(num);
    
                                events.push(BarAppEvents::ItemSelected(
                                    BarCodeData::from(&self.rows[self.selected.unwrap()])
                                ));
                                events.push(BarAppEvents::SwitchTabToUpdate);
                            }
                        }
                    }
                });
            }
        });

        self.events_handler(events);
    }
}
