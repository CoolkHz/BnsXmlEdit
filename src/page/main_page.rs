use eframe::epaint::ahash::{HashMap, HashMapExt};
use egui::Context;

use crate::{
    service::{
        load_item::{GroceryTable, SearchGroceryDataList},
        upgrade_jewelry::{EvolutionaryAlgorithm, UpgradeJewelryClient, UpgradeJewelryList},
    },
    util::{
        get_winpc_id::get_system_uuid,
        tool::{
            get_config, get_res_code, is_have_token, load_config, set_config, time_now_format,
            Config,
        },
    },
};

use super::{
    bottom_panel::{bt::bt_panel, center::center_panel, left::left_panel, top::top_panel},
    search_tiem::{search_grocery, search_item},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ElItem {
    pub item_id: i32,
    pub em_name: String,
    pub item_name: String,
    pub item_tf: String,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ItemList {
    pub count: i32,
    pub list: Vec<ElItem>,
}

impl Default for ScrollDemo {
    fn default() -> Self {
        Self::ScrollTo
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollDemo {
    ManyLines,
    ScrollTo,
    // LargeCanvas,
    // StickToEnd,
    // Bidirectional,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Algorithm {
    None,
    Evolutionary,
    Weapon,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(default, serde::Deserialize, serde::Serialize)
)]
pub struct MyApp {
    pub server_path: String,
    pub client_path: String,
    pub tran_path: String,
    pub num_rows: usize,
    pub scroll_to_row_slider: usize,
    pub scroll_to_row: Option<usize>,
    pub demo: ScrollDemo,
    pub tran: HashMap<String, String>,
    pub search_text: String,
    pub item_list: UpgradeJewelryList,
    pub grocery_list: GroceryTable,
    pub update_item_list: UpgradeJewelryList,
    pub evolutionary_algorithm: EvolutionaryAlgorithm,
    pub evolutionary_algorithm_client: UpgradeJewelryClient,
    pub edit_log: String,
    pub config: Config,
    pub search_status: bool,
    pub search_status_1: bool,
    pub search_item_status: bool,
    pub search_item_status_1: bool,
    pub search_grocery_data: SearchGroceryDataList,
    pub search_item_data: UpgradeJewelryList,
    pub search_grocery_text: String,
    pub add_item_type: (i32, i32),
    pub is_have_token: bool,
    pub key: String,
    pub v_status: i32,
    pub radio: Algorithm,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            server_path: "".to_owned(),
            client_path: "".to_owned(),
            tran_path: "".to_owned(),
            num_rows: 10_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
            demo: ScrollDemo::default(),
            tran: HashMap::new(),
            search_text: "".to_owned(),
            item_list: UpgradeJewelryList::default(),
            update_item_list: UpgradeJewelryList::default(),
            evolutionary_algorithm: EvolutionaryAlgorithm::default(),
            evolutionary_algorithm_client: UpgradeJewelryClient::default(),
            config: load_config(),
            edit_log: time_now_format().as_str().to_owned() + " | 欢迎使用奇遇xml修改工具",
            grocery_list: GroceryTable::default(),
            search_status: false,
            search_status_1: false,
            search_item_status: false,
            search_item_status_1: false,
            search_grocery_data: SearchGroceryDataList::default(),
            search_item_data: UpgradeJewelryList::default(),
            search_grocery_text: "".to_owned(),
            add_item_type: (0, 0),
            is_have_token: is_have_token(),
            key: "".to_owned(),
            v_status: 0,
            radio: Algorithm::None,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //初始化字体
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../../static/font/HarmonyOS_Sans_SC_Regular.ttf"
            )),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        //初始化主题
        let dark_mode = get_config("theme", "dark_mode");
        if dark_mode == "false" {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
        } else {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
        }

        Self::default()
    }

    pub fn seach_g(&mut self, ctx: &Context) {
        search_grocery(self, ctx);
    }

    pub fn seach_item(&mut self, ctx: &Context) {
        search_item(self, ctx);
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.search_status == true && self.search_status_1 == true {
            self.seach_g(ctx);
        } else {
            self.search_status_1 = false;
            self.search_status = false;
            self.search_grocery_data = SearchGroceryDataList::default();
            self.search_grocery_text = "".to_string();
        }

        if self.search_item_status == true && self.search_item_status_1 == true {
            self.seach_item(ctx);
        } else {
            self.search_item_status_1 = false;
            self.search_item_status = false;
            self.search_item_data = UpgradeJewelryList::default();
            self.search_text = "".to_string();
        }

        // 武器首饰进化布局
        egui::Area::new("panel").show(ctx, |ui| {
            top_panel(self, ctx, ui);
            left_panel(self, ctx, ui);
            // right_panel(self, ctx, ui);
            bt_panel(self, ctx, ui);
            center_panel(self, ctx, ui);
        });
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(default, serde::Deserialize, serde::Serialize)
)]
pub struct MyEguiApp {
    pub key: String,
    pub v_status: i32,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            key: "".to_owned(),
            v_status: 0,
        }
    }
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../../static/font/HarmonyOS_Sans_SC_Regular.ttf"
            )),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // ui.horizontal(|ui| {
                let machine_code = get_system_uuid();
                ui.heading("请根据以下信息联系作者获取授权码");
                ui.add(egui::TextEdit::singleline(&mut machine_code.clone()));
                // let mut v_code = "".to_string();
                ui.add(egui::TextEdit::singleline(&mut self.key).hint_text("在此填入注册码"));
                if ui.button("验证").clicked() {
                    let res_code = get_res_code();
                    if res_code == self.key {
                        set_config("pc", "res_code", &self.key);
                        self.v_status = 1;
                    } else {
                        self.v_status = 2;
                    }
                }
                match self.v_status {
                    1 => ui.heading("注册成功！请重新启动软件！"),
                    2 => ui.heading("注册码错误，请重新输入"),
                    _ => ui.heading(""),
                }
            })
        });
    }
}
