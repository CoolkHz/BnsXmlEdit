use eframe::egui;
// use eframe::epi;
use egui::{Sense, Ui, Window};
use egui_extras::{Column, TableBuilder};

use crate::service::{
    load_item::{SearchGroceryData, SearchGroceryDataList},
    upgrade_jewelry::{is_in_update_item_list, UpgradeJewelryList},
};

use super::main_page::MyApp;

pub fn search_jewelry(ctx: &egui::Context, _ui: &mut Ui) {
    egui::Window::new("My Window").show(ctx, |ui| {
        ui.label("Hello World!");
    });
}

pub fn search_grocery(me: &mut MyApp, ctx: &egui::Context) {
    Window::new("查找物品")
        .open(&mut me.search_status)
        .min_height(600.0)
        .min_width(400.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut me.search_grocery_text));
                    if ui.button("搜索").clicked() {
                        me.search_grocery_data = SearchGroceryDataList::default();

                        let r_nums: Vec<_> = me
                            .grocery_list
                            .record
                            .iter_mut()
                            .filter(|r| {
                                r.id == me.search_grocery_text
                                    || r.alias.contains(&me.search_grocery_text) == true
                                    || r.tran
                                        .as_ref()
                                        .map_or(false, |t| t.contains(&me.search_grocery_text))
                            })
                            .collect();
                        for r in r_nums {
                            let search_data = SearchGroceryData {
                                id: r.id.clone(),
                                alias: r.alias.clone(),
                                tran: r.tran.clone(),
                            };
                            me.search_grocery_data.data.push(search_data);
                            me.search_grocery_data.count += 1;
                        }
                    }
                });
                ui.separator();
                TableBuilder::new(ui)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(25.0))
                    .column(Column::initial(60.0))
                    .column(Column::initial(300.0))
                    .column(Column::initial(150.0))
                    .column(Column::initial(50.0))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("序号");
                        });
                        header.col(|ui| {
                            ui.label("物品ID");
                        });
                        header.col(|ui| {
                            ui.label("物品别名");
                        });
                        header.col(|ui| {
                            ui.label("物品名称");
                        });
                    })
                    .body(|body| {
                        body.rows(
                            20.0,
                            me.search_grocery_data.count.try_into().unwrap(),
                            |row_index, mut row| {
                                row.col(|ui| {
                                    ui.label(row_index.to_string());
                                });
                                row.col(|ui| {
                                    if let Some(elem) = me.search_grocery_data.data.get(row_index) {
                                        ui.label(elem.id.to_string());
                                    }
                                    // ui.label(me.search_grocery_data.data[row_index].id.to_string());
                                });
                                row.col(|ui| {
                                    if let Some(elem) = me.search_grocery_data.data.get(row_index) {
                                        ui.label(elem.alias.to_string());
                                    }
                                    // ui.label(
                                    //     me.search_grocery_data.data[row_index].alias.to_string(),
                                    // );
                                });
                                row.col(|ui| {
                                    if let Some(elem) = me.search_grocery_data.data.get(row_index) {
                                        ui.label(
                                            elem.tran.as_deref().unwrap_or_default().to_string(),
                                        );
                                    }
                                });
                                row.col(|ui| {
                                    if ui.button("选中").clicked() {
                                        match me.add_item_type.0 {
                                            1 => {
                                                let sub_item = "item:".to_string()
                                                    + &me.search_grocery_data.data[row_index].alias;
                                                me.evolutionary_algorithm.sub_ingredient_1 =
                                                    sub_item;
                                                let tran_str = me.search_grocery_data.data
                                                    [row_index]
                                                    .tran
                                                    .as_ref()
                                                    .unwrap()
                                                    .to_string();

                                                me.evolutionary_algorithm.tran = tran_str;
                                                me.evolutionary_algorithm.sub_ingredient_1_id =
                                                    me.search_grocery_data.data[row_index]
                                                        .id
                                                        .clone();
                                            }
                                            2 => match me.add_item_type.1 {
                                                1 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_1 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_1_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_1_tran = tran_str;
                                                }
                                                2 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_2 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_2_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_2_tran = tran_str;
                                                }
                                                3 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_3 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_3_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_3_tran = tran_str;
                                                }
                                                4 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_4 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_4_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_4_tran = tran_str;
                                                }
                                                5 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_5 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_5_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_5_tran = tran_str;
                                                }
                                                6 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_6 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_6_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();

                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_6_tran = tran_str;
                                                }
                                                7 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_7 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_7_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_7_tran = tran_str;
                                                }
                                                8 => {
                                                    let fixed_item = &me.search_grocery_data.data
                                                        [row_index]
                                                        .alias;
                                                    me.evolutionary_algorithm.fixed_ingredient_8 =
                                                        fixed_item.to_string();
                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_8_id =
                                                        me.search_grocery_data.data[row_index]
                                                            .id
                                                            .clone();
                                                    let tran_str = me.search_grocery_data.data
                                                        [row_index]
                                                        .tran
                                                        .as_ref()
                                                        .unwrap()
                                                        .to_string();

                                                    me.evolutionary_algorithm
                                                        .fixed_ingredient_8_tran = tran_str;
                                                }
                                                _ => {}
                                            },
                                            _ => {}
                                        }
                                        me.add_item_type = (0, 0);
                                        me.search_status_1 = false;
                                        me.search_grocery_data = SearchGroceryDataList::default();
                                        me.search_grocery_text = "".to_string();
                                    }
                                });
                            },
                        );
                    });
            });
        });

    // CentralPanel::default().show(ctx, |_| {

    // });
}

