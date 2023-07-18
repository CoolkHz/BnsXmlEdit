use egui_extras::Size;
use egui_extras::StripBuilder;

use crate::page::main_page::MyApp;

pub fn center_panel(me: &mut MyApp, ctx: &egui::Context, _ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ctx, |ui| {
        StripBuilder::new(ui)
            .size(Size::exact(40.0)) // bottom cell
            .vertical(|mut strip| {
                // We add a nested strip in the bottom cell:
                strip.strip(|builder| {
                    builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                        strip.cell(|ui| {
                           
                            ui.vertical_centered(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("进化配方");
                                    // 分割线
                                    ui.separator();
                            });

                            egui::Grid::new("my_grid")
                                .num_columns(3)
                                .spacing([40.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {

                                    ui.heading("基本属性名");
                                    ui.heading("基本属性值");
                                    ui.heading("说明");
                                    ui.end_row();

                                    ui.label("金钱成本");
                                    ui.text_edit_singleline(&mut me.evolutionary_algorithm.money_cost);
                                    ui.label("单位：铜");
                                    ui.end_row();

                                    ui.label("主要材料");
                                    ui.label("---");
                                    ui.label("无法更改-批量更改模式");
                                    ui.end_row();

                                    ui.label("主要材料类型");
                                    ui.label("---");
                                    ui.label("无法更改-批量更改模式");
                                    ui.end_row();

                                    ui.label("主要材料显示物品");
                                    ui.label("---");
                                    ui.label("无法更改-批量更改模式");
                                    ui.end_row();

                                    // ui.label("主要材料数量要求");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.main_ingredient_stack_count);
                                    // // ui.label("无法更改-批量更改模式");
                                    // ui.end_row();

                                    // ui.label("是否保留主材料宝石插槽");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.keep_main_ingredient_weapon_gem_slot);
                                    // ui.label("y:是/n：否");
                                    // ui.end_row();

                                    // ui.label("是否保留主材料外观");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.keep_main_ingredient_weapon_appearance);
                                    // ui.label("y:是/n：否");
                                    // ui.end_row();

                                    // ui.label("是否保留主材料精气");
                                    // ui.text_edit_singleline(&mut 0.to_string());
                                    // ui.label("1:是/0：否");
                                    // ui.end_row();

                                    // ui.label("进化失败时是否消耗主材料");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.main);
                                    // ui.label("1:是/0：否");
                                    // ui.end_row();

                                    ui.label("进化失败时是否消耗祭品");
                                    ui.text_edit_singleline(&mut me.evolutionary_algorithm.consume_sub_ingredient);
                                    ui.label("y:是/n：否");
                                    ui.end_row();

                                    ui.label("进化失败时是否消耗材料");
                                    ui.text_edit_singleline(&mut me.evolutionary_algorithm.consume_fixed_ingredient);
                                    ui.label("y:是/n：否");
                                    ui.end_row();

                                    // ui.label("进化类别");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.category);
                                    // ui.label("4:普通武器/5:传说武器-暂不支持更改");
                                    // ui.end_row();

                                    // ui.label("是否使用随机进化");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.use_random);
                                    // ui.label("y:是/n：否").on_hover_text("如果使用随机，则进化结果按照 随机物品 配置结果进行进化\n如果不使用随机，则进化结果按照 固定物品 配置结果进行进化");
                                    // ui.end_row();

                                    // ui.label("是否保存随机进化进度");
                                    // ui.text_edit_singleline(&mut me.evolutionary_algorithm.random_failure_mileage_save);
                                    // ui.label("y:是/n：否").on_hover_text("保存失败进度，多次失败且进度值达100%时，将确定成长,三系似乎没作用");
                                    // ui.end_row();

                                    ui.label("提示信息");
                                    ui.text_edit_singleline(&mut me.evolutionary_algorithm.todo);
                                    ui.label("请输入对应编号").on_hover_text("0 - 确定成长\n1 - 消耗使用材料，有一定概率成长失败。\n2 - 使用材料将被消耗，有一定几率获得无法成长的桃子粉末\n3 - 成长时将不反应当前武器中增加的宝石槽，进行初始化。\n4 - 消耗使用材料，有一定概率成长失败。成长时当前武器上不会体现增加的宝石槽，并且初始化。\n5 - 可获得有一定概率无法成长的 黑天装备 。成长时将不反应当前武器中增加的宝石槽，进行初始化。\n6 - 有一定概率成长失败。失败时，只消耗 神秘水和 金，其他材料保留。\n7 - 有一定概率使解放数值下降。进化积分对于成功率不会造成任何影响。有进化积分时，将初始化。\n8 - 有一定概率使解放数值下降。进化积分对于成功率不会造成任何影响。有进化积分时，将初始化。当前装备上附加的宝石孔无法得到反映，将初始化。\n9 - 存在着成长失败的可能性。失败后，仅有祭品材料会被消耗掉，其他材料将得到保留。\n10 - 变换后将获得 无法交易的产物。\n11 - 未知\n12 - 未知\n13 - 传授到当前装备的能力值全部消失。\n14 - 有一定概率 失败。成长时，当前装备上所传授的能力值将消失。\n15 - 现在装备上添加的宝石孔及传授的能力值不被反映，将初始化。\n16 - 有一定概率 失败。现在装备上添加的宝石孔及传授的能力值不被反映，将初始化。\n17 - 使用材料将被消耗，有一定概率成长 失败，或阶段 下滑。成长时，当前装备上所传授的能力值将消失。\n18 - 使用材料被消耗掉，有一定概率成长 失败或阶段 下降。成长时，现在装备上添加的宝石孔及传授的能力值不被反映，将初始化。\n19 - 有一定概率成长 失败。失败后，仅消耗祭品材料，其他材料将保留。成长时，当前装备上所传授的能力值将消失。\n20 - 进化成觉醒神功牌时无法分离。\n21 - 有一定概率 会失败。进化成觉醒神功牌时无法分离。\n");
                                    ui.end_row();

                                });
                        });
                        });
                        strip.cell(|ui| {
                            StripBuilder::new(ui)
                                .size(Size::exact(200.0)) // top cell
                                .size(Size::exact(200.0)) // bottom cell
                                .vertical(|mut strip| {
                                    // Add the top 'cell'
                                    strip.cell(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.heading("进化结果");
                                                // 分割线
                                                ui.separator();
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("固定物品成功率:");
                                                ui.text_edit_singleline(&mut "不支持修改物品".to_string());
                                            });
                                            ui.separator();

                                        egui::Grid::new("guding_grid")
                                            .num_columns(3)
                                            .spacing([40.0, 4.0])
                                            .striped(true)
                                            .show(ui, |ui| {        
                                                ui.label("固定产出物品名称");
                                                ui.label("产出数量");
                                                ui.button("添加").on_hover_text("添加固定产出物品");
                                                ui.end_row();
        
                                                ui.label("未开放");
                                                ui.text_edit_singleline(&mut 0.to_string());
                                                if ui.button("X").clicked() {
                                                    // println!("点击删除材料");
                                                }
                                                ui.end_row();
                                            });
                                      ui.separator();

                                    });
                                    });
                                    strip.cell(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.separator();
    
                                            ui.horizontal(|ui| {
                                                ui.label("随机物品成功率:").on_hover_text("100=100%");
                                                ui.text_edit_singleline(&mut me.evolutionary_algorithm.random_item_success_probability);
                                            });
                                            ui.separator();
                                        egui::Grid::new("suiji_grid")
                                            .num_columns(4)
                                            .spacing([40.0, 4.0])
                                            .striped(true)
                                            .show(ui, |ui| {
        
                                                ui.label("随机物品名称");
                                                ui.label("随即权重(可)");
                                                ui.label("产出数量(可)");
                                                ui.button("添加物品").on_hover_text("添加随机产出物品-暂未开放");
                                                ui.end_row();
        
                                                ui.label("暂不支持批量修改");

                                                ui.text_edit_singleline(&mut me.evolutionary_algorithm.random_item_select_prop_weight_1);
                                                ui.text_edit_singleline(&mut me.evolutionary_algorithm.random_item_stack_count_1);
                                                // if ui.button("X").clicked() {
                                                //     println!("点击删除材料");
                                                // }
                                                ui.end_row();
                                            });
                                    });
                                    });
                                });
                        });
                    });
                });
            });
    });
}
