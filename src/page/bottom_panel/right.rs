use crate::page::main_page::MyApp;

pub fn right_panel(_me: &mut MyApp, ctx: &egui::Context, _ui: &mut egui::Ui) {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(200.0)
        .width_range(200.0..=500.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("待定区域");
            });
        });
}
