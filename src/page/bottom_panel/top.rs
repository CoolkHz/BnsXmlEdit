use egui::Button;

use crate::{
    page::main_page::{Algorithm, MyApp},
    service::{
        load_item::{load_grocery, load_tran},
        save_item::{save_jewelry_client, save_jewelry_server},
        upgrade_jewelry::{load_jewelry_client, load_jewelry_server, UpgradeJewelryList},
    },
    util::{
        get_winpc_id::get_system_uuid,
        tool::{edit_log, get_config, get_res_code, is_have_token, set_config, time_now_format},
    },
};

pub fn top_panel(me: &mut MyApp, ctx: &egui::Context, _ui: &mut egui::Ui) {
    egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .default_height(32.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    if me.radio == Algorithm::None {
                        ui.radio_value(&mut me.radio, Algorithm::Evolutionary, "首饰");
                        ui.radio_value(&mut me.radio, Algorithm::Weapon, "武器");
                    }

                    ui.add_space(20.0);

                    ui.add(
                        egui::TextEdit::singleline(&mut me.config.directory.server)
                            .hint_text("选择服务端目录"),
                    );
                    if ui.button("选择").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            me.config.directory.server = path.display().to_string();
                            set_config("directory", "server", path.display().to_string().as_str());
                        }
                    }
                    ui.add_space(20.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut me.config.directory.client)
                            .hint_text("选择客户端目录"),
                    );
                    if ui.button("选择").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            me.config.directory.client = path.display().to_string();
                            set_config("directory", "client", path.display().to_string().as_str());
                        }
                    }
                    ui.add_space(20.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut me.config.directory.tran)
                            .hint_text("选择327目录"),
                    );
                    if ui.button("选择").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            me.config.directory.tran = path.display().to_string();
                            set_config("directory", "tran", path.display().to_string().as_str());
                        }
                    }

                    if ui.button("载入数据").clicked() {
                        let is_have_token = is_have_token();
                        me.is_have_token = is_have_token;

                        me.item_list = UpgradeJewelryList::default();

                        if me.radio == Algorithm::None {
                            return edit_log(me, "请选择进化种类 首饰/武器".to_string());
                        }

                        if me.radio == Algorithm::Evolutionary {
                            let data = load_jewelry_server(me.radio);
                            if let Err(e) = data {
                                let log = time_now_format().as_str().to_owned()
                                    + format!(" | {:?}\n", e).as_str();
                                return me.edit_log.insert_str(0, &log);
                            };
                            me.item_list = data.unwrap();
                        } else if me.radio == Algorithm::Weapon {
                            let data = load_jewelry_server(me.radio);
                            if let Err(e) = data {
                                let log = time_now_format().as_str().to_owned()
                                    + format!(" | {:?}\n", e).as_str();
                                return me.edit_log.insert_str(0, &log);
                            };
                            me.item_list = data.unwrap();
                        }

                        // info!("{:?}", me.item_list);

                        if let Err(e) = load_jewelry_client(&mut me.item_list) {
                            let log = time_now_format().as_str().to_owned()
                                + format!(" | {:?}\n", e).as_str();
                            return me.edit_log.insert_str(0, &log);
                        };

                        if let Err(e) = load_grocery(&mut me.grocery_list) {
                            let log = time_now_format().as_str().to_owned()
                                + format!(" | {:?}\n", e).as_str();
                            return me.edit_log.insert_str(0, &log);
                        };

                        let load_tran = load_tran(me);
                        if let Err(e) = load_tran {
                            return edit_log(me, e);
                        };

                        let log = time_now_format().as_str().to_owned() + " | 载入数据成功\n";
                        me.edit_log.insert_str(0, &log);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("☀/🌙").clicked() {
                            let dark_mode = get_config("theme", "dark_mode");
                            if dark_mode == "true" {
                                ctx.set_visuals(egui::Visuals::light());
                                set_config("theme", "dark_mode", "false");
                            } else {
                                ctx.set_visuals(egui::Visuals::dark());
                                set_config("theme", "dark_mode", "true");
                            }
                        };
                        if ui.add(Button::new("保存数据")).clicked() {
                            if me.update_item_list.upgrade_jewelry_list.len() == 0 {
                                let log =
                                    time_now_format().as_str().to_owned() + " | 请先添加更新物品\n";
                                return me.edit_log.insert_str(0, &log);
                            }

                            for mut i in &mut me.update_item_list.upgrade_jewelry_list {
                                i.money_cost = Some(me.evolutionary_algorithm.money_cost.clone());
                                // info!("{:?}", i.money_cost);
                                i.consume_sub_ingredient =
                                    Some(me.evolutionary_algorithm.consume_sub_ingredient.clone());
                                i.consume_fixed_ingredient = Some(
                                    me.evolutionary_algorithm.consume_fixed_ingredient.clone(),
                                );
                                i.random_failure_mileage_save = Some(
                                    me.evolutionary_algorithm
                                        .random_failure_mileage_save
                                        .clone(),
                                );
                                // if me.evolutionary_algorithm.
                                if me.evolutionary_algorithm.sub_ingredient_1 != "" {
                                    i.sub_ingredient_1 =
                                        Some(me.evolutionary_algorithm.sub_ingredient_1.clone());
                                } else {
                                    return edit_log(me, "请添加祭品".to_string());
                                }
                                if me.evolutionary_algorithm.sub_ingredient_stack_count_1 != "" {
                                    i.sub_ingredient_stack_count_1 = Some(
                                        me.evolutionary_algorithm
                                            .sub_ingredient_stack_count_1
                                            .clone(),
                                    );
                                } else {
                                    return edit_log(me, "请填写祭品数量".to_string());
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_1 != "" {
                                    i.fixed_ingredient_1 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_1.clone());
                                } else {
                                    i.fixed_ingredient_1 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_2 != "" {
                                    i.fixed_ingredient_2 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_2.clone());
                                } else {
                                    i.fixed_ingredient_2 = None;
                                }

                                if me.evolutionary_algorithm.fixed_ingredient_3 != "" {
                                    i.fixed_ingredient_3 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_3.clone());
                                } else {
                                    i.fixed_ingredient_3 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_4 != "" {
                                    i.fixed_ingredient_4 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_4.clone());
                                } else {
                                    i.fixed_ingredient_4 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_5 != "" {
                                    i.fixed_ingredient_5 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_5.clone());
                                } else {
                                    i.fixed_ingredient_5 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_6 != "" {
                                    i.fixed_ingredient_6 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_6.clone());
                                } else {
                                    i.fixed_ingredient_6 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_7 != "" {
                                    i.fixed_ingredient_7 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_7.clone());
                                } else {
                                    i.fixed_ingredient_7 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_8 != "" {
                                    i.fixed_ingredient_8 =
                                        Some(me.evolutionary_algorithm.fixed_ingredient_8.clone());
                                } else {
                                    i.fixed_ingredient_8 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_1 != "" {
                                    i.fixed_ingredient_stack_count_1 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_1
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_1 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_2 != "" {
                                    i.fixed_ingredient_stack_count_2 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_2
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_2 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_3 != "" {
                                    i.fixed_ingredient_stack_count_3 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_3
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_3 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_4 != "" {
                                    i.fixed_ingredient_stack_count_4 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_4
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_4 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_5 != "" {
                                    i.fixed_ingredient_stack_count_5 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_5
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_5 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_6 != "" {
                                    i.fixed_ingredient_stack_count_6 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_6
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_6 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_7 != "" {
                                    i.fixed_ingredient_stack_count_7 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_7
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_7 = None;
                                }
                                if me.evolutionary_algorithm.fixed_ingredient_stack_count_8 != "" {
                                    i.fixed_ingredient_stack_count_8 = Some(
                                        me.evolutionary_algorithm
                                            .fixed_ingredient_stack_count_8
                                            .clone(),
                                    );
                                } else {
                                    i.fixed_ingredient_stack_count_8 = None;
                                }
                                i.random_item_select_prop_weight_1 = Some(
                                    me.evolutionary_algorithm
                                        .random_item_select_prop_weight_1
                                        .clone(),
                                );
                                i.random_item_stack_count_1 = Some(
                                    me.evolutionary_algorithm.random_item_stack_count_1.clone(),
                                );
                                i.random_item_success_probability = Some(
                                    me.evolutionary_algorithm
                                        .random_item_success_probability
                                        .clone(),
                                );
                            }

                            if me.radio == Algorithm::None {
                                return edit_log(me, "请选择进化种类 首饰/武器".to_string());
                            }

                            if me.radio == Algorithm::Evolutionary {
                                let save_server =
                                    save_jewelry_server(&mut me.update_item_list, me.radio);
                                if save_server.is_err() {
                                    edit_log(me, "服务端保存失败".to_string());
                                } else {
                                    edit_log(me, "服务端保存成功".to_string());
                                }
                            } else if me.radio == Algorithm::Weapon {
                                let save_server =
                                    save_jewelry_server(&mut me.update_item_list, me.radio);
                                if save_server.is_err() {
                                    edit_log(me, "服务端保存失败".to_string());
                                } else {
                                    edit_log(me, "服务端保存成功".to_string());
                                }
                            }

                            let save_client = save_jewelry_client(
                                &mut me.update_item_list,
                                &mut me.evolutionary_algorithm,
                            );
                            if save_client.is_err() {
                                edit_log(me, "客户端保存失败".to_string());
                            } else {
                                edit_log(me, "客户端保存成功".to_string());
                            }

                            // return edit_log(me, "保存成功".to_string());
                        }
                    });
                });
            });
        });
}
