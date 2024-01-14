use eframe::egui::{Context, TopBottomPanel};
use strum::IntoEnumIterator;

use crate::{tabs::Tab, app_state::AppState};

pub fn render(ctx: &Context, app_state: &mut AppState) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for tab in Tab::iter() {
                ui.selectable_value(&mut app_state.selected_tab, tab, tab.get_localized_name());
            }
        });
        ui.add_space(2.0);
    });
}
