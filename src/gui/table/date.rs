use chrono::prelude::*;
use eframe::egui::{DragValue, Ui};

trait MonthLenght {
    fn month_lenght(year: i32, month: u32) -> u32;
}

impl MonthLenght for Month {
    fn month_lenght(year: i32, month: u32) -> u32 {
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
        }.signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
            .num_days() as u32
    }
}

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
                let max_days = Month::month_lenght(value.year, value.month);

                if value.day > max_days {
                    max_days
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
        let max_days = Month::month_lenght(self.year, self.month);

        ui.add_enabled(is_active, DragValue::new(&mut self.day).clamp_range(1..=max_days).speed(0.1).suffix(" д"));
        ui.add_enabled(is_active, DragValue::new(&mut self.month).clamp_range(1..=12).speed(0.05).suffix(" м"));
        ui.add_enabled(is_active, DragValue::new(&mut self.year).clamp_range(0..=9999).speed(0.01).suffix(" г"));
    }
}