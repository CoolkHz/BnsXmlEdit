use log::info;
use regex::Regex;

use crate::{
    page::{
        item_table::{edit_item_table, seach_item_table},
        main_page::{MyApp, ScrollDemo},
    },
    service::upgrade_jewelry::UpgradeJewelryList,
    util::tool::edit_log,
};

pub fn left_panel(me: &mut MyApp, ctx: &egui::Context, _ui: &mut egui::Ui) {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=500.0)
        .show(ctx, |ui| {
            // ui.vertical_centered(|ui| {
            //     ui.heading("Left Panel");
            // });

            // 表格按钮组
            ui.horizontal(|ui| {
                ui.selectable_value(&mut me.demo, ScrollDemo::ScrollTo, "待更改物品进化表");
                ui.selectable_value(&mut me.demo, ScrollDemo::ManyLines, "物品进化表");
                // ui.add(egui::TextEdit::singleline(&mut me.search_text));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("搜索进化").clicked() {
                        me.search_item_status = true;
                        me.search_item_status_1 = true;
                    }
                });
            });

            // 分割线
            ui.separator();
            // 匹配表格
            match me.demo {
                ScrollDemo::ScrollTo => edit_item_table(me, ui),
                ScrollDemo::ManyLines => seach_item_table(me, ui),
                // ScrollDemo::LargeCanvas => todo!(),
                // ScrollDemo::StickToEnd => todo!(),
                // ScrollDemo::Bidirectional => todo!(),
            }
            //表格

            ui.separator();
            if ui.button("一键清空待更改列表").clicked() {
                // me.item_table_data.clear();
                me.update_item_list = UpgradeJewelryList::default()
            }
            ui.add_space(20.0);
            ui.label("更换阶段只适合进化名末尾为数字且下一阶段是增长1的情况下使用");
            ui.horizontal(|ui| {
                // ui.add_space(20.0);
                if ui.button("武器概率成长：更换下一阶段").clicked() {
                    // me.item_table_data.clear();
                    let mut up_alias = vec![];
                    let mut new_up_alias = vec![];
                    // me.update_item_list = UpgradeJewelryList::default()
                    for i in &me.update_item_list.upgrade_jewelry_list {
                        up_alias.push(i.alias.clone());
                    }
                    let re = Regex::new(r"([^_]+)$").unwrap();
                    for i in &up_alias {
                        let last_number = re.find(i).unwrap().as_str();
                        // info!("last_number:{}", last_number);
                        if last_number == "BM" {
                            let new_len = i.len() - 3;
                            let new_i_str = &i[..new_len].to_string();
                            let num = re.find(new_i_str).unwrap().as_str();
                            info!("num:{}", num);
                            if let Ok(n) = num.parse::<i32>() {
                                info!("n:{}", n);
                                let new_num = n + 1;
                                info!("new_num:{}", new_num);
                                // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                // info!("new_alias:{}", new_alias);
                                let new_len = new_i_str.len() - 2;
                                let new_i_str = &i[..new_len].to_string();
                                let num_len = new_num.to_string().len();
                                if num_len == 1 {
                                    let new_i =
                                        new_i_str.to_string() + "0" + &new_num.to_string() + "_BM";
                                    // i = &i.to_string() + &new_num.to_string();
                                    new_up_alias.push(new_i.to_string());
                                } else {
                                    let new_i =
                                        new_i_str.to_string() + &new_num.to_string() + "_BM";
                                    new_up_alias.push(new_i.to_string());
                                }
                            }
                        } else {
                            if let Ok(n) = last_number.parse::<i32>() {
                                if n >= 7 {
                                    let new_num = n + 1;
                                    // info!("new_num:{}", new_num);
                                    // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                    // info!("new_alias:{}", new_alias);
                                    let new_len = i.len() - 2;
                                    let new_i_str = &i[..new_len].to_string();
                                    let num_len = new_num.to_string().len();
                                    if num_len == 1 {
                                        let new_i = new_i_str.to_string()
                                            + "0"
                                            + &new_num.to_string()
                                            + "_BM";
                                        // i = &i.to_string() + &new_num.to_string();
                                        new_up_alias.push(new_i.to_string());
                                    } else {
                                        let new_i =
                                            new_i_str.to_string() + &new_num.to_string() + "_BM";
                                        new_up_alias.push(new_i.to_string());
                                    }
                                } else {
                                    // info!("n:{}", n);
                                    let new_num = n + 1;
                                    // info!("new_num:{}", new_num);
                                    // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                    // info!("new_alias:{}", new_alias);
                                    let new_len = i.len() - 2;
                                    let new_i_str = &i[..new_len].to_string();
                                    let num_len = new_num.to_string().len();
                                    if num_len == 1 {
                                        let new_i =
                                            new_i_str.to_string() + "0" + &new_num.to_string();
                                        // i = &i.to_string() + &new_num.to_string();
                                        new_up_alias.push(new_i.to_string());
                                    } else {
                                        let new_i = new_i_str.to_string() + &new_num.to_string();
                                        new_up_alias.push(new_i.to_string());
                                    }
                                }
                            }
                        }
                    }
                    info!("up_alias:{:?}", new_up_alias);
                    me.update_item_list = UpgradeJewelryList::default();
                    for i in &new_up_alias {
                        let new_item = me
                            .item_list
                            .upgrade_jewelry_list
                            .iter()
                            .find(|&x| x.alias == i.clone());
                        // info!("new_item:{:?}", new_item);
                        if new_item != None {
                            me.update_item_list
                                .upgrade_jewelry_list
                                .push(new_item.unwrap().clone());
                            me.update_item_list.count =
                                me.update_item_list.count.map(|val| val + 1);
                        } else {
                            let log = format!("{}更换失败", i);
                            edit_log(me, log);
                        }
                        // return edit_log(me, "更换成功".to_string());
                    }
                }

                if ui.button("武器确定成长：更换下一阶段").clicked() {
                    // me.item_table_data.clear();
                    let mut up_alias = vec![];
                    let mut new_up_alias = vec![];
                    // me.update_item_list = UpgradeJewelryList::default()
                    for i in &me.update_item_list.upgrade_jewelry_list {
                        up_alias.push(i.alias.clone());
                    }
                    let re = Regex::new(r"([^_]+)$").unwrap();
                    for i in &up_alias {
                        let last_number = re.find(i).unwrap().as_str();
                        // info!("last_number:{}", last_number);
                        if last_number == "BM" {
                            let new_len = i.len() - 3;
                            let new_i_str = &i[..new_len].to_string();
                            let num = re.find(new_i_str).unwrap().as_str();
                            info!("num:{}", num);
                            if let Ok(n) = num.parse::<i32>() {
                                if n >= 7 {
                                    info!("n:{}", n);
                                    let new_num = n + 1;
                                    info!("new_num:{}", new_num);
                                    // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                    // info!("new_alias:{}", new_alias);
                                    let new_len = new_i_str.len() - 2;
                                    let new_i_str = &i[..new_len].to_string();
                                    let num_len = new_num.to_string().len();
                                    if num_len == 1 {
                                        let new_i = new_i_str.to_string()
                                            + "0"
                                            + &new_num.to_string()
                                            + "_BM2";
                                        // i = &i.to_string() + &new_num.to_string();
                                        new_up_alias.push(new_i.to_string());
                                    } else {
                                        let new_i =
                                            new_i_str.to_string() + &new_num.to_string() + "_BM2";
                                        new_up_alias.push(new_i.to_string());
                                    }
                                } else {
                                    info!("n:{}", n);
                                    let new_num = n + 1;
                                    info!("new_num:{}", new_num);
                                    // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                    // info!("new_alias:{}", new_alias);
                                    let new_len = new_i_str.len() - 2;
                                    let new_i_str = &i[..new_len].to_string();
                                    let num_len = new_num.to_string().len();
                                    if num_len == 1 {
                                        let new_i = new_i_str.to_string()
                                            + "0"
                                            + &new_num.to_string()
                                            + "_BM";
                                        // i = &i.to_string() + &new_num.to_string();
                                        new_up_alias.push(new_i.to_string());
                                    } else {
                                        let new_i =
                                            new_i_str.to_string() + &new_num.to_string() + "_BM";
                                        new_up_alias.push(new_i.to_string());
                                    }
                                }
                            }
                        } else if last_number == "BM2" {
                            let new_len = i.len() - 4;
                            let new_i_str = &i[..new_len].to_string();
                            let num = re.find(new_i_str).unwrap().as_str();
                            info!("num:{}", num);
                            if let Ok(n) = num.parse::<i32>() {
                                info!("n:{}", n);
                                let new_num = n + 1;
                                info!("new_num:{}", new_num);
                                // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                // info!("new_alias:{}", new_alias);
                                let new_len = new_i_str.len() - 2;
                                let new_i_str = &i[..new_len].to_string();
                                let num_len = new_num.to_string().len();
                                if num_len == 1 {
                                    let new_i =
                                        new_i_str.to_string() + "0" + &new_num.to_string() + "_BM2";
                                    // i = &i.to_string() + &new_num.to_string();
                                    new_up_alias.push(new_i.to_string());
                                } else {
                                    let new_i =
                                        new_i_str.to_string() + &new_num.to_string() + "_BM2";
                                    new_up_alias.push(new_i.to_string());
                                }
                            }
                        }
                    }
                    info!("up_alias:{:?}", new_up_alias);
                    me.update_item_list = UpgradeJewelryList::default();
                    for i in &new_up_alias {
                        let new_item = me
                            .item_list
                            .upgrade_jewelry_list
                            .iter()
                            .find(|&x| x.alias == i.clone());
                        // info!("new_item:{:?}", new_item);
                        if new_item != None {
                            me.update_item_list
                                .upgrade_jewelry_list
                                .push(new_item.unwrap().clone());
                            me.update_item_list.count =
                                me.update_item_list.count.map(|val| val + 1);
                        } else {
                            let log = format!("{}更换失败", i);
                            edit_log(me, log);
                        }
                        // return edit_log(me, "更换成功".to_string());
                    }
                }
            });
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("元气/真气石概率成长：更换下一阶段").clicked() {
                    // me.item_table_data.clear();
                    let mut up_alias = vec![];
                    let mut new_up_alias = vec![];
                    // me.update_item_list = UpgradeJewelryList::default()
                    for i in &me.update_item_list.upgrade_jewelry_list {
                        up_alias.push(i.alias.clone());
                    }
                    let re = Regex::new(r"([^_]+)$").unwrap();
                    for i in &up_alias {
                        let last_number = re.find(i).unwrap().as_str();
                        // info!("last_number:{}", last_number);
                        if last_number == "BM" {
                            let new_len = i.len() - 3;
                            let new_i_str = &i[..new_len].to_string();
                            let num = re.find(new_i_str).unwrap().as_str();
                            info!("num:{}", num);
                            if let Ok(n) = num.parse::<i32>() {
                                info!("n:{}", n);
                                let new_num = n + 1;
                                info!("new_num:{}", new_num);
                                // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                // info!("new_alias:{}", new_alias);
                                let new_len = new_i_str.len() - 2;
                                let new_i_str = &i[..new_len].to_string();
                                let num_len = new_num.to_string().len();
                                if num_len == 1 {
                                    let new_i =
                                        new_i_str.to_string() + "0" + &new_num.to_string() + "_BM";
                                    // i = &i.to_string() + &new_num.to_string();
                                    new_up_alias.push(new_i.to_string());
                                } else {
                                    let new_i =
                                        new_i_str.to_string() + &new_num.to_string() + "_BM";
                                    new_up_alias.push(new_i.to_string());
                                }
                            }
                        } else {
                            if let Ok(n) = last_number.parse::<i32>() {
                                // info!("n:{}", n);
                                let new_num = n + 1;
                                // info!("new_num:{}", new_num);
                                // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                // info!("new_alias:{}", new_alias);
                                let new_len = i.len() - 2;
                                let new_i_str = &i[..new_len].to_string();
                                let num_len = new_num.to_string().len();
                                if num_len == 1 {
                                    let new_i = new_i_str.to_string() + "0" + &new_num.to_string();
                                    // i = &i.to_string() + &new_num.to_string();
                                    new_up_alias.push(new_i.to_string());
                                } else {
                                    let new_i = new_i_str.to_string() + &new_num.to_string();
                                    new_up_alias.push(new_i.to_string());
                                }
                            }
                        }
                    }
                    info!("up_alias:{:?}", new_up_alias);
                    me.update_item_list = UpgradeJewelryList::default();
                    for i in &new_up_alias {
                        let new_item = me
                            .item_list
                            .upgrade_jewelry_list
                            .iter()
                            .find(|&x| x.alias == i.clone());
                        // info!("new_item:{:?}", new_item);
                        if new_item != None {
                            me.update_item_list
                                .upgrade_jewelry_list
                                .push(new_item.unwrap().clone());
                            me.update_item_list.count =
                                me.update_item_list.count.map(|val| val + 1);
                        } else {
                            let log = format!("{}更换失败", i);
                            edit_log(me, log);
                        }
                        // return edit_log(me, "更换成功".to_string());
                    }
                }
                ui.add_space(20.0);
                if ui.button("元气/真气石确定成长：更换下一阶段").clicked() {
                    // me.item_table_data.clear();
                    let mut up_alias = vec![];
                    let mut new_up_alias = vec![];
                    // me.update_item_list = UpgradeJewelryList::default()
                    for i in &me.update_item_list.upgrade_jewelry_list {
                        up_alias.push(i.alias.clone());
                    }
                    let re = Regex::new(r"([^_]+)$").unwrap();
                    for i in &up_alias {
                        let last_number = re.find(i).unwrap().as_str();
                        // info!("last_number:{}", last_number);
                        if last_number == "BM" {
                            let new_len = i.len() - 3;
                            let new_i_str = &i[..new_len].to_string();
                            let num = re.find(new_i_str).unwrap().as_str();
                            info!("num:{}", num);
                            if let Ok(n) = num.parse::<i32>() {
                                info!("n:{}", n);
                                let new_num = n + 1;
                                info!("new_num:{}", new_num);
                                // let new_alias = re.replace(i, new_num.to_string().as_str()).to_string();
                                // info!("new_alias:{}", new_alias);
                                let new_len = new_i_str.len() - 2;
                                let new_i_str = &i[..new_len].to_string();
                                let num_len = new_num.to_string().len();
                                if num_len == 1 {
                                    let new_i =
                                        new_i_str.to_string() + "0" + &new_num.to_string() + "_BM";
                                    // i = &i.to_string() + &new_num.to_string();
                                    new_up_alias.push(new_i.to_string());
                                } else {
                                    let new_i =
                                        new_i_str.to_string() + &new_num.to_string() + "_BM";
                                    new_up_alias.push(new_i.to_string());
                                }
                            }
                        }
                    }
                    info!("up_alias:{:?}", new_up_alias);
                    me.update_item_list = UpgradeJewelryList::default();
                    for i in &new_up_alias {
                        let new_item = me
                            .item_list
                            .upgrade_jewelry_list
                            .iter()
                            .find(|&x| x.alias == i.clone());
                        // info!("new_item:{:?}", new_item);
                        if new_item != None {
                            me.update_item_list
                                .upgrade_jewelry_list
                                .push(new_item.unwrap().clone());
                            me.update_item_list.count =
                                me.update_item_list.count.map(|val| val + 1);
                        } else {
                            let log = format!("{}更换失败", i);
                            edit_log(me, log);
                        }
                        // return edit_log(me, "更换成功".to_string());
                    }
                }
            });
            // ui.add_space(20.0);
        });
}
