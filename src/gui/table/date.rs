use chrono::prelude::*;
use eframe::egui::{DragValue, Ui};

pub struct DatePicker {
    day: u32,
    month: u32,
    year: i32
}

impl From<&DatePicker> for NaiveDate {
    fn from(value: &DatePicker) -> Self {
        NaiveDate::from_ymd_opt(
            value.year,
            value.month,
            {
                if value.day > value.month_lenght() {
                    value.month_lenght()
                } else {
                    value.day
                }
            }
        ).unwrap()
    }
}

impl DatePicker {
    pub fn new() -> Self {
        let now = Utc::now();

        DatePicker {
            day: now.day(),
            month: now.month(),
            year: now.year()
        }
    }

    pub fn update(&mut self, ui: &mut Ui, is_active: bool) {
        let max_days = self.month_lenght();

        ui.add_enabled(is_active, DragValue::new(&mut self.day).clamp_range(1..=max_days).speed(0.1).suffix(" д"));
        ui.add_enabled(is_active, DragValue::new(&mut self.month).clamp_range(1..=12).speed(0.05).suffix(" м"));
        ui.add_enabled(is_active, DragValue::new(&mut self.year).clamp_range(0..=9999).speed(0.01).suffix(" г"));
    }

    fn month_lenght(&self) -> u32 {
        if self.month == 12 {
            NaiveDate::from_ymd_opt(self.year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(self.year, self.month + 1, 1).unwrap()
        }.signed_duration_since(NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap())
            .num_days() as u32
    }
}