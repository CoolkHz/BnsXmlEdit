use egui_extras::Size;
use egui_extras::StripBuilder;

use crate::page::main_page::MyApp;

pub fn bt_panel(me: &mut MyApp, ctx: &egui::Context, _ui: &mut egui::Ui) {
    egui::TopBottomPanel::bottom("cailiao_panel")
        .resizable(false)
        .default_height(150.0)
        .max_height(150.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // 遍历me.edit_log 每行都输出一个label
                ui.horizontal_centered(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut me.edit_log)
                                .desired_rows(9)
                                .lock_focus(true)
                                .cursor_at_end(true)
                                .desired_width(f32::INFINITY),
                        );
                    });
                });
            });
        });
    egui::TopBottomPanel::bottom("add_panel")
        .resizable(true)
        .min_height(150.0)
        .max_height(300.0)
        .show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(50.0))
                .size(Size::remainder())
                .size(Size::relative(0.5).at_least(60.0))
                .size(Size::exact(10.0))
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("祭品");
                                    ui.separator();
                                    egui::Grid::new("jipin_grid")
                                        .num_columns(3)
                                        .spacing([40.0, 4.0])
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label("祭品");
                                            ui.label("数量");
                                            ui.end_row();
                                            ui.label(me.evolutionary_algorithm.tran.to_string());
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .sub_ingredient_stack_count_1,
                                            );
                                            if ui.button("更改祭品").clicked() {
                                                // 当前时间 + 点击添加祭品
                                                // search_grocery(ctx, ui);
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (1, 1);
                                                // let log = time_now_format().as_str().to_owned()
                                                //     + " | 点击更改祭品\n";
                                                // me.edit_log.insert_str(0, &log);
                                            }
                                            ui.end_row();
                                        });
                                });
                            });
                            strip.cell(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("祭品");
                                    ui.separator();
                                    egui::Grid::new("材料_grid")
                                        .num_columns(3)
                                        .spacing([40.0, 4.0])
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label("材料");
                                            ui.label("数量");
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_1_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_1,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_1 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_1 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_1_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 1);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_2_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_2,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_2 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_2 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_2_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 2);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_3_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_3,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_3 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_3 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_3_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 3);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_4_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_4,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_4 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_4 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_4_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 4);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_5_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_5,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_5 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_5 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_5_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 5);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_6_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_6,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_6 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_6 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_6_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 6);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_7_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_7,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_7 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_7 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_7_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 7);
                                            }
                                            ui.end_row();
                                            ui.label(
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_8_tran
                                                    .to_string(),
                                            );
                                            ui.text_edit_singleline(
                                                &mut me
                                                    .evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_8,
                                            );
                                            if ui.button("X").clicked() {
                                                me.evolutionary_algorithm.fixed_ingredient_8 =
                                                    String::new();
                                                me.evolutionary_algorithm
                                                    .fixed_ingredient_stack_count_8 = String::new();
                                                me.evolutionary_algorithm.fixed_ingredient_8_tran =
                                                    String::new();
                                            }
                                            if ui.button("+").clicked() {
                                                me.search_status = true;
                                                me.search_status_1 = true;
                                                me.add_item_type = (2, 8);
                                            }
                                            ui.end_row();
                                        });
                                });
                            });
                        });
                    });
                });

            // ui.horizontal(|ui| {
            //     egui::Grid::new("材料_grid")
            //         .num_columns(3)
            //         .spacing([40.0, 4.0])
            //         .striped(true)
            //         .show(ui, |ui| {
            //             ui.label("奇遇1币");
            //             ui.text_edit_singleline(&mut 10.to_string());
            //             ui.end_row();
            //             ui.label("奇遇1币");
            //             ui.text_edit_singleline(&mut 10.to_string());
            //             ui.end_row();
            //         });
            // });
        });
}
