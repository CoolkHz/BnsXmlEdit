use std::{
    fs::File,
    io::{BufReader, Read},
};

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::{
    page::main_page::Algorithm,
    util::tool::{get_config, is_file, is_path},
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename = "table")]
pub struct UpgradeJewelryList {
    pub count: Option<usize>,
    #[serde(rename = "record")]
    pub upgrade_jewelry_list: Vec<UpgradeJewelry>,
}

impl Default for UpgradeJewelryList {
    fn default() -> Self {
        Self {
            count: Some(0),
            upgrade_jewelry_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpgradeJewelryClient {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    #[serde(rename = "money-cost")]
    pub money_cost: Option<String>, //金钱成本
    #[serde(rename = "sub-ingredient-1")]
    pub sub_ingredient_1: Option<String>, // 祭品1
    #[serde(rename = "sub-ingredient-state-1")]
    pub sub_ingredient_state_1: Option<String>,
    #[serde(rename = "sub-ingredient-condition-type-1")]
    pub sub_ingredient_condition_type_1: Option<String>, // 祭品1条件类型
    #[serde(rename = "sub-ingredient-min-level-1")]
    pub sub_ingredient_min_level_1: Option<String>, // 祭品1最小等级
    #[serde(rename = "sub-ingredient-stack-count-1")]
    pub sub_ingredient_stack_count_1: Option<String>, // 祭品1数量
    #[serde(rename = "sub-ingredient-title-name-1")]
    pub sub_ingredient_title_name_1: Option<String>,
    #[serde(rename = "sub-ingredient-title-item-1")]
    pub sub_ingredient_title_item_1: Option<String>,
    #[serde(rename = "sub-ingredient-title-item-state-1")]
    pub sub_ingredient_title_item_state_1: Option<String>,
    #[serde(rename = "consume-sub-ingredient")]
    pub consume_sub_ingredient: Option<String>, //是否消耗祭品
    #[serde(rename = "fixed-ingredient-1")]
    pub fixed_ingredient_1: Option<String>,
    #[serde(rename = "fixed-ingredient-state-1")]
    pub fixed_ingredient_state_1: Option<String>,
    #[serde(rename = "fixed-ingredient-2")]
    pub fixed_ingredient_2: Option<String>,
    #[serde(rename = "fixed-ingredient-state-2")]
    pub fixed_ingredient_state_2: Option<String>,
    #[serde(rename = "fixed-ingredient-3")]
    pub fixed_ingredient_3: Option<String>,
    #[serde(rename = "fixed-ingredient-state-3")]
    pub fixed_ingredient_state_3: Option<String>,
    #[serde(rename = "fixed-ingredient-4")]
    pub fixed_ingredient_4: Option<String>,
    #[serde(rename = "fixed-ingredient-state-4")]
    pub fixed_ingredient_state_4: Option<String>,
    #[serde(rename = "fixed-ingredient-5")]
    pub fixed_ingredient_5: Option<String>,
    #[serde(rename = "fixed-ingredient-state-5")]
    pub fixed_ingredient_state_5: Option<String>,
    #[serde(rename = "fixed-ingredient-6")]
    pub fixed_ingredient_6: Option<String>,
    #[serde(rename = "fixed-ingredient-state-6")]
    pub fixed_ingredient_state_6: Option<String>,
    #[serde(rename = "fixed-ingredient-7")]
    pub fixed_ingredient_7: Option<String>,
    #[serde(rename = "fixed-ingredient-state-7")]
    pub fixed_ingredient_state_7: Option<String>,
    #[serde(rename = "fixed-ingredient-8")]
    pub fixed_ingredient_8: Option<String>,
    #[serde(rename = "fixed-ingredient-state-8")]
    pub fixed_ingredient_state_8: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-1")]
    pub fixed_ingredient_stack_count_1: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-2")]
    pub fixed_ingredient_stack_count_2: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-3")]
    pub fixed_ingredient_stack_count_3: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-4")]
    pub fixed_ingredient_stack_count_4: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-5")]
    pub fixed_ingredient_stack_count_5: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-6")]
    pub fixed_ingredient_stack_count_6: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-7")]
    pub fixed_ingredient_stack_count_7: Option<String>,
    #[serde(rename = "fixed-ingredient-stack-count-8")]
    pub fixed_ingredient_stack_count_8: Option<String>,
    #[serde(rename = "consume-fixed-ingredient")]
    pub consume_fixed_ingredient: Option<String>, //是否消耗材料
    #[serde(rename = "title-item")]
    pub title_item: Option<String>,
    #[serde(rename = "title-item-state")]
    pub title_item_state: Option<String>,
    #[serde(rename = "warning")]
    pub warning: Option<String>,
    #[serde(rename = "sub-ingredient-1-type")]
    pub sub_ingredient_1_type: Option<String>,
}

impl Default for UpgradeJewelryClient {
    fn default() -> Self {
        Self {
            id: None,
            alias: None,
            money_cost: None,
            sub_ingredient_1: None,
            sub_ingredient_state_1: None,
            sub_ingredient_condition_type_1: None,
            sub_ingredient_min_level_1: None,
            sub_ingredient_stack_count_1: None,
            sub_ingredient_title_name_1: None,
            sub_ingredient_title_item_1: None,
            sub_ingredient_title_item_state_1: None,
            consume_sub_ingredient: None,
            fixed_ingredient_1: None,
            fixed_ingredient_state_1: None,
            fixed_ingredient_2: None,
            fixed_ingredient_state_2: None,
            fixed_ingredient_3: None,
            fixed_ingredient_state_3: None,
            fixed_ingredient_4: None,
            fixed_ingredient_state_4: None,
            fixed_ingredient_5: None,
            fixed_ingredient_state_5: None,
            fixed_ingredient_6: None,
            fixed_ingredient_state_6: None,
            fixed_ingredient_7: None,
            fixed_ingredient_state_7: None,
            fixed_ingredient_8: None,
            fixed_ingredient_state_8: None,
            fixed_ingredient_stack_count_1: None,
            fixed_ingredient_stack_count_2: None,
            fixed_ingredient_stack_count_3: None,
            fixed_ingredient_stack_count_4: None,
            fixed_ingredient_stack_count_5: None,
            fixed_ingredient_stack_count_6: None,
            fixed_ingredient_stack_count_7: None,
            fixed_ingredient_stack_count_8: None,
            consume_fixed_ingredient: None,
            title_item: None,
            title_item_state: None,
            warning: None,
            sub_ingredient_1_type: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpgradeJewelry {
    pub tran: Option<String>,
    #[serde(rename = "@alias")]
    pub alias: String,
    #[serde(rename = "@category")]
    pub category: Option<String>,
    #[serde(rename = "@consume-fixed-ingredient")]
    pub consume_fixed_ingredient: Option<String>, //是否消耗材料
    #[serde(rename = "@consume-sub-ingredient")]
    pub consume_sub_ingredient: Option<String>, //是否消耗祭品
    #[serde(rename = "@fail-effect")] //
    pub fail_effect: Option<String>,
    #[serde(rename = "@keep-main-ingredient-weapon-appearance")]
    pub keep_main_ingredient_weapon_appearance: Option<String>,
    #[serde(rename = "@keep-main-ingredient-weapon-gem-slot")]
    pub keep_main_ingredient_weapon_gem_slot: Option<String>,
    #[serde(rename = "@keep-main-ingredient-spirit")]
    pub keep_main_ingredient_spirit: Option<String>,
    #[serde(rename = "@main-ingredient")]
    pub main_ingredient: Option<String>,
    #[serde(rename = "@main-ingredient-condition-type")]
    pub main_ingredient_condition_type: Option<String>,
    #[serde(rename = "@main-ingredient-min-level")]
    pub main_ingredient_min_level: Option<String>,
    #[serde(rename = "@main-ingredient-stack-count")]
    pub main_ingredient_stack_count: Option<String>,
    #[serde(rename = "@money-cost")]
    pub money_cost: Option<String>, //金钱成本
    #[serde(rename = "@random-failure-mileage-distribution-type")]
    pub random_failure_mileage_distribution_type: Option<String>,
    #[serde(rename = "@random-item-1")]
    pub random_item_1: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-2")]
    pub random_item_2: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-3")]
    pub random_item_3: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-4")]
    pub random_item_4: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-5")]
    pub random_item_5: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-6")]
    pub random_item_6: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-7")]
    pub random_item_7: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-8")]
    pub random_item_8: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-9")]
    pub random_item_9: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-10")]
    pub random_item_10: Option<String>, //随机进化物品
    #[serde(rename = "@random-item-select-prop-weight-1")]
    pub random_item_select_prop_weight_1: Option<String>, //随机进化物品权重
    #[serde(rename = "@random-item-select-prop-weight-2")]
    pub random_item_select_prop_weight_2: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-3")]
    pub random_item_select_prop_weight_3: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-4")]
    pub random_item_select_prop_weight_4: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-5")]
    pub random_item_select_prop_weight_5: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-6")]
    pub random_item_select_prop_weight_6: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-7")]
    pub random_item_select_prop_weight_7: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-8")]
    pub random_item_select_prop_weight_8: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-9")]
    pub random_item_select_prop_weight_9: Option<String>,
    #[serde(rename = "@random-item-select-prop-weight-10")]
    pub random_item_select_prop_weight_10: Option<String>,
    #[serde(rename = "@random-item-stack-count-1")]
    pub random_item_stack_count_1: Option<String>, //随机进化物品数量
    #[serde(rename = "@random-item-stack-count-2")]
    pub random_item_stack_count_2: Option<String>,
    #[serde(rename = "@random-item-stack-count-3")]
    pub random_item_stack_count_3: Option<String>,
    #[serde(rename = "@random-item-stack-count-4")]
    pub random_item_stack_count_4: Option<String>,
    #[serde(rename = "@random-item-stack-count-5")]
    pub random_item_stack_count_5: Option<String>,
    #[serde(rename = "@random-item-stack-count-6")]
    pub random_item_stack_count_6: Option<String>,
    #[serde(rename = "@random-item-stack-count-7")]
    pub random_item_stack_count_7: Option<String>,
    #[serde(rename = "@random-item-stack-count-8")]
    pub random_item_stack_count_8: Option<String>,
    #[serde(rename = "@random-item-stack-count-9")]
    pub random_item_stack_count_9: Option<String>,
    #[serde(rename = "@random-item-stack-count-10")]
    pub random_item_stack_count_10: Option<String>,
    #[serde(rename = "@random-item-success-probability")]
    pub random_item_success_probability: Option<String>, //随机进化成功概率
    #[serde(rename = "@random-item-total-count")]
    pub random_item_total_count: Option<String>, //随机进化物品数量
    #[serde(rename = "@random-retry-cost")]
    pub random_retry_cost: Option<String>, //随机进化重试消耗
    #[serde(rename = "@random-failure-mileage-save")]
    pub random_failure_mileage_save: Option<String>,
    #[serde(rename = "@required-inven-capacity")]
    pub required_inven_capacity: Option<String>,
    #[serde(rename = "@sub-ingredient-1")]
    pub sub_ingredient_1: Option<String>, // 祭品1
    #[serde(rename = "@sub-ingredient-condition-type-1")]
    pub sub_ingredient_condition_type_1: Option<String>, // 祭品1条件类型
    #[serde(rename = "@sub-ingredient-min-level-1")]
    pub sub_ingredient_min_level_1: Option<String>, // 祭品1最小等级
    #[serde(rename = "@sub-ingredient-stack-count-1")]
    pub sub_ingredient_stack_count_1: Option<String>, // 祭品1数量
    #[serde(rename = "@title-item")]
    pub title_item: Option<String>,
    #[serde(rename = "@use-random")]
    pub use_random: Option<String>,
    #[serde(rename = "@fixed-ingredient-1")]
    pub fixed_ingredient_1: Option<String>,
    #[serde(rename = "@fixed-ingredient-2")]
    pub fixed_ingredient_2: Option<String>,
    #[serde(rename = "@fixed-ingredient-3")]
    pub fixed_ingredient_3: Option<String>,
    #[serde(rename = "@fixed-ingredient-4")]
    pub fixed_ingredient_4: Option<String>,
    #[serde(rename = "@fixed-ingredient-5")]
    pub fixed_ingredient_5: Option<String>,
    #[serde(rename = "@fixed-ingredient-6")]
    pub fixed_ingredient_6: Option<String>,
    #[serde(rename = "@fixed-ingredient-7")]
    pub fixed_ingredient_7: Option<String>,
    #[serde(rename = "@fixed-ingredient-8")]
    pub fixed_ingredient_8: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-1")]
    pub fixed_ingredient_stack_count_1: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-2")]
    pub fixed_ingredient_stack_count_2: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-3")]
    pub fixed_ingredient_stack_count_3: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-4")]
    pub fixed_ingredient_stack_count_4: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-5")]
    pub fixed_ingredient_stack_count_5: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-6")]
    pub fixed_ingredient_stack_count_6: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-7")]
    pub fixed_ingredient_stack_count_7: Option<String>,
    #[serde(rename = "@fixed-ingredient-stack-count-8")]
    pub fixed_ingredient_stack_count_8: Option<String>,
    #[serde(rename = "@normal-item-1")]
    pub normal_item_1: Option<String>,
    #[serde(rename = "@normal-item-select-count")]
    pub normal_item_select_count: Option<String>,
    #[serde(rename = "@normal-item-stack-count-1")]
    pub normal_item_stack_count_1: Option<String>,
    #[serde(rename = "@normal-item-success-probability")]
    pub normal_item_success_probability: Option<String>,
    #[serde(rename = "@normal-item-total-count")]
    pub normal_item_total_count: Option<String>,
    #[serde(rename = "@upgrade-grocery")]
    pub upgrade_grocery: Option<String>,
    pub todo: Option<String>,
    pub filed_id: Option<String>,
    pub datafile_data: Option<String>,
}

// impl std::fmt::Display for UpgradeJewelry {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "name=\"{}\" age=\"{}\"", self.alias, self.age)?;
//         if let Some(email) = &self.email {
//             write!(f, " email=\"{}\"", email)?;
//         }
//         Ok(())
//     }
// }

impl Default for UpgradeJewelry {
    fn default() -> Self {
        Self {
            tran: Some(String::new()),
            alias: String::new(),
            category: Some(String::new()),
            consume_fixed_ingredient: Some(String::new()), //是否消耗材料
            consume_sub_ingredient: Some(String::new()),   //是否消耗祭品
            fail_effect: Some(String::new()),
            keep_main_ingredient_weapon_appearance: Some(String::new()),
            keep_main_ingredient_weapon_gem_slot: Some(String::new()),
            keep_main_ingredient_spirit: Some(String::new()),
            main_ingredient: Some(String::new()),
            main_ingredient_condition_type: Some(String::new()),
            main_ingredient_min_level: Some(String::new()),
            main_ingredient_stack_count: Some(String::new()),
            money_cost: Some(String::new()), //金钱成本
            random_failure_mileage_distribution_type: Some(String::new()),
            random_item_1: Some(String::new()),  //随机进化物品
            random_item_2: Some(String::new()),  //随机进化物品
            random_item_3: Some(String::new()),  //随机进化物品
            random_item_4: Some(String::new()),  //随机进化物品
            random_item_5: Some(String::new()),  //随机进化物品
            random_item_6: Some(String::new()),  //随机进化物品
            random_item_7: Some(String::new()),  //随机进化物品
            random_item_8: Some(String::new()),  //随机进化物品
            random_item_9: Some(String::new()),  //随机进化物
            random_item_10: Some(String::new()), //随机进化物品
            random_item_select_prop_weight_1: Some(String::new()), //随机进化物品权重
            random_item_select_prop_weight_2: Some(String::new()),
            random_item_select_prop_weight_3: Some(String::new()),
            random_item_select_prop_weight_4: Some(String::new()),
            random_item_select_prop_weight_5: Some(String::new()),
            random_item_select_prop_weight_6: Some(String::new()),
            random_item_select_prop_weight_7: Some(String::new()),
            random_item_select_prop_weight_8: Some(String::new()),
            random_item_select_prop_weight_9: Some(String::new()),
            random_item_select_prop_weight_10: Some(String::new()),
            random_item_stack_count_1: Some(String::new()), //随机进化物品数量
            random_item_stack_count_2: Some(String::new()),
            random_item_stack_count_3: Some(String::new()),
            random_item_stack_count_4: Some(String::new()),
            random_item_stack_count_5: Some(String::new()),
            random_item_stack_count_6: Some(String::new()),
            random_item_stack_count_7: Some(String::new()),
            random_item_stack_count_8: Some(String::new()),
            random_item_stack_count_9: Some(String::new()),
            random_item_stack_count_10: Some(String::new()),
            random_item_success_probability: Some(String::new()), //随机进化成功概率
            random_item_total_count: Some(String::new()),         //随机进化物品数量
            random_retry_cost: Some(String::new()),               //随机进化重试消耗
            random_failure_mileage_save: Some(String::new()),
            required_inven_capacity: Some(String::new()),
            sub_ingredient_1: Some(String::new()), // 祭品1
            sub_ingredient_condition_type_1: Some(String::new()), // 祭品1条件类型
            sub_ingredient_min_level_1: Some(String::new()), // 祭品1最小等
            sub_ingredient_stack_count_1: Some(String::new()), // 祭品1数量
            title_item: Some(String::new()),
            use_random: Some(String::new()),
            fixed_ingredient_1: Some(String::new()),
            fixed_ingredient_2: Some(String::new()),
            fixed_ingredient_3: Some(String::new()),
            fixed_ingredient_4: Some(String::new()),
            fixed_ingredient_5: Some(String::new()),
            fixed_ingredient_6: Some(String::new()),
            fixed_ingredient_7: Some(String::new()),
            fixed_ingredient_8: Some(String::new()),
            fixed_ingredient_stack_count_1: Some(String::new()),
            fixed_ingredient_stack_count_2: Some(String::new()),
            fixed_ingredient_stack_count_3: Some(String::new()),
            fixed_ingredient_stack_count_4: Some(String::new()),
            fixed_ingredient_stack_count_5: Some(String::new()),
            fixed_ingredient_stack_count_6: Some(String::new()),
            fixed_ingredient_stack_count_7: Some(String::new()),
            fixed_ingredient_stack_count_8: Some(String::new()),
            normal_item_1: Some(String::new()),
            normal_item_select_count: Some(String::new()),
            normal_item_stack_count_1: Some(String::new()),
            normal_item_success_probability: Some(String::new()),
            normal_item_total_count: Some(String::new()),
            upgrade_grocery: Some(String::new()),
            todo: Some(String::new()),
            filed_id: Some(String::new()),
            datafile_data: Some(String::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvolutionaryAlgorithm {
    pub money_cost: String,
    pub tran: String,
    // pub main_ingredient_stack_count: String,
    // pub keep_main_ingredient_weapon_gem_slot: String,
    // pub keep_main_ingredient_weapon_appearance: String,
    // pub keep_main_ingredient_spirit: String,
    pub consume_sub_ingredient: String,
    pub consume_fixed_ingredient: String,
    // pub category: String,
    pub use_random: String,
    pub random_failure_mileage_save: String,
    pub todo: String,
    pub sub_ingredient_1: String,
    pub sub_ingredient_1_id: String,
    // pub sub_ingredient_condition_type_1: String,
    // pub sub_ingredient_min_level_1: String,
    pub sub_ingredient_stack_count_1: String,
    pub fixed_ingredient_1: String,
    pub fixed_ingredient_1_id: String,
    pub fixed_ingredient_2: String,
    pub fixed_ingredient_2_id: String,
    pub fixed_ingredient_3: String,
    pub fixed_ingredient_3_id: String,
    pub fixed_ingredient_4: String,
    pub fixed_ingredient_4_id: String,
    pub fixed_ingredient_5: String,
    pub fixed_ingredient_5_id: String,
    pub fixed_ingredient_6: String,
    pub fixed_ingredient_6_id: String,
    pub fixed_ingredient_7: String,
    pub fixed_ingredient_7_id: String,
    pub fixed_ingredient_8: String,
    pub fixed_ingredient_8_id: String,
    pub fixed_ingredient_stack_count_1: String,
    pub fixed_ingredient_stack_count_2: String,
    pub fixed_ingredient_stack_count_3: String,
    pub fixed_ingredient_stack_count_4: String,
    pub fixed_ingredient_stack_count_5: String,
    pub fixed_ingredient_stack_count_6: String,
    pub fixed_ingredient_stack_count_7: String,
    pub fixed_ingredient_stack_count_8: String,
    pub fixed_ingredient_1_tran: String,
    pub fixed_ingredient_2_tran: String,
    pub fixed_ingredient_3_tran: String,
    pub fixed_ingredient_4_tran: String,
    pub fixed_ingredient_5_tran: String,
    pub fixed_ingredient_6_tran: String,
    pub fixed_ingredient_7_tran: String,
    pub fixed_ingredient_8_tran: String,
    // pub random_item_1: String,                    //随机进化物品
    pub random_item_select_prop_weight_1: String, //随机进化物品权重
    pub random_item_stack_count_1: String,        //随机进化物品数量
    pub random_item_success_probability: String,  //随机进化成功概率
                                                  // pub random_item_total_count: String,          //随机进化物品数量
}

impl Default for EvolutionaryAlgorithm {
    fn default() -> Self {
        Self {
            money_cost: "50000".to_string(),
            tran: String::new(),
            // main_ingredient_stack_count: "1".to_string(),
            // keep_main_ingredient_weapon_gem_slot: "y".to_string(),
            // keep_main_ingredient_weapon_appearance: "y".to_string(),
            // keep_main_ingredient_spirit: "0".to_string(),
            consume_sub_ingredient: "y".to_string(),
            consume_fixed_ingredient: "y".to_string(),
            // category: String::new(),
            use_random: "y".to_string(),
            random_failure_mileage_save: "n".to_string(),
            todo: String::new(),
            sub_ingredient_1: String::new(),
            sub_ingredient_1_id: String::new(),
            // sub_ingredient_condition_type_1: "all".to_string(),
            // sub_ingredient_min_level_1: String::new(),
            sub_ingredient_stack_count_1: String::new(),
            fixed_ingredient_1: String::new(),
            fixed_ingredient_1_id: String::new(),
            fixed_ingredient_2: String::new(),
            fixed_ingredient_2_id: String::new(),
            fixed_ingredient_3: String::new(),
            fixed_ingredient_3_id: String::new(),
            fixed_ingredient_4: String::new(),
            fixed_ingredient_4_id: String::new(),
            fixed_ingredient_5: String::new(),
            fixed_ingredient_5_id: String::new(),
            fixed_ingredient_6: String::new(),
            fixed_ingredient_6_id: String::new(),
            fixed_ingredient_7: String::new(),
            fixed_ingredient_7_id: String::new(),
            fixed_ingredient_8: String::new(),
            fixed_ingredient_8_id: String::new(),
            fixed_ingredient_stack_count_1: String::new(),
            fixed_ingredient_stack_count_2: String::new(),
            fixed_ingredient_stack_count_3: String::new(),
            fixed_ingredient_stack_count_4: String::new(),
            fixed_ingredient_stack_count_5: String::new(),
            fixed_ingredient_stack_count_6: String::new(),
            fixed_ingredient_stack_count_7: String::new(),
            fixed_ingredient_stack_count_8: String::new(),
            fixed_ingredient_1_tran: String::new(),
            fixed_ingredient_2_tran: String::new(),
            fixed_ingredient_3_tran: String::new(),
            fixed_ingredient_4_tran: String::new(),
            fixed_ingredient_5_tran: String::new(),
            fixed_ingredient_6_tran: String::new(),
            fixed_ingredient_7_tran: String::new(),
            fixed_ingredient_8_tran: String::new(),
            // random_item_1: String::new(),
            random_item_select_prop_weight_1: "1000".to_string(),
            random_item_stack_count_1: "1".to_string(),
            random_item_success_probability: "100".to_string(),
            // random_item_total_count: "1".to_string(),
        }
    }
}

pub fn load_jewelry_server(radio: Algorithm) -> Result<UpgradeJewelryList, String> {
    if is_path() == false {
        return Err("Error! 未找到当前目录！".to_string());
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
    }

    let file = File::open(jewelry_server_path).unwrap();
    let mut file = BufReader::new(file);
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut data: UpgradeJewelryList = from_str(&buffer).unwrap();

    data.count = Some(data.upgrade_jewelry_list.len());

    Ok(data)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "list")]
struct List {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@size")]
    size: String,
    #[serde(rename = "@unk1")]
    unk1: String,
    #[serde(rename = "@unk2")]
    unk2: String,
    #[serde(rename = "@unk3")]
    unk3: String,
    #[serde(rename = "collection")]
    collection: Collection,
}

#[derive(Debug, Deserialize, Serialize)]
struct Collection {
    #[serde(rename = "@compressed")]
    compressed: String,
    #[serde(rename = "archive")]
    archive: Archive,
}

#[derive(Debug, Deserialize, Serialize)]
struct Archive {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "SubArchives")]
    sub_archives: SubArchives,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubArchives {
    #[serde(rename = "BXML_SUBARCHIVE")]
    bxml_subarchive: Vec<BxmlSubarchive>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BxmlSubarchive {
    #[serde(rename = "@fieldLookupCount")]
    field_lookup_count: String,
    #[serde(rename = "fields")]
    fields: Fields,
    #[serde(rename = "lookup")]
    lookup: Lookup,
}

#[derive(Debug, Deserialize, Serialize)]
struct Fields {
    #[serde(rename = "field")]
    field: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Field {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@size")]
    size: String,
    #[serde(rename = "@unk1")]
    unk1: String,
    #[serde(rename = "@unk2")]
    unk2: String,
    #[serde(rename = "data")]
    data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Lookup {
    #[serde(rename = "BXML_LOOKUPTABLE")]
    bxml_lookuptable: Vec<BxmlLookuptable>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BxmlLookuptable {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "@empty_count")]
    empty_count: String,
    #[serde(rename = "@reall_count")]
    reall_count: String,
    #[serde(rename = "words")]
    words: Words,
}

#[derive(Debug, Deserialize, Serialize)]
struct Words {
    #[serde(rename = "string")]
    string: String,
}

pub fn load_jewelry_client(up_list_data: &mut UpgradeJewelryList) -> Result<(), String> {
    if is_path() == false {
        return Err("Error! 未找到当前目录！".to_string());
    }

    let jewelry_server_path = get_config("directory", "client") + "/datafile_177.xml";

    if is_file(jewelry_server_path.as_str()) == false {
        return Err("Error! 未找到当前文件！".to_string());
    }

    let file = File::open(jewelry_server_path).unwrap();
    let mut file = BufReader::new(file);
    let mut buf = [0; 3];
    file.read_exact(&mut buf).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let data: List = from_str(&buffer).unwrap();

    let bxml_subarchive = data.collection.archive.sub_archives.bxml_subarchive;

    // 循环所有bxml_subarchive 合并一个迭代器 进行遍历
    for i in bxml_subarchive {
        let iter = i.fields.field.iter().zip(i.lookup.bxml_lookuptable.iter());
        for (f, l) in iter {
            if let Some(r) = up_list_data
                .upgrade_jewelry_list
                .iter_mut()
                .find(|d| d.alias == l.words.string)
            {
                r.alias = l.words.string.clone();
                r.filed_id = Some(f.id.clone());
                r.datafile_data = Some(f.data.value.clone());
            }
            // else {
            //     // info!("Field not found: {:?}", l.words.string);
            // }
        }
    }
    Ok(())
}

pub fn is_in_update_item_list(update_item_list: &UpgradeJewelryList, alias: String) -> bool {
    for i in &update_item_list.upgrade_jewelry_list {
        if i.alias == alias {
            return true;
        }
    }
    false
}

//---------------------------------------------------------------
