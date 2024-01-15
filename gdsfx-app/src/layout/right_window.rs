use eframe::{egui::{Context, CentralPanel, Button, Slider}, epaint::Color32};
use gdsfx_audio::AudioSettings;
use gdsfx_library::EntryKind;

use crate::backend::AppState;

// TODO can we make this less of a list of ui elements
// and instead maybe put some stuff on the right side of the screen
// also make sure everything fits on the ui
pub fn render(ctx: &Context, app_state: &mut AppState) {
    if let Some(entry) = app_state.selected_sfx.clone() {
        if let EntryKind::Sound { bytes, duration } = &entry.kind {
            CentralPanel::default().show(ctx, |ui| {
                ui.heading(&entry.name);
    
                ui.add_space(10.0);
    
                ui.code(entry.to_string());
    
                ui.add_space(10.0);
    
                ui.heading(t!("sound.info.id", id = entry.id));
                ui.heading(t!("sound.info.category.id", id = entry.parent_id));
                ui.heading(t!("sound.info.size", size = pretty_bytes::converter::convert(*bytes as f64)));
                ui.heading(t!("sound.info.duration", duration = duration));
    
                ui.add_space(25.0);

                if let Some(file_handler) = entry.create_file_handler(&app_state.settings.gd_folder) {
                    let file_exists = file_handler.file_exists();
    
                    let download_button = Button::new(t!("sound.download"));
                    if ui.add_enabled(!file_exists, download_button).clicked() {
                        app_state.download_sound(&entry);
                    }
        
                    let delete_button = Button::new(t!("sound.delete"));
                    if ui.add_enabled(file_exists, delete_button).clicked() {
                        file_handler.try_delete_file();
                    }
                } else {
                    ui.colored_label(Color32::KHAKI, t!("settings.gd_folder.not_found"));
                }
                
                ui.add_space(10.0);
    
                if ui.button(t!("sound.play")).clicked() {
                    app_state.play_sound(&entry);
                }
    
                let stop_button = Button::new(t!("sound.stop"));
                if ui.add_enabled(gdsfx_audio::is_playing_audio(), stop_button).clicked() {
                    gdsfx_audio::stop_all();
                }

                ui.add_space(10.0);

                ui.label(t!("sound.speed"));
                ui.add(Slider::new(&mut app_state.audio_settings.speed, -12.0..=12.0));
                
                ui.label(t!("sound.pitch"));
                ui.add(Slider::new(&mut app_state.audio_settings.pitch, -12.0..=12.0));

                ui.label(t!("sound.volume"));
                ui.add(Slider::new(&mut app_state.audio_settings.volume, 0.0..=2.0));

                ui.add_space(10.0);

                let reset_button = Button::new(t!("sound.reset"));
                let default_audio_settings = AudioSettings::default();
                if ui.add_enabled(app_state.audio_settings != default_audio_settings, reset_button).clicked() {
                    app_state.audio_settings = default_audio_settings;
                }

                ui.add_space(10.0);

                let favorite_button_label = match app_state.favorites.has_favorite(entry.id) {
                    false => t!("sound.favorite.add"),
                    true => t!("sound.favorite.remove"),
                };
                if ui.button(favorite_button_label).clicked() {
                    app_state.favorites.toggle_favorite(entry.id);
                    ui.close_menu();
                }
            });
        }
    }
}
