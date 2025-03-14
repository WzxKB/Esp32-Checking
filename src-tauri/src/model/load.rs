use super::xlsx::{DeviceIdenticationSheet, QRCodeSheet};
use crate::model::sql::query_db_with_id;
use crate::model::xlsx::{CreateSheet, DeviceSheet};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct LoadInfo {
    id: Vec<u32>,
    path: String,
}

impl LoadInfo {
    #[allow(unused)]
    fn new(id: Vec<u32>, path: String) -> Self {
        LoadInfo { id, path }
    }

    fn from_json(s: &str) -> Result<Self, String> {
        let cleaned_json_str = s.replace(r#"\""#, r#"""#);

        match serde_json::from_str::<LoadInfo>(&cleaned_json_str) {
            Ok(load_info) => Ok(load_info),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn load(s: &str) -> Result<String, String> {
    let info = LoadInfo::from_json(s)?;
    let inspector_sheets = query_db_with_id(info.id)?;
    let device_sheets = DeviceSheet::sheet_new(&inspector_sheets).map_err(|e| e.to_string())?;
    let identification_sheet =
        DeviceIdenticationSheet::sheet_new(&inspector_sheets).map_err(|e| e.to_string())?;
    let qr_code_sheet = QRCodeSheet::sheet_new(&inspector_sheets).map_err(|e| e.to_string())?;

    let work_inspector = CreateSheet::new(
        info.path,
        inspector_sheets,
        device_sheets,
        identification_sheet,
        qr_code_sheet,
    );
    CreateSheet::create_inspector_sheet(&work_inspector)
        .map_err(|e| format!("生成表格inspector出错:{}", e))?;
    CreateSheet::create_device_sheet(&work_inspector)
        .map_err(|e| format!("生成表格device出错:{}", e))?;
    CreateSheet::create_device_identication_sheet(&work_inspector)
        .map_err(|e| format!("生成表格device identication出错:{}", e))?;
    CreateSheet::create_qr_device_sheet(&work_inspector)
        .map_err(|e| format!("生成表格qr device出错:{}", e))?;
    Ok("200".to_string())
}

pub fn reset_config_to_false() -> Result<String, String> {
    // 构建重置配置的JSON字符串
    let reset_json = r#"{
        "tags": [],
        "gateway": {
            "ble_mode_check": false,
            "blecard_init_check": false,
            "blecard_recv_check": false,
            "nvs_check": false,
            "onboard_sensor_check": false,
            "online_check": false,
            "recv_sensor_check": false
        },
        "independent": {
            "ble_mode_check": false,
            "nvs_check": false,
            "online_check": false,
            "send_data_check": false
        },
        "sensor": {
            "ble_mode_check": false,
            "config_to_gateway": false,
            "nvs_check": false,
            "send_data_check": false
        }
    }"#;
    crate::model::config::write_config(reset_json)
    // 调用现有写入函数应用配置
}

pub fn delete_db() -> Result<String, String> {
    let db_path = crate::get_board_path().ok_or("全局变量丢失".to_string())?;
    fs::remove_file(db_path).map_err(|e| e.to_string())?;
    Ok("数据库文件已删除".to_string())
}

pub fn ondelete() -> Result<String, String> {
    //删除tag
    //toml文件所有置fasle
    reset_config_to_false()?;
    delete_db()?;
    Ok("200".to_string())
    //清空数据库
}
