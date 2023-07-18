use std::{
    fs::File,
    io::{BufReader, Read},
};

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::{
    page::main_page::MyApp,
    util::tool::{get_config, is_file, is_path},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename = "table")]
pub struct GroceryTable {
    #[serde(rename = "@release-module")]
    pub release_module: String,
    #[serde(rename = "@release-side")]
    pub release_side: String,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "record")]
    pub record: Vec<GroceryRecord>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GroceryRecord {
    #[serde(rename = "@alias")]
    pub alias: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub tran: Option<String>,
}

impl Default for GroceryTable {
    fn default() -> Self {
        Self {
            release_module: "".to_string(),
            release_side: "".to_string(),
            type_: "".to_string(),
            version: "".to_string(),
            record: vec![],
        }
    }
}

pub fn load_grocery(grocery_list: &mut GroceryTable) -> Result<(), String> {
    if is_path() == false {
        return Err("Error! 未找到当前目录！".to_string());
    }

    let jewelry_server_path = get_config("directory", "server") + "/grocerydata_default.xml";

    if is_file(jewelry_server_path.as_str()) == false {
        return Err("Error! 未找到当前文件！".to_string());
    }

    let file = File::open(jewelry_server_path).unwrap();
    let mut file = BufReader::new(file);
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let data: GroceryTable = from_str(&buffer).unwrap();

    grocery_list.release_module = data.release_module;
    grocery_list.release_side = data.release_side;
    grocery_list.type_ = data.type_;
    grocery_list.version = data.version;
    grocery_list.record = data.record;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SearchGroceryDataList {
    pub count: i32,
    pub data: Vec<SearchGroceryData>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SearchGroceryData {
    pub id: String,
    pub alias: String,
    pub tran: Option<String>,
}

impl Default for SearchGroceryDataList {
    fn default() -> Self {
        Self {
            count: 0,
            data: vec![],
        }
    }
}

pub fn load_tran(me: &mut MyApp) -> Result<(), String> {
    if is_path() == false {
        return Err("Error! 未找到当前目录！".to_string());
    }

    let jewelry_server_path = get_config("directory", "tran") + "/datafile_327.xml";

    if is_file(jewelry_server_path.as_str()) == false {
        return Err("Error! 未找到当前文件！".to_string());
    }

    let mut file = File::open(jewelry_server_path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut line_num = 0;
    let mut tran_line_num = 0;
    let mut tran = String::new();

    for line in buffer.lines() {
        line_num += 1;
        if line.contains("<string>Item.Name2") {
            tran_line_num = line_num + 1;
            tran = line.to_string().replace("<string>Item.Name2.", "");
            tran = tran.replace(" ", "");
            tran = tran.replace("</string>", "");
        }
        if tran_line_num == line_num {
            let mut tran_str = line.to_string().replace("<string>", "");
            tran_str = tran_str.replace("</string>", "");
            tran_str = tran_str.replace(" ", "");
            me.tran.insert(tran.clone(), tran_str);
            tran = String::new();
            tran_line_num = 0;
        }
    }
    // info!("{:?}", me.tran);
    for mut i in &mut me.item_list.upgrade_jewelry_list {
        let title_item = i.title_item.as_ref().unwrap();
        let tran_str = me.tran.get(title_item);
        if tran_str.is_some() {
            i.tran = Some(tran_str.unwrap().to_string());
        }
    }

    for mut i in &mut me.grocery_list.record {
        let tran_str = me.tran.get(&i.alias);
        if tran_str.is_some() {
            i.tran = tran_str.cloned();
        }
    }
    Ok(())
}
