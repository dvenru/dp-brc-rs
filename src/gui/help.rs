use eframe::egui::{text::{LayoutJob, TextWrapping}, Color32, FontFamily, FontId, Label, RichText, TextFormat, Ui, Visuals};

#[derive(PartialEq)]
pub enum AppThemes {
    Dark,
    Light
}

pub struct HelpMenu {
    pub theme: AppThemes
}

impl HelpMenu {
    pub fn new() -> Self {
        HelpMenu { theme: AppThemes::Dark }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        ui.menu_button("Помощь", |ui| {
            ui.vertical(|ui| {
                ui.add_sized([350.0, 20.0], Label::new("Справка по использованию:"));
            });

            let text = LayoutJob::single_section(
                "Для получения штрих-кода товара, необходимо сначало добавить его в таблицу. Данные вводятся в панели слева, все поля являются обязательными, при этом название должно быть уникальным. После заполнения всех полей, нажимаете на кнопку добавить, после чего новая запись появится в таблице справа. Для сохранения изображения штрих-кода, нужно выбрать запись в таблице (записи из таблицы с историей выбирать нельзя), после чего нажать на кнопку 'Сохранить как...', где необходимо будет выбрать название изображения и его местоположение. Также в панели свойств можно обновить данные о товаре, при это есть возможность увидеть полную историю изменений.".to_owned(),
                TextFormat {
                    line_height: Some(18.0),
                    font_id: FontId {
                        family: FontFamily::Proportional,
                        size: 12.0
                    },
                    ..Default::default()
                }
            );

            ui.add_sized([350.0, 200.0], Label::new(text));

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Цветовая тема:");

                if ui.selectable_value(&mut self.theme, AppThemes::Dark, "Темная").clicked() {
                    let ctx = ui.ctx();
                    let mut visual_mode = Visuals::dark();
    
                    visual_mode.override_text_color = Some(Color32::WHITE);
                    ctx.set_visuals(visual_mode);
                }
    
                if ui.selectable_value(&mut self.theme, AppThemes::Light, "Светлая").clicked() {
                    let ctx = ui.ctx();
                    let mut visual_mode = Visuals::light();
    
                    visual_mode.override_text_color = Some(Color32::BLACK);
                    ctx.set_visuals(visual_mode);
                }
            });

        });
    }
}