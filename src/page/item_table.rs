use egui::Sense;
use egui_extras::{Column, TableBuilder};

use crate::{service::upgrade_jewelry::is_in_update_item_list, util::tool::edit_log};

use super::main_page::MyApp;

pub fn edit_item_table(me: &mut MyApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().range(15.0..=60.0))
            .column(Column::auto().range(40.0..=300.0).clip(true))
            .column(Column::auto().range(40.0..=300.0).clip(true))
            .column(Column::auto().range(40.0..=300.0).clip(true))
            .column(Column::remainder().range(40.0..=300.0).clip(true))
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
                    me.update_item_list
                        .count
                        .unwrap_or_default()
                        .try_into()
                        .unwrap(),
                    |row_index, mut row| {
                        row.col(|ui| {
                            if ui
                                .interact(
                                    ui.available_rect_before_wrap(),
                                    ui.make_persistent_id(row_index),
                                    Sense::click(),
                                )
                                .double_clicked()
                            {
                                me.update_item_list.upgrade_jewelry_list.remove(row_index);

                                me.update_item_list.count =
                                    me.update_item_list.count.map(|val| val - 1);
                                let log = format!(
                                    "已移出待更改列表 {}",
                                    me.item_list.upgrade_jewelry_list[row_index]
                                        .alias
                                        .to_string()
                                );
                                // info!("{:?}", me.update_item_list);
                                return edit_log(me, log);
                            }

                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            // ui.text_edit_singleline(&mut me.item_list.list[0].em_name);
                            // 判断是否存在
                            let is_e = me.update_item_list.upgrade_jewelry_list.get(row_index);

                            if is_e != None {
                                ui.label(
                                    me.update_item_list.upgrade_jewelry_list[row_index]
                                        .alias
                                        .to_string(),
                                );
                            }
                        });
                        row.col(|ui| {
                            ui.label("暂无");
                        });
                        row.col(|ui| {
                            let is_e = me.update_item_list.upgrade_jewelry_list.get(row_index);
                            if is_e != None {
                                ui.label(
                                    me.update_item_list.upgrade_jewelry_list[row_index]
                                        .title_item
                                        .as_deref()
                                        .unwrap_or_default()
                                        .to_string(),
                                );
                            }
                        });
                        row.col(|ui| {
                            let is_e = me.update_item_list.upgrade_jewelry_list.get(row_index);

                            if is_e != None {
                                ui.label(
                                    me.update_item_list.upgrade_jewelry_list[row_index]
                                        .tran
                                        .as_deref()
                                        .unwrap_or_default()
                                        .to_string(),
                                );
                            }
                        });
                    },
                );
            });
    });
}

pub fn seach_item_table(me: &mut MyApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        //搜索框
        ui.horizontal(|_ui| {});
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().range(15.0..=60.0))
            .column(Column::remainder().range(40.0..=200.0).clip(true))
            .column(Column::remainder().range(40.0..=80.0).clip(true))
            .column(Column::remainder().range(40.0..=200.0).clip(true))
            .column(Column::remainder().range(40.0..=100.0).clip(true))
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
                body.rows(20.0, me.item_list.count.unwrap(), |row_index, mut row| {
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
                                me.item_list.upgrade_jewelry_list[row_index]
                                    .alias
                                    .to_string(),
                            ) == true
                            {
                                let log = format!(
                                    "该物品已存在 {}",
                                    me.item_list.upgrade_jewelry_list[row_index]
                                        .alias
                                        .to_string()
                                );
                                return edit_log(me, log);
                            }

                            me.update_item_list
                                .upgrade_jewelry_list
                                .push(me.item_list.upgrade_jewelry_list[row_index].clone());
                            me.update_item_list.count =
                                me.update_item_list.count.map(|val| val + 1);
                            let log = format!(
                                "添加到待修改列表成功 {}",
                                me.item_list.upgrade_jewelry_list[row_index]
                                    .alias
                                    .to_string()
                            );
                            // info!("{:?}", me.update_item_list);
                            return edit_log(me, log);
                        }

                        ui.label(row_index.to_string());
                    });
                    row.col(|ui| {
                        // ui.text_edit_singleline(&mut me.item_list.list[0].em_name);
                        ui.label(
                            me.item_list.upgrade_jewelry_list[row_index]
                                .alias
                                .to_string(),
                        );
                    });
                    row.col(|ui| {
                        ui.label("暂无");
                    });
                    row.col(|ui| {
                        ui.label(
                            me.item_list.upgrade_jewelry_list[row_index]
                                .title_item
                                .as_deref()
                                .unwrap_or_default()
                                .to_string(),
                        );
                    });
                    row.col(|ui| {
                        ui.label(
                            me.item_list.upgrade_jewelry_list[row_index]
                                .tran
                                .as_deref()
                                .unwrap_or_default()
                                .to_string(),
                        );
                    });
                });
            });
    });
}
