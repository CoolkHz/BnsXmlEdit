use std::fs::File;
use std::io::{prelude::*, BufReader};

use base64::engine::general_purpose;
use base64::Engine;
use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use serde::{Deserialize, Serialize};

use crate::page::main_page::MyApp;

use super::get_winpc_id::get_system_uuid;

pub fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn time_now_format() -> String {
    let now = Local::now();
    // 格式化毫秒
    now.format("%Y/%m/%d/ %H:%M:%S").to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub directory: Directory,
    pub pc: Pc,
    pub theme: Theme,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pc {
    pub res_code: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Theme {
    pub dark_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Directory {
    pub server: String,
    pub client: String,
    pub tran: String,
}

pub fn load_config() -> Config {
    let path = File::open("././config.toml").unwrap();
    let mut buf_reader = BufReader::new(path);
    let mut toml_string = String::new();
    buf_reader.read_to_string(&mut toml_string).unwrap();

    let config = toml::from_str::<Config>(&mut toml_string).unwrap();
    config
}

pub fn save_config(config: Config) {
    // info!("{:?}", config);
    let toml_string = toml::to_string(&config).unwrap();
    let mut file = File::create("././config.toml").unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();
}

pub fn get_config(package: &str, key: &str) -> String {
    let config = load_config();
    // info!("{:?}", config);

    match package {
        "directory" => match key {
            "server" => {
                return config.directory.server;
            }
            "client" => {
                return config.directory.client;
            }
            "tran" => {
                return config.directory.tran;
            }
            _ => "err".to_string(),
        },
        "theme" => match key {
            "dark_mode" => {
                return config.theme.dark_mode;
            }
            _ => "err".to_string(),
        },
        "pc" => {
            if key == "res_code" {
                return config.pc.res_code;
            } else {
                return "err".to_string();
            }
        }
        _ => "err".to_string(),
    }
}

pub fn set_config(package: &str, key: &str, value: &str) {
    let mut config = load_config();
    match package {
        "directory" => match key {
            "server" => {
                config.directory.server = value.to_string();
                return save_config(config);
            }
            "client" => {
                config.directory.client = value.to_string();
                return save_config(config);
            }
            "tran" => {
                config.directory.tran = value.to_string();
                return save_config(config);
            }
            _ => "err".to_string(),
        },

        "theme" => match key {
            "dark_mode" => {
                config.theme.dark_mode = value.to_string();
                return save_config(config);
            }
            _ => "err".to_string(),
        },

        "pc" => match key {
            "res_code" => {
                config.pc.res_code = value.to_string();
                return save_config(config);
            }
            _ => "err".to_string(),
        },
        _ => "err".to_string(),
    };
}

// 判断路径是否设置
pub fn is_path() -> bool {
    let config = load_config();
    if config.directory.server == "" || config.directory.client == "" {
        return false;
    }
    true
}

pub fn is_file(path: &str) -> bool {
    let path = std::path::Path::new(path);
    if path.exists() && path.is_file() {
        return true;
    }
    false
}

//判断电脑是否有资格实用软件
pub fn is_have_token() -> bool {
    //获取主板序列号
    let machine_code = get_system_uuid();
    // base64编码
    let base64_machine_code = general_purpose::STANDARD.encode(&machine_code);

    // md5 加盐
    let salt = "qiyu_toolsDW@#dd";
    let salted_str = format!("{}{}", base64_machine_code, salt);

    // MD5 加密
    let mut hasher = Md5::new();
    hasher.input_str(&salted_str);
    let result = hasher.result_str();

    // info!("result: {}", result);

    let pc_res_code = get_config("pc", "res_code");
    // info!("pc_res_code: {}", pc_res_code);
    if pc_res_code == result {
        return true;
    }

    false
}

pub fn get_res_code() -> String {
    let machine_code = get_system_uuid();
    // base64编码
    let base64_machine_code = general_purpose::STANDARD.encode(&machine_code);

    // md5 加盐
    let salt = "qiyu_toolsDW@#dd";
    let salted_str = format!("{}{}", base64_machine_code, salt);

    // MD5 加密
    let mut hasher = Md5::new();
    hasher.input_str(&salted_str);
    hasher.result_str()
}

pub fn edit_log(me: &mut MyApp, log: String) {
    let text = format!(" | {}\n", log);
    let log = time_now_format().as_str().to_owned() + &text;
    return me.edit_log.insert_str(0, &log);
}
