use crate::model::xlsx::QRCodeSheet;
use config::ConfigError;
use config::{Config, File};
use serde::Deserialize;
use serde_json::json;
use serde_json::{self, Value as JsonValue};
use std::fs;
use toml::{self, map::Map, Value as TomlValue};
#[derive(Debug, Deserialize)]
#[allow(unused)]
struct GatewayConfig {
    blecard_init_check: bool,
    blecard_recv_check: bool,
    nvs_check: bool,
    onboard_sensor_check: bool,
    online_check: bool,
    ble_mode_check: bool,
    recv_sensor_check: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct SensorConfig {
    nvs_check: bool,
    send_data_check: bool,
    ble_mode_check: bool,
    config_to_gateway: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct IndependentSensorConfig {
    nvs_check: bool,
    online_check: bool,
    send_data_check: bool,
    ble_mode_check: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    gateway: GatewayConfig,
    sensor: SensorConfig,
    independent: IndependentSensorConfig,
    tags: Option<Vec<String>>,
    devices: Vec<QRCodeSheet>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        if let Some(path) = crate::get_config_path() {
            let s = Config::builder()
                .add_source(File::with_name(&path))
                .build()?;

            // You can deserialize (and thus freeze) the entire configuration as
            s.try_deserialize()
        } else {
            Err(ConfigError::Frozen)
        }
    }
    pub fn read(&self) -> Result<String, String> {
        let ta = match self.tags {
            Some(ref tags) => tags.clone(),
            None => vec![],
        };
        let json = json!({
            "gateway": {
                "blecard_recv_check": self.gateway.blecard_recv_check,
                "blecard_init_check": self.gateway.blecard_init_check,
                "onboard_sensor_check": self.gateway.onboard_sensor_check,
                "nvs_check": self.gateway.nvs_check,
                "online_check": self.gateway.online_check,
                "ble_mode_check":self.gateway.ble_mode_check,
                "recv_sensor_check":self.gateway.recv_sensor_check,

            },
            "sensor": {
                "nvs_check": self.sensor.nvs_check,
                "send_data_check": self.sensor.send_data_check,
                "ble_mode_check":self.sensor.ble_mode_check,
                "config_to_gateway":self.sensor.config_to_gateway,
            },
            "independent":{
                "nvs_check": self.independent.nvs_check,
                "online_check": self.independent.online_check,
                "send_data_check":self.independent.send_data_check,
                "ble_mode_check":self.independent.ble_mode_check,
            },
            "tags":ta,
        });
        Ok(json.to_string())
    }

    pub fn write(&self, config: &str) -> Result<String, String> {
        // 1. 读取现有TOML配置
        let path = crate::get_config_path().ok_or("读取全局变量错误".to_string())?;
        let old_toml_str =
            fs::read_to_string(path).map_err(|e| format!("读取TOML文件失败: {}", e))?;

        let mut old_toml = old_toml_str
            .parse::<TomlValue>()
            .map_err(|e| format!("解析TOML失败: {}", e))?;

        // 2. 解析输入的JSON配置
        let json_update: JsonValue =
            serde_json::from_str(config).map_err(|e| format!("解析JSON失败: {}", e))?;

        // 3. 将JSON转换为TOML格式的Value
        let toml_update =
            json_to_toml_value(json_update).map_err(|e| format!("JSON转换TOML失败: {}", e))?;

        // 4. 深度合并配置
        merge_toml(&mut old_toml, &toml_update);

        // 5. 写回更新后的配置
        let updated_toml =
            toml::to_string(&old_toml).map_err(|e| format!("生成TOML失败: {}", e))?;
        let path = crate::get_config_path().ok_or("读取全局变量错误".to_string())?;
        fs::write(path, updated_toml).map_err(|e| format!("写入文件失败: {}", e))?;

        Ok("200".to_owned())
    }

    pub fn read_device_template(&self, device_name: &str) -> Result<QRCodeSheet, String> {
        self.devices
            .iter()
            .find(|item| item.device_name == device_name)
            .cloned()
            .ok_or("未在模板库中找到相应设备".to_string())
    }
}

pub fn read_config() -> Result<String, String> {
    // 创建配置管理器
    let settings = Settings::new();
    // 将配置反序列化为 HashMap<String, String>
    println!("{settings:?}");

    settings
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())
}

pub fn write_config(config: &str) -> Result<String, String> {
    //读JOSN数据

    // 加载 TOML 配置文件
    let settings = Settings::new();
    // 将配置反序列化为 HashMap<String, String>
    println!("{settings:?}");

    settings
        .map_err(|e| e.to_string())?
        .write(config)
        .map_err(|e| e.to_string())
}

// JSON Value 转 TOML Value 的递归转换函数
fn json_to_toml_value(json: JsonValue) -> Result<TomlValue, String> {
    match json {
        JsonValue::Null => Err("不支持null值".into()),
        JsonValue::Bool(b) => Ok(TomlValue::Boolean(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(TomlValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(TomlValue::Float(f))
            } else {
                Err("无效的数字类型".into())
            }
        }
        JsonValue::String(s) => Ok(TomlValue::String(s)),
        JsonValue::Array(arr) => {
            let mut converted = Vec::new();
            for val in arr {
                converted.push(json_to_toml_value(val)?);
            }
            Ok(TomlValue::Array(converted))
        }
        JsonValue::Object(obj) => {
            let mut map = Map::new();
            for (k, v) in obj {
                map.insert(k, json_to_toml_value(v)?);
            }
            Ok(TomlValue::Table(map))
        }
    }
}

// 深度合并两个TOML值的递归函数
fn merge_toml(base: &mut TomlValue, update: &TomlValue) {
    match (base, update) {
        // 合并表（嵌套结构）
        (TomlValue::Table(base_map), TomlValue::Table(update_map)) => {
            for (key, val) in update_map {
                if base_map.contains_key(key) {
                    merge_toml(base_map.get_mut(key).unwrap(), val);
                } else {
                    base_map.insert(key.clone(), val.clone());
                }
            }
        }
        // 其他类型直接覆盖
        (base_val, update_val) => *base_val = update_val.clone(),
    }
}
