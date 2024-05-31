use eframe::egui::{SelectableLabel, Ui};

pub mod add_panel;
pub mod update_panel;

use super::{events::*, BarAppEvents, Element, BarCodeData};
use add_panel::*;
use update_panel::*;

#[derive(PartialEq)]
pub enum ControlPanelState {
    Add,
    Update,
}

pub struct PanelEdits {
    pub name: String,
    pub count: u32,
    pub location: String,
    pub brcode: String,
}

impl PanelEdits {
    pub fn new() -> Self {
        PanelEdits {
            name: String::new(),
            count: 0,
            location: String::new(),
            brcode: String::new()
        }
    }

    pub fn check(&self) -> bool {
        self.name.trim().len() > 0 &&
        self.location.trim().len() > 0 &&
        self.brcode.len() > 0
    }
}

impl From<&PanelEdits> for BarCodeData {
    fn from(value: &PanelEdits) -> Self {
        BarCodeData {
            name: value.name.clone(),
            count: value.count,
            storage_location: value.location.clone(),
            brcode: value.brcode.clone()
        }
    }
}

pub struct ControlPanel {
    pub state: ControlPanelState,
    pub panel_add: ControlPanelAdd,
    pub panel_update: ControlPanelUpdate,
    
}

impl ControlPanel {
    pub fn new() -> Self {
        ControlPanel {
            state: ControlPanelState::Add,
            panel_add: ControlPanelAdd::new(),
            panel_update: ControlPanelUpdate::new()
        }
    }
}

impl Element for ControlPanel {
    fn update(&mut self, ui: &mut Ui, events: &mut Events) {
        ui.horizontal(|ui| {
            ui.columns(2, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    ui.selectable_value(&mut self.state, ControlPanelState::Add, "Добавить");
                });
                cols[1].vertical_centered_justified(|ui| {
                    let res = ui.add_enabled(
                        self.panel_update.is_active,
                        SelectableLabel::new(self.state == ControlPanelState::Update, "Сведения")
                    );

                    if res.clicked() {
                        self.state = ControlPanelState::Update;
                    }
                });
            });
        });

        ui.add_space(20.0);

        match self.state {
            ControlPanelState::Add => {
                self.panel_add.update(ui, events);
            }
            ControlPanelState::Update => {
                self.panel_update.update(ui, events);
            }
        }

        self.events_handler(events);
    }

    fn events_handler(&mut self, events: &mut Events) {
        let mut read_events = Vec::<usize>::new();

        for (idx, event) in events.iter().enumerate() {
            match event {
                BarAppEvents::SwitchTabToUpdate => {
                    self.state = ControlPanelState::Update;

                    read_events.push(idx);
                }
                _ => {}
            }
        }

        events.remove_multiple(read_events);
    }
}