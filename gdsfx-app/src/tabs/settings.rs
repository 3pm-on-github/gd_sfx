use eframe::egui::{Ui, ComboBox};
use strum::IntoEnumIterator;

use crate::{app_state::AppState, i18n::LocalizedEnum};

pub fn render(ui: &mut Ui, app_state: &mut AppState) {
    ui.heading(t!("settings"));
    
    ui.add_space(10.0);
    set_enum_setting(ui, &mut app_state.settings.search_filter_mode);
    
    ui.add_space(10.0);
    set_enum_setting(ui, &mut app_state.settings.sfx_select_mode);
    ui.checkbox(&mut app_state.settings.play_sfx_on_click, t!("settings.play_sfx_on_click"));
    
    ui.add_space(10.0);
    set_locale(ui, app_state);

    ui.add_space(10.0);
    ui.text_edit_singleline(&mut app_state.settings.gd_folder);

    let _ = app_state.settings.try_save_if_changed();
}

fn set_enum_setting<T>(ui: &mut Ui, selected: &mut T)
where
    T: LocalizedEnum + IntoEnumIterator + PartialEq + Copy
{
    ComboBox::from_label(T::localize_enum())
        .selected_text(selected.localize_variant())
        .show_ui(ui, |ui| {
            for mode in T::iter() {
                ui.selectable_value(selected, mode, mode.localize_variant());
            }
        });
}

fn set_locale(ui: &mut Ui, app_state: &mut AppState) {
    ComboBox::from_label(t!("settings.language"))
        .selected_text(t!("language.name"))
        .show_ui(ui, |ui| {
            for locale in rust_i18n::available_locales!() {
                ui.selectable_value(
                    &mut app_state.settings.locale,
                    locale.to_string(), t!("language.name", locale = locale)
                );
            }
        });

    rust_i18n::set_locale(&app_state.settings.locale);
}
