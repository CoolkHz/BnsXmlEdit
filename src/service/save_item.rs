use log::info;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Write};

use crate::page::main_page::Algorithm;
use crate::util::tool::{get_config, is_file, is_path};

use super::upgrade_jewelry::{
    is_in_update_item_list, EvolutionaryAlgorithm, UpgradeJewelry, UpgradeJewelryList,
};

pub fn save_jewelry_server(
    update_item_list: &mut UpgradeJewelryList,
    radio: Algorithm,
) -> Result<(), String> {
    if is_path() == false {
        return Err("Error! 未找到当前目录！".to_string());
        // // info!("Error! 未找到当前目录！")
    }

    let mut jewelry_server_path = String::new();
    if radio == Algorithm::Evolutionary {
        jewelry_server_path =
            get_config("directory", "server") + "/itemtransformrecipedata_evolution.xml";
    } else if radio == Algorithm::Weapon {
        jewelry_server_path =
            get_config("directory", "server") + "/itemtransformrecipedata_weapon.xml";
    }

    if jewelry_server_path == "" {
        return Err("Error! 未选择进化类型".to_string());
    }

    if is_file(jewelry_server_path.as_str()) == false {
        return Err("Error! 未找到当前文件！".to_string());
        // // info!("Error! 未找到当前文件！")
    }

    let mut file = File::open(jewelry_server_path.clone()).unwrap();

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut output = String::new();

    //正则表达式匹配alias的值
    let re_alias = Regex::new(r#"alias="([^"]+)""#).unwrap();

    let mut is_edit = false;

    // 遍历每一行来判断是否在更新列表内
    for line in buffer.lines() {
        is_edit = false;
        if line.to_string().contains("alias") {
            if let Some(captures) = re_alias.captures(line) {
                let alias = captures.get(1).unwrap().as_str();
                // // info!("alias{}", alias);
                // 判断当前alias是否在更新列表内 否则跳过此次循环
                if is_in_update_item_list(update_item_list, alias.to_owned()) == false {
                    let new_line = line.clone();
                    output.push_str(&format!("{}\n", new_line));
                    continue;
                }
                // 获取update_item_list内alias为alias的值
                let update_item = update_item_list
                    .upgrade_jewelry_list
                    .iter()
                    .find(|&x| x.alias == alias.to_owned())
                    .unwrap();

                let new_record = upgrade_jewelry_to_string(update_item.clone());

                let new_line = line.replace(line, &new_record);

                output.push_str(&format!("{}\n", new_line));
                is_edit = true;
                // // info!("new_line:{:?}", new_line);
            }
        }
        if is_edit == false {
            let new_line = line.clone();
            output.push_str(&format!("{}\n", new_line));
        }
    }
    // // info!("buffer:{:?}", output);
    let mut output_file = File::create(jewelry_server_path).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();
    Ok(())
}

pub fn save_jewelry_client(
    update_item_list: &mut UpgradeJewelryList,
    evolutionary_algorithm: &mut EvolutionaryAlgorithm,
) -> Result<(), String> {
    if is_path() == false {
        return Err("Error! 未找到当前目录！".to_string());
        // // info!("Error! 未找到当前目录！")
    }

    let jewelry_client_path = get_config("directory", "client") + "/datafile_177.xml";

    if is_file(jewelry_client_path.as_str()) == false {
        return Err("Error! 未找到当前文件！".to_string());
        // // info!("Error! 未找到当前文件！")
    }

    let mut file = File::open(jewelry_client_path.clone()).unwrap();

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut output = String::new();
    let mut line_count = 1;
    let mut line_count_edit = 0;
    let mut edit_fixed_id = String::new();
    let mut is_edit = false;
    // 先把更新列表数据内的数据更新然后匹配文件的内容进行替换
    for i in &mut update_item_list.upgrade_jewelry_list {
        let old_v = num_to_byte_for_string(i.datafile_data.clone().unwrap(), 8, true);

        let mut items: Vec<String> = old_v.chars().map(|c| c.to_string()).collect();
        // // info!("items{:?}", items);

        //更改祭品leix
        let sub_type = "9F".to_string();
        let sub_type_array: Vec<String> = sub_type.chars().map(|c| c.to_string()).collect();
        // // info!("sub_type_array{:?}", sub_type_array);
        for j in 0..2 {
            items[128 + j.clone()] = sub_type_array[j].clone();
        }

        // 更改祭品
        let sub1 = evolutionary_algorithm.sub_ingredient_1_id.clone();
        let sub1_16 = num_to_byte_for_string(sub1, 8, true);
        let sub1_16_array: Vec<String> = sub1_16.chars().map(|c| c.to_string()).collect();
        // info!("sub1_16_array{:?}", sub1_16_array);
        for j in 0..8 {
            items[136 + j.clone()] = sub1_16_array[j].clone();
        }
        // 更改祭品状态
        let sub1_state = "01".to_string();
        let sub1_state_array: Vec<String> = sub1_state.chars().map(|c| c.to_string()).collect();
        // info!("sub1_state_array{:?}", sub1_state_array);
        for j in 0..2 {
            items[144 + j.clone()] = sub1_state_array[j].clone();
        }
        // 更改祭品类型
        let sub1_type = "01".to_string();
        let sub1_type_array: Vec<String> = sub1_type.chars().map(|c| c.to_string()).collect();
        // info!("sub1_type_array{:?}", sub1_type_array);
        for j in 0..2 {
            items[296 + j.clone()] = sub1_type_array[j].clone();
        }

        //更改祭品最小等级
        let sub1_min_level = "01".to_string();
        let sub1_min_level_array: Vec<String> =
            sub1_min_level.chars().map(|c| c.to_string()).collect();
        // info!("sub1_min_level_array{:?}", sub1_min_level_array);
        for j in 0..2 {
            items[310 + j.clone()] = sub1_min_level_array[j].clone();
        }

        //更改祭品数量
        let sub1_count = evolutionary_algorithm.sub_ingredient_stack_count_1.clone();
        let sub1_count_16 = num_to_byte_for_string(sub1_count, 4, true);
        let sub1_count_array: Vec<String> = sub1_count_16.chars().map(|c| c.to_string()).collect();
        // info!("sub1_count{:?}", sub1_count);
        for j in 0..4 {
            items[324 + j.clone()] = sub1_count_array[j].clone();
        }

        // 更改祭品标题
        let sub1_title = evolutionary_algorithm.sub_ingredient_1_id.clone();
        let sub1_title_16 = num_to_byte_for_string(sub1_title, 8, true);
        let sub1_title_array: Vec<String> = sub1_title_16.chars().map(|c| c.to_string()).collect();
        // info!("sub1_title_array{:?}", sub1_title_array);
        for j in 0..8 {
            items[464 + j.clone()] = sub1_title_array[j].clone();
        }
        // 更改祭品标题状态
        let sub1_title_state = "01".to_string();
        let sub1_title_state_array: Vec<String> =
            sub1_title_state.chars().map(|c| c.to_string()).collect();
        // info!("sub1_title_state_array{:?}", sub1_title_state_array);
        for j in 0..2 {
            items[472 + j.clone()] = sub1_title_state_array[j].clone();
        }

        //更改成长成本
        let money_cost = evolutionary_algorithm.money_cost.clone();
        let money_cost_16 = num_to_byte_for_string(money_cost, 8, true);
        let money_cost_array: Vec<String> = money_cost_16.chars().map(|c| c.to_string()).collect();
        // info!("money_cost_array{:?}", money_cost_array);
        for j in 0..8 {
            items[48 + j.clone()] = money_cost_array[j].clone();
        }

        if evolutionary_algorithm.fixed_ingredient_1 != "" {
            let fixed_1_id = evolutionary_algorithm.fixed_ingredient_1_id.clone();
            let fixed_1_id_16 = num_to_byte_for_string(fixed_1_id, 8, true);
            let fixed_1_id_array: Vec<String> =
                fixed_1_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_1_id_array{:?}", fixed_1_id_array);
            for j in 0..8 {
                items[584 + j.clone()] = fixed_1_id_array[j].clone();
            }

            let fixed_1_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_1
                .clone();
            let fixed_1_count_16 = num_to_byte_for_string(fixed_1_count, 4, true);
            let fixed_1_count_array: Vec<String> =
                fixed_1_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_1_count{:?}", fixed_1_count);
            for j in 0..4 {
                items[712 + j.clone()] = fixed_1_count_array[j].clone();
            }

            let fixed_1_state = "01".to_string();
            let fixed_1_state_array: Vec<String> =
                fixed_1_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_1_state_array{:?}", fixed_1_state_array);
            for j in 0..2 {
                items[592 + j.clone()] = fixed_1_state_array[j].clone();
            }
        } else {
            let fixed_1_id = "00000000".to_string();
            let fixed_1_id_array: Vec<String> = fixed_1_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_1_id_array{:?}", fixed_1_id_array);
            for j in 0..8 {
                items[584 + j.clone()] = fixed_1_id_array[j].clone();
            }

            let fixed_1_count = "0000".to_string();
            let fixed_1_count_array: Vec<String> =
                fixed_1_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_1_count{:?}", fixed_1_count);
            for j in 0..4 {
                items[712 + j.clone()] = fixed_1_count_array[j].clone();
            }

            let fixed_1_state = "00".to_string();
            let fixed_1_state_array: Vec<String> =
                fixed_1_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_1_state_array{:?}", fixed_1_state_array);
            for j in 0..2 {
                items[592 + j.clone()] = fixed_1_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_2 != "" {
            let fixed_2_id = evolutionary_algorithm.fixed_ingredient_2_id.clone();
            let fixed_2_id_16 = num_to_byte_for_string(fixed_2_id, 8, true);
            let fixed_2_id_array: Vec<String> =
                fixed_2_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_2_id_array{:?}", fixed_2_id_array);
            for j in 0..8 {
                items[600 + j.clone()] = fixed_2_id_array[j].clone();
            }

            let fixed_2_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_2
                .clone();
            let fixed_2_count_16 = num_to_byte_for_string(fixed_2_count, 4, true);
            let fixed_2_count_array: Vec<String> =
                fixed_2_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_2_count{:?}", fixed_2_count);
            for j in 0..4 {
                items[716 + j.clone()] = fixed_2_count_array[j].clone();
            }

            let fixed_2_state = "01".to_string();
            let fixed_2_state_array: Vec<String> =
                fixed_2_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_2_state_array{:?}", fixed_2_state_array);
            for j in 0..2 {
                items[608 + j.clone()] = fixed_2_state_array[j].clone();
            }
        } else {
            let fixed_2_id = "00000000".to_string();
            let fixed_2_id_array: Vec<String> = fixed_2_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_2_id_array{:?}", fixed_2_id_array);
            for j in 0..8 {
                items[600 + j.clone()] = fixed_2_id_array[j].clone();
            }

            let fixed_2_count = "0000".to_string();
            let fixed_2_count_array: Vec<String> =
                fixed_2_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_2_count{:?}", fixed_2_count);
            for j in 0..4 {
                items[716 + j.clone()] = fixed_2_count_array[j].clone();
            }

            let fixed_2_state = "00".to_string();
            let fixed_2_state_array: Vec<String> =
                fixed_2_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_2_state_array{:?}", fixed_2_state_array);
            for j in 0..2 {
                items[608 + j.clone()] = fixed_2_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_3 != "" {
            let fixed_3_id = evolutionary_algorithm.fixed_ingredient_3_id.clone();
            let fixed_3_id_16 = num_to_byte_for_string(fixed_3_id, 8, true);
            let fixed_3_id_array: Vec<String> =
                fixed_3_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_3_id_array{:?}", fixed_3_id_array);
            for j in 0..8 {
                items[616 + j.clone()] = fixed_3_id_array[j].clone();
            }

            let fixed_3_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_3
                .clone();
            let fixed_3_count_16 = num_to_byte_for_string(fixed_3_count, 4, true);
            let fixed_3_count_array: Vec<String> =
                fixed_3_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_3_count{:?}", fixed_3_count);
            for j in 0..4 {
                items[720 + j.clone()] = fixed_3_count_array[j].clone();
            }

            let fixed_3_state = "01".to_string();
            let fixed_3_state_array: Vec<String> =
                fixed_3_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_3_state_array{:?}", fixed_3_state_array);
            for j in 0..2 {
                items[624 + j.clone()] = fixed_3_state_array[j].clone();
            }
        } else {
            let fixed_3_id = "00000000".to_string();
            let fixed_3_id_array: Vec<String> = fixed_3_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_3_id_array{:?}", fixed_3_id_array);
            for j in 0..8 {
                items[616 + j.clone()] = fixed_3_id_array[j].clone();
            }

            let fixed_3_count = "0000".to_string();
            let fixed_3_count_array: Vec<String> =
                fixed_3_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_3_count{:?}", fixed_3_count);
            for j in 0..4 {
                items[720 + j.clone()] = fixed_3_count_array[j].clone();
            }

            let fixed_3_state = "00".to_string();
            let fixed_3_state_array: Vec<String> =
                fixed_3_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_3_state_array{:?}", fixed_3_state_array);
            for j in 0..2 {
                items[624 + j.clone()] = fixed_3_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_4 != "" {
            let fixed_4_id = evolutionary_algorithm.fixed_ingredient_4_id.clone();
            let fixed_4_id_16 = num_to_byte_for_string(fixed_4_id, 8, true);
            let fixed_4_id_array: Vec<String> =
                fixed_4_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_4_id_array{:?}", fixed_4_id_array);
            for j in 0..8 {
                items[632 + j.clone()] = fixed_4_id_array[j].clone();
            }

            let fixed_4_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_4
                .clone();
            let fixed_4_count_16 = num_to_byte_for_string(fixed_4_count, 4, true);
            let fixed_4_count_array: Vec<String> =
                fixed_4_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_4_count{:?}", fixed_4_count);
            for j in 0..4 {
                items[724 + j.clone()] = fixed_4_count_array[j].clone();
            }

            let fixed_4_state = "01".to_string();
            let fixed_4_state_array: Vec<String> =
                fixed_4_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_4_state_array{:?}", fixed_4_state_array);
            for j in 0..2 {
                items[640 + j.clone()] = fixed_4_state_array[j].clone();
            }
        } else {
            let fixed_4_id = "00000000".to_string();
            let fixed_4_id_array: Vec<String> = fixed_4_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_4_id_array{:?}", fixed_4_id_array);
            for j in 0..8 {
                items[632 + j.clone()] = fixed_4_id_array[j].clone();
            }

            let fixed_4_count = "0000".to_string();
            let fixed_4_count_array: Vec<String> =
                fixed_4_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_4_count{:?}", fixed_4_count);
            for j in 0..4 {
                items[724 + j.clone()] = fixed_4_count_array[j].clone();
            }

            let fixed_4_state = "00".to_string();
            let fixed_4_state_array: Vec<String> =
                fixed_4_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_4_state_array{:?}", fixed_4_state_array);
            for j in 0..2 {
                items[640 + j.clone()] = fixed_4_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_5 != "" {
            let fixed_5_id = evolutionary_algorithm.fixed_ingredient_5_id.clone();
            let fixed_5_id_16 = num_to_byte_for_string(fixed_5_id, 8, true);
            let fixed_5_id_array: Vec<String> =
                fixed_5_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_5_id_array{:?}", fixed_5_id_array);
            for j in 0..8 {
                items[648 + j.clone()] = fixed_5_id_array[j].clone();
            }

            let fixed_5_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_5
                .clone();
            let fixed_5_count_16 = num_to_byte_for_string(fixed_5_count, 4, true);
            let fixed_5_count_array: Vec<String> =
                fixed_5_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_5_count{:?}", fixed_5_count);
            for j in 0..4 {
                items[728 + j.clone()] = fixed_5_count_array[j].clone();
            }

            let fixed_5_state = "01".to_string();
            let fixed_5_state_array: Vec<String> =
                fixed_5_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_5_state_array{:?}", fixed_5_state_array);
            for j in 0..2 {
                items[656 + j.clone()] = fixed_5_state_array[j].clone();
            }
        } else {
            let fixed_5_id = "00000000".to_string();
            let fixed_5_id_array: Vec<String> = fixed_5_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_5_id_array{:?}", fixed_5_id_array);
            for j in 0..8 {
                items[648 + j.clone()] = fixed_5_id_array[j].clone();
            }

            let fixed_5_count = "0000".to_string();
            let fixed_5_count_array: Vec<String> =
                fixed_5_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_5_count{:?}", fixed_5_count);
            for j in 0..4 {
                items[728 + j.clone()] = fixed_5_count_array[j].clone();
            }

            let fixed_5_state = "00".to_string();
            let fixed_5_state_array: Vec<String> =
                fixed_5_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_5_state_array{:?}", fixed_5_state_array);
            for j in 0..2 {
                items[656 + j.clone()] = fixed_5_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_6 != "" {
            let fixed_6_id = evolutionary_algorithm.fixed_ingredient_6_id.clone();
            let fixed_6_id_16 = num_to_byte_for_string(fixed_6_id, 8, true);
            let fixed_6_id_array: Vec<String> =
                fixed_6_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_6_id_array{:?}", fixed_6_id_array);
            for j in 0..8 {
                items[664 + j.clone()] = fixed_6_id_array[j].clone();
            }

            let fixed_6_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_6
                .clone();
            let fixed_6_count_16 = num_to_byte_for_string(fixed_6_count, 4, true);
            let fixed_6_count_array: Vec<String> =
                fixed_6_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_6_count{:?}", fixed_6_count);
            for j in 0..4 {
                items[732 + j.clone()] = fixed_6_count_array[j].clone();
            }

            let fixed_6_state = "01".to_string();
            let fixed_6_state_array: Vec<String> =
                fixed_6_state.chars().map(|c| c.to_string()).collect();
            for j in 0..2 {
                items[672 + j.clone()] = fixed_6_state_array[j].clone();
            }
        } else {
            let fixed_6_id = "00000000".to_string();
            let fixed_6_id_array: Vec<String> = fixed_6_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_6_id_array{:?}", fixed_6_id_array);
            for j in 0..8 {
                items[664 + j.clone()] = fixed_6_id_array[j].clone();
            }

            let fixed_6_count = "0000".to_string();
            let fixed_6_count_array: Vec<String> =
                fixed_6_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_6_count{:?}", fixed_6_count);
            for j in 0..4 {
                items[732 + j.clone()] = fixed_6_count_array[j].clone();
            }

            let fixed_6_state = "00".to_string();
            let fixed_6_state_array: Vec<String> =
                fixed_6_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_6_state_array{:?}", fixed_6_state_array);

            for j in 0..2 {
                items[672 + j.clone()] = fixed_6_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_7 != "" {
            let fixed_7_id = evolutionary_algorithm.fixed_ingredient_7_id.clone();
            let fixed_7_id_16 = num_to_byte_for_string(fixed_7_id, 8, true);
            let fixed_7_id_array: Vec<String> =
                fixed_7_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_7_id_array{:?}", fixed_7_id_array);
            for j in 0..8 {
                items[680 + j.clone()] = fixed_7_id_array[j].clone();
            }

            let fixed_7_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_7
                .clone();
            let fixed_7_count_16 = num_to_byte_for_string(fixed_7_count, 4, true);
            let fixed_7_count_array: Vec<String> =
                fixed_7_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_7_count{:?}", fixed_7_count);
            for j in 0..4 {
                items[736 + j.clone()] = fixed_7_count_array[j].clone();
            }

            let fixed_7_state = "01".to_string();
            let fixed_7_state_array: Vec<String> =
                fixed_7_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_7_state_array{:?}", fixed_7_state_array);
            for j in 0..2 {
                items[688 + j.clone()] = fixed_7_state_array[j].clone();
            }
        } else {
            let fixed_7_id = "00000000".to_string();
            let fixed_7_id_array: Vec<String> = fixed_7_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_7_id_array{:?}", fixed_7_id_array);
            for j in 0..8 {
                items[680 + j.clone()] = fixed_7_id_array[j].clone();
            }

            let fixed_7_count = "0000".to_string();
            let fixed_7_count_array: Vec<String> =
                fixed_7_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_7_count{:?}", fixed_7_count);
            for j in 0..4 {
                items[736 + j.clone()] = fixed_7_count_array[j].clone();
            }

            let fixed_7_state = "00".to_string();
            let fixed_7_state_array: Vec<String> =
                fixed_7_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_7_state_array{:?}", fixed_7_state_array);
            for j in 0..2 {
                items[688 + j.clone()] = fixed_7_state_array[j].clone();
            }
        }

        if evolutionary_algorithm.fixed_ingredient_8 != "" {
            let fixed_8_id = evolutionary_algorithm.fixed_ingredient_8_id.clone();
            let fixed_8_id_16 = num_to_byte_for_string(fixed_8_id, 8, true);
            let fixed_8_id_array: Vec<String> =
                fixed_8_id_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_8_id_array{:?}", fixed_8_id_array);
            for j in 0..8 {
                items[696 + j.clone()] = fixed_8_id_array[j].clone();
            }

            let fixed_8_count = evolutionary_algorithm
                .fixed_ingredient_stack_count_8
                .clone();
            let fixed_8_count_16 = num_to_byte_for_string(fixed_8_count, 4, true);
            let fixed_8_count_array: Vec<String> =
                fixed_8_count_16.chars().map(|c| c.to_string()).collect();
            // info!("fixed_8_count{:?}", fixed_8_count);
            for j in 0..4 {
                items[740 + j.clone()] = fixed_8_count_array[j].clone();
            }

            let fixed_8_state = "01".to_string();
            let fixed_8_state_array: Vec<String> =
                fixed_8_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_8_state_array{:?}", fixed_8_state_array);
            for j in 0..2 {
                items[704 + j.clone()] = fixed_8_state_array[j].clone();
            }
        } else {
            let fixed_8_id = "00000000".to_string();
            let fixed_8_id_array: Vec<String> = fixed_8_id.chars().map(|c| c.to_string()).collect();
            // info!("fixed_8_id_array{:?}", fixed_8_id_array);
            for j in 0..8 {
                items[696 + j.clone()] = fixed_8_id_array[j].clone();
            }

            let fixed_8_count = "0000".to_string();
            let fixed_8_count_array: Vec<String> =
                fixed_8_count.chars().map(|c| c.to_string()).collect();
            // info!("fixed_8_count{:?}", fixed_8_count);
            for j in 0..4 {
                items[740 + j.clone()] = fixed_8_count_array[j].clone();
            }

            let fixed_8_state = "00".to_string();
            let fixed_8_state_array: Vec<String> =
                fixed_8_state.chars().map(|c| c.to_string()).collect();
            // info!("fixed_8_state_array{:?}", fixed_8_state_array);
            for j in 0..2 {
                items[704 + j.clone()] = fixed_8_state_array[j].clone();
            }
        }

        // // info!("items{:?}", items);
        let new_v = byte_to_num_for_string(items.concat(), true);
        let new_v = new_v.trim_end_matches(',');
        i.datafile_data = Some(new_v.to_string());
        // // info!("new_v{:?}", new_v);
    }

    //遍历177文件 如果xml是filed在更新列表中 那就进行数据替换
    for line in buffer.lines() {
        is_edit = false;
        if line_count_edit == line_count {
            for t in &update_item_list.upgrade_jewelry_list {
                if t.filed_id == Some(edit_fixed_id.clone()) {
                    let new_data = t.datafile_data.clone();
                    // // info!("new_data{:?}", new_data);
                    let new_line = format!(
                        "              <data>{:?}</data>",
                        new_data.unwrap_or_default().to_string()
                    );
                    let new_line = new_line.replace("\"", "");
                    output.push_str(&format!("{}\n", new_line));
                    is_edit = true;
                    edit_fixed_id = "".to_string();
                    break;
                }
            }
        } else {
            if line.to_string().contains("<field id=") {
                for j in &update_item_list.upgrade_jewelry_list {
                    let fixed_id = j.filed_id.clone();
                    let fixed_id_t_str =
                        format!("field id=\"{}\"", fixed_id.unwrap_or_default().to_string());
                    if line.to_string().contains(&fixed_id_t_str) {
                        edit_fixed_id = j.filed_id.clone().unwrap_or_default().to_string();
                        line_count_edit = line_count + 1;
                    }
                }
            }
        }
        if is_edit == false {
            let new_line = line.clone();
            output.push_str(&format!("{}\n", new_line));
        }
        line_count += 1;
    }
    // // info!("output{:?}", output);
    let mut output_file = File::create(jewelry_client_path).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();

    Ok(())
}

pub fn num_to_byte_for_string(i: String, data_len: usize, shift: bool) -> String {
    let i = i.replace("\"", "");
    let data_array: Vec<&str> = i.split(",").collect();
    let mut count = 0;
    // // info!("dataArray{:?}", data_array);
    let mut new_data_array = String::new();
    for byte_str in data_array {
        // info!("num_str{:?}", byte_str);
        let mut hex_str = format!("{:X}", byte_str.parse::<i32>().unwrap());
        count += 1;
        let new_str_len = hex_str.len();
        if new_str_len < data_len {
            for _ in 0..(data_len - new_str_len) {
                hex_str.insert_str(0, "0");
            }
        }
        // info!("hex_str{:?}", hex_str);
        if shift {
            let tmp_str_array: Vec<String> = hex_str.chars().map(|c| c.to_string()).collect();
            // // info!("tmp_str_array{:?}", tmp_str_array);
            match data_len {
                2 => {
                    new_data_array.push_str(&tmp_str_array[0]);
                    new_data_array.push_str(&tmp_str_array[1]);
                }
                4 => {
                    new_data_array.push_str(&tmp_str_array[2]);
                    new_data_array.push_str(&tmp_str_array[3]);
                    new_data_array.push_str(&tmp_str_array[0]);
                    new_data_array.push_str(&tmp_str_array[1]);
                }
                8 => {
                    new_data_array.push_str(&tmp_str_array[6]);
                    new_data_array.push_str(&tmp_str_array[7]);
                    new_data_array.push_str(&tmp_str_array[4]);
                    new_data_array.push_str(&tmp_str_array[5]);
                    new_data_array.push_str(&tmp_str_array[2]);
                    new_data_array.push_str(&tmp_str_array[3]);
                    new_data_array.push_str(&tmp_str_array[0]);
                    new_data_array.push_str(&tmp_str_array[1]);
                }
                _ => {}
            }
        } else {
            new_data_array.push_str(&hex_str);
        }
    }
    // info!("new_data_array:{}", new_data_array);
    new_data_array
}

pub fn byte_to_num_for_string(i: String, shift: bool) -> String {
    // // info!("byte_to_num_for_string:{}", i);
    let old_data_array: Vec<String> = i.clone().chars().map(|c| c.to_string()).collect();
    let mut new_data_array = String::new();
    if i.len() <= 8 {
        if shift {
            let old_data_array_2: Vec<String> = i.chars().map(|c| c.to_string()).collect();
            match old_data_array_2.len() {
                2 => {
                    new_data_array.push_str(&old_data_array_2[0]);
                    new_data_array.push_str(&old_data_array_2[1]);
                }
                4 => {
                    new_data_array.push_str(&old_data_array_2[2]);
                    new_data_array.push_str(&old_data_array_2[3]);
                    new_data_array.push_str(&old_data_array_2[0]);
                    new_data_array.push_str(&old_data_array_2[1]);
                }
                8 => {
                    new_data_array.push_str(&old_data_array_2[6]);
                    new_data_array.push_str(&old_data_array_2[7]);
                    new_data_array.push_str(&old_data_array_2[4]);
                    new_data_array.push_str(&old_data_array_2[5]);
                    new_data_array.push_str(&old_data_array_2[2]);
                    new_data_array.push_str(&old_data_array_2[3]);
                    new_data_array.push_str(&old_data_array_2[0]);
                    new_data_array.push_str(&old_data_array_2[1]);
                }
                _ => {}
            }
        }
        //十六进制转十进制
        let int_d = i64::from_str_radix(&new_data_array, 16).unwrap();
        new_data_array = int_d.to_string();
    } else {
        let mut q = 0;
        loop {
            let p = q.clone();
            let mut tmp_str = String::new();
            if p < old_data_array.len() {
                if shift {
                    tmp_str.push_str(&old_data_array[p + 6]);
                    tmp_str.push_str(&old_data_array[p + 7]);
                    tmp_str.push_str(&old_data_array[p + 4]);
                    tmp_str.push_str(&old_data_array[p + 5]);
                    tmp_str.push_str(&old_data_array[p + 2]);
                    tmp_str.push_str(&old_data_array[p + 3]);
                    tmp_str.push_str(&old_data_array[p + 0]);
                    tmp_str.push_str(&old_data_array[p + 1]);
                } else {
                    tmp_str.push_str(&old_data_array[p + 0]);
                    tmp_str.push_str(&old_data_array[p + 1]);
                    tmp_str.push_str(&old_data_array[p + 2]);
                    tmp_str.push_str(&old_data_array[p + 3]);
                    tmp_str.push_str(&old_data_array[p + 4]);
                    tmp_str.push_str(&old_data_array[p + 5]);
                    tmp_str.push_str(&old_data_array[p + 6]);
                    tmp_str.push_str(&old_data_array[p + 7]);
                }
                let int_d = i64::from_str_radix(&tmp_str, 16).unwrap();
                new_data_array.push_str(int_d.to_string().as_str());
                new_data_array.push_str(",");
                q = p + 8;
            } else {
                break;
            }
        }
    }
    new_data_array
}

// 分割键值对
pub fn re_get_kv(str: &str) -> Vec<(&str, &str)> {
    let mut kv_list = vec![];
    let re_kv = Regex::new(r#"([\w-]+)="([^"]+)""#).unwrap();
    for captures in re_kv.captures_iter(str) {
        let key = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str();
        kv_list.push((key, value));
    }
    kv_list
}

pub fn upgrade_jewelry_to_string(item: UpgradeJewelry) -> String {
    let mut result = format!(r#"	<record alias="{}""#, item.alias);
    if let Some(i) = &item.category {
        result.push_str(&format!(r#" category="{}""#, i));
    }
    if let Some(i) = &item.consume_fixed_ingredient {
        result.push_str(&format!(r#" consume-fixed-ingredient="{}""#, i));
    }
    if let Some(i) = &item.consume_sub_ingredient {
        result.push_str(&format!(r#" consume-sub-ingredient="{}""#, i));
    }
    if let Some(i) = &item.fail_effect {
        result.push_str(&format!(r#" fail-effect="{}""#, i));
    }
    if let Some(i) = &item.keep_main_ingredient_weapon_appearance {
        result.push_str(&format!(
            r#" keep-main-ingredient-weapon-appearance="{}""#,
            i
        ));
    }
    if let Some(i) = &item.keep_main_ingredient_weapon_gem_slot {
        result.push_str(&format!(r#" keep-main-ingredient-weapon-gem-slot="{}""#, i));
    }
    if let Some(i) = &item.keep_main_ingredient_spirit {
        result.push_str(&format!(r#" keep-main-ingredient-spirit="{}""#, i));
    }
    if let Some(i) = &item.main_ingredient {
        result.push_str(&format!(r#" main-ingredient="{}""#, i));
    }
    if let Some(i) = &item.main_ingredient_condition_type {
        result.push_str(&format!(r#" main-ingredient-condition-type="{}""#, i));
    }
    if let Some(i) = &item.main_ingredient_min_level {
        result.push_str(&format!(r#" main-ingredient-min-level="{}""#, i));
    }
    if let Some(i) = &item.main_ingredient_stack_count {
        result.push_str(&format!(r#" main-ingredient-stack-count="{}""#, i));
    }
    if let Some(i) = &item.money_cost {
        result.push_str(&format!(r#" money-cost="{}""#, i));
    }
    if let Some(i) = &item.random_failure_mileage_distribution_type {
        result.push_str(&format!(
            r#" random-failure-mileage-distribution-type="{}""#,
            i
        ));
    }
    if let Some(i) = &item.random_item_1 {
        result.push_str(&format!(r#" random-item-1="{}""#, i));
    }
    if let Some(i) = &item.random_item_2 {
        result.push_str(&format!(r#" random-item-2="{}""#, i));
    }
    if let Some(i) = &item.random_item_3 {
        result.push_str(&format!(r#" random-item-3="{}""#, i));
    }
    if let Some(i) = &item.random_item_4 {
        result.push_str(&format!(r#" random-item-4="{}""#, i));
    }
    if let Some(i) = &item.random_item_5 {
        result.push_str(&format!(r#" random-item-5="{}""#, i));
    }
    if let Some(i) = &item.random_item_6 {
        result.push_str(&format!(r#" random-item-6="{}""#, i));
    }
    if let Some(i) = &item.random_item_7 {
        result.push_str(&format!(r#" random-item-7="{}""#, i));
    }
    if let Some(i) = &item.random_item_8 {
        result.push_str(&format!(r#" random-item-8="{}""#, i));
    }
    if let Some(i) = &item.random_item_9 {
        result.push_str(&format!(r#" random-item-9="{}""#, i));
    }
    if let Some(i) = &item.random_item_10 {
        result.push_str(&format!(r#" random-item-10="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_1 {
        result.push_str(&format!(r#" random-item-select-prop-weight-1="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_2 {
        result.push_str(&format!(r#" random-item-select-prop-weight-2="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_3 {
        result.push_str(&format!(r#" random-item-select-prop-weight-3="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_4 {
        result.push_str(&format!(r#" random-item-select-prop-weight-4="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_5 {
        result.push_str(&format!(r#" random-item-select-prop-weight-5="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_6 {
        result.push_str(&format!(r#" random-item-select-prop-weight-6="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_7 {
        result.push_str(&format!(r#" random-item-select-prop-weight-7="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_8 {
        result.push_str(&format!(r#" random-item-select-prop-weight-8="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_9 {
        result.push_str(&format!(r#" random-item-select-prop-weight-9="{}""#, i));
    }
    if let Some(i) = &item.random_item_select_prop_weight_10 {
        result.push_str(&format!(r#" random-item-select-prop-weight-10="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_1 {
        result.push_str(&format!(r#" random-item-stack-count-1="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_2 {
        result.push_str(&format!(r#" random-item-stack-count-2="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_3 {
        result.push_str(&format!(r#" random-item-stack-count-3="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_4 {
        result.push_str(&format!(r#" random-item-stack-count-4="{}""#, i));
    }

    if let Some(i) = &item.random_item_stack_count_5 {
        result.push_str(&format!(r#" random-item-stack-count-5="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_6 {
        result.push_str(&format!(r#" random-item-stack-count-6="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_7 {
        result.push_str(&format!(r#" random-item-stack-count-7="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_8 {
        result.push_str(&format!(r#" random-item-stack-count-8="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_9 {
        result.push_str(&format!(r#" random-item-stack-count-9="{}""#, i));
    }
    if let Some(i) = &item.random_item_stack_count_10 {
        result.push_str(&format!(r#" random-item-stack-count-10="{}""#, i));
    }
    if let Some(i) = &item.random_item_success_probability {
        result.push_str(&format!(r#" random-item-success-probability="{}""#, i));
    }
    if let Some(i) = &item.random_item_total_count {
        result.push_str(&format!(r#" random-item-total-count="{}""#, i));
    }
    if let Some(i) = &item.random_retry_cost {
        result.push_str(&format!(r#" random-retry-cost="{}""#, i));
    }
    // if let Some(i) = &item.random_failure_mileage_save {
    //     result.push_str(&format!(r#" random-failure-mileage-save="{}""#, i));
    // }
    if let Some(i) = &item.required_inven_capacity {
        result.push_str(&format!(r#" required-inven-capacity="{}""#, i));
    }
    if let Some(i) = &item.sub_ingredient_1 {
        result.push_str(&format!(r#" sub-ingredient-1="{}""#, i));
    }
    if let Some(i) = &item.sub_ingredient_condition_type_1 {
        result.push_str(&format!(r#" sub-ingredient-condition-type-1="{}""#, i));
    }
    if let Some(i) = &item.sub_ingredient_min_level_1 {
        result.push_str(&format!(r#" sub-ingredient-min-level-1="{}""#, i));
    }
    if let Some(i) = &item.sub_ingredient_stack_count_1 {
        result.push_str(&format!(r#" sub-ingredient-stack-count-1="{}""#, i));
    }
    if let Some(i) = &item.title_item {
        result.push_str(&format!(r#" title-item="{}""#, i));
    }
    if let Some(i) = &item.use_random {
        result.push_str(&format!(r#" use-random="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_1 {
        result.push_str(&format!(r#" fixed-ingredient-1="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_2 {
        result.push_str(&format!(r#" fixed-ingredient-2="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_3 {
        result.push_str(&format!(r#" fixed-ingredient-3="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_4 {
        result.push_str(&format!(r#" fixed-ingredient-4="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_5 {
        result.push_str(&format!(r#" fixed-ingredient-5="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_6 {
        result.push_str(&format!(r#" fixed-ingredient-6="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_7 {
        result.push_str(&format!(r#" fixed-ingredient-7="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_8 {
        result.push_str(&format!(r#" fixed-ingredient-8="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_1 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-1="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_2 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-2="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_3 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-3="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_4 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-4="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_5 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-5="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_6 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-6="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_7 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-7="{}""#, i));
    }
    if let Some(i) = &item.fixed_ingredient_stack_count_8 {
        result.push_str(&format!(r#" fixed-ingredient-stack-count-8="{}""#, i));
    }
    if let Some(i) = &item.normal_item_1 {
        result.push_str(&format!(r#" normal-item-1="{}""#, i));
    }
    if let Some(i) = &item.normal_item_select_count {
        result.push_str(&format!(r#" normal-item-select-count="{}""#, i));
    }
    if let Some(i) = &item.normal_item_stack_count_1 {
        result.push_str(&format!(r#" normal-item-stack-count-1="{}""#, i));
    }
    if let Some(i) = &item.normal_item_success_probability {
        result.push_str(&format!(r#" normal-item-success-probability="{}""#, i));
    }
    if let Some(i) = &item.normal_item_total_count {
        result.push_str(&format!(r#" normal-item-total-count="{}""#, i));
    }
    if let Some(i) = &item.upgrade_grocery {
        result.push_str(&format!(r#" upgrade-grocery="{}""#, i));
    }
    result.push_str(r#" />"#);
    // // info!("{}", result);
    result
}
