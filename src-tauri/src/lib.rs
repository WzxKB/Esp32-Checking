use std::sync::Mutex;
use tauri::Manager;
mod model;
use chrono::prelude::*;
use tauri::path::BaseDirectory;
#[tauri::command]
fn get_time() -> String {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt);
    now.to_string()
}

#[tauri::command]
fn _get_serial_port() -> String {
    crate::model::serial::get_serial_port()
}

#[tauri::command]
async fn _start_inspect(
    port: &str,
    baud: u32,
    tag: &str,
    app: tauri::AppHandle,
) -> Result<String, String> {
    crate::model::serial::start_inspect(port, baud, tag, app)
}

#[tauri::command]
fn _stop_inspect(app: tauri::AppHandle) -> String {
    crate::model::serial::stop_inspect(app)
}

#[tauri::command]
fn _read_config() -> Result<String, String> {
    crate::model::config::read_config()
}
#[tauri::command]
fn _write_config(config: &str) -> Result<String, String> {
    crate::model::config::write_config(config)
}

#[tauri::command]
fn _create_db() -> Result<String, String> {
    crate::model::sql::create_db()
}

#[tauri::command]
fn _insert_db(s: &str) -> Result<String, String> {
    crate::model::sql::insert_db(s)
}

#[tauri::command]
fn _query_db(s: &str) -> Result<String, String> {
    crate::model::sql::query_db(s)
}

#[tauri::command]
fn _load(s: &str) -> Result<String, String> {
    crate::model::load::load(s)
}

#[tauri::command]
fn _ondelete() -> Result<String, String> {
    crate::model::load::ondelete()
}

// 全局变量存储 config 目录路径
lazy_static::lazy_static! {
    static ref CONFIG_DIR: Mutex<Option<String>> = Mutex::new(None);
    static ref CONFIG_PATH: Mutex<Option<String>> = Mutex::new(None);
    static ref BOARD_PATH: Mutex<Option<String>> = Mutex::new(None);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 获取 config 目录路径
            let config_dir = app
                .path()
                .resolve("config", BaseDirectory::Resource)
                .expect("Failed to resolve config directory");

            // 将 config 目录路径存储到全局变量
            *CONFIG_DIR.lock().unwrap() = Some(config_dir.to_string_lossy().to_string());
            let config_path = app
                .path()
                .resolve("config/config.toml", BaseDirectory::Resource)
                .expect("Failed to resolve config.toml");

            // 获取 board.db 文件路径
            let board_path = app
                .path()
                .resolve("config/board.db", BaseDirectory::Resource)
                .expect("Failed to resolve board.db");

            // 将文件路径存储到全局变量
            *CONFIG_PATH.lock().unwrap() = Some(config_path.to_string_lossy().to_string());
            *BOARD_PATH.lock().unwrap() = Some(board_path.to_string_lossy().to_string());

            println!("Config path: {:?}", config_path);
            println!("Board path: {:?}", board_path);
            println!("Config directory: {:?}", config_dir);
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            _get_serial_port,
            get_time,
            _start_inspect,
            _stop_inspect,
            _read_config,
            _write_config,
            _create_db,
            _insert_db,
            _query_db,
            _load,
            _ondelete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 提供一个函数供其他程序获取 config 目录路径
pub fn get_config_dir() -> Option<String> {
    CONFIG_DIR.lock().unwrap().clone()
}

// 提供函数供其他程序获取文件路径
pub fn get_config_path() -> Option<String> {
    CONFIG_PATH.lock().unwrap().clone()
}

pub fn get_board_path() -> Option<String> {
    BOARD_PATH.lock().unwrap().clone()
}