pub fn search_item(me: &mut MyApp, ctx: &egui::Context) {
    Window::new("查找物品(双击添加)")
        .open(&mut me.search_item_status)
        .min_height(600.0)
        .min_width(400.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut me.search_text));
                    if ui.button("搜索").clicked() {
                        me.search_item_data = UpgradeJewelryList::default();

                        let r_nums: Vec<_> = me
                            .item_list
                            .upgrade_jewelry_list
                            .iter_mut()
                            .filter(|r| {
                                r.alias.contains(&me.search_text) == true
                                    || r.tran
                                        .as_ref()
                                        .map_or(false, |t| t.contains(&me.search_text))
                            })
                            .collect();

                        for r in r_nums {
                            // let search_item_data = UpgradeJewelryList {
                            //     id: r.id.clone(),
                            //     alias: r.alias.clone(),
                            //     tran: Some("".to_string()),
                            //     count: todo!(),
                            // };
                            me.search_item_data.upgrade_jewelry_list.push(r.clone());
                            let b: Option<usize> = Some(1);
                            me.search_item_data.count =
                                Some(me.search_item_data.count.unwrap_or(0) + b.unwrap_or(0));
                        }
                    }
                });
            });
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().range(15.0..=60.0))
                .column(Column::remainder().range(40.0..=300.0).clip(true))
                .column(Column::remainder().range(40.0..=80.0).clip(true))
                .column(Column::remainder().range(40.0..=200.0).clip(true))
                .column(Column::remainder().range(80.0..=120.0).clip(true))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label("序号");
                    });
                    header.col(|ui| {
                        ui.label("进化别名");
                    });
                    header.col(|ui| {
                        ui.label("物品编号");
                    });
                    header.col(|ui| {
                        ui.label("物品别名");
                    });
                    header.col(|ui| {
                        ui.label("物品名称");
                    });
                })
                .body(|body| {
                    body.rows(
                        20.0,
                        me.search_item_data.count.unwrap(),
                        |row_index, mut row| {
                            //检测鼠标是否在行上
                            row.col(|ui| {
                                if ui
                                    .interact(
                                        ui.available_rect_before_wrap(),
                                        ui.make_persistent_id(row_index),
                                        Sense::click(),
                                    )
                                    .double_clicked()
                                {
                                    if is_in_update_item_list(
                                        &me.update_item_list,
                                        me.search_item_data.upgrade_jewelry_list[row_index]
                                            .alias
                                            .to_string(),
                                    ) == false
                                    {
                                        me.update_item_list.upgrade_jewelry_list.push(
                                            me.search_item_data.upgrade_jewelry_list[row_index]
                                                .clone(),
                                        );
                                        me.update_item_list.count =
                                            me.update_item_list.count.map(|val| val + 1);
                                    }
                                }

                                ui.label(row_index.to_string());
                            });
                            row.col(|ui| {
                                // ui.text_edit_singleline(&mut me.item_list.list[0].em_name);
                                ui.label(
                                    me.search_item_data.upgrade_jewelry_list[row_index]
                                        .alias
                                        .to_string(),
                                );
                            });
                            row.col(|ui| {
                                ui.label("暂无");
                            });
                            row.col(|ui| {
                                ui.label(
                                    me.search_item_data.upgrade_jewelry_list[row_index]
                                        .title_item
                                        .as_deref()
                                        .unwrap_or_default()
                                        .to_string(),
                                );
                            });
                            row.col(|ui| {
                                ui.label(
                                    me.search_item_data.upgrade_jewelry_list[row_index]
                                        .tran
                                        .as_deref()
                                        .unwrap_or_default()
                                        .to_string(),
                                );
                                if ui.button("选中").clicked() {
                                    if is_in_update_item_list(
                                        &me.update_item_list,
                                        me.search_item_data.upgrade_jewelry_list[row_index]
                                            .alias
                                            .to_string(),
                                    ) == false
                                    {
                                        me.update_item_list.upgrade_jewelry_list.push(
                                            me.search_item_data.upgrade_jewelry_list[row_index]
                                                .clone(),
                                        );
                                        me.update_item_list.count =
                                            me.update_item_list.count.map(|val| val + 1);
                                    }
                                }
                            });
                            // row.col(|ui| {
                            //     ui.label(
                            //         me.search_item_data.upgrade_jewelry_list[row_index]
                            //             .tran
                            //             .as_deref()
                            //             .unwrap_or_default()
                            //             .to_string(),
                            //     );
                            // });
                        },
                    );
                });
        });

    // CentralPanel::default().show(ctx, |_| {

    // });
}
