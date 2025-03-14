use crate::model::xlsx::InspectorSheet;
use rusqlite::{named_params, params, Connection, ToSql};
use serde_json::Value;

#[derive(Debug)]
#[allow(unused)]
struct BoardInfo {
    id: u32,
    mac: String,
    board_id: u32,
    board_name: String,
    tag: String,
    remark: String,
    blecard_init_check: Option<bool>,
    blecard_recv_check: Option<bool>,
    nvs_check: Option<bool>,
    onboard_sensor_check: Option<bool>,
    online_check: Option<bool>,
    send_data_check: Option<bool>,
    ble_mode_check: Option<bool>,
    config_to_gateway: Option<bool>,
    recv_sensor_check: Option<bool>,
    time: String,
    result: String,
}

#[derive(Debug, serde::Serialize)]
struct BoardRecord {
    id: i32,
    mac: String,
    board_name: String,
    tag: String,
    remark: String,
    time: String,
    result: String,
}

impl BoardInfo {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: u32,
        mac: String,
        board_id: u32,
        board_name: String,
        tag: String,
        remark: String,
        blecard_init_check: Option<bool>,
        blecard_recv_check: Option<bool>,
        nvs_check: Option<bool>,
        onboard_sensor_check: Option<bool>,
        online_check: Option<bool>,
        send_data_check: Option<bool>,
        ble_mode_check: Option<bool>,
        config_to_gateway: Option<bool>,
        recv_sensor_check: Option<bool>,
        time: String,
        result: String,
    ) -> Self {
        BoardInfo {
            id,
            mac,
            board_id,
            board_name,
            tag,
            remark,
            blecard_init_check,
            blecard_recv_check,
            nvs_check,
            onboard_sensor_check,
            online_check,
            send_data_check,
            ble_mode_check,
            config_to_gateway,
            recv_sensor_check,
            time,
            result,
        }
    }

    fn from_json(s: &str) -> Result<Self, String> {
        println!("from_json:{}", s);
        let cleaned_json_str = s.replace(r#"\""#, r#"""#);
        let json: Value = serde_json::from_str(&cleaned_json_str).map_err(|e| e.to_string())?;

        // Helper function to parse optional bool fields
        let parse_opt_bool =
            |field: &str| -> Option<bool> { json.get(field).and_then(Value::as_bool) };

        // Parse required fields
        let mac = json["mac"]
            .as_str()
            .ok_or("Field 'mac' is missing or not a string")?
            .to_string();

        let board_id = json["board_id"]
            .as_u64()
            .ok_or("Field 'board_id' is missing or not a number")? as u32;

        let board_name = json["board_name"]
            .as_str()
            .ok_or("Field 'board_name' is missing or not a string")?
            .to_string();

        let tag = json["tag"]
            .as_str()
            .ok_or("Field 'tag' is missing or not a string")?
            .to_string();

        let remark = json["remark"]
            .as_str()
            .ok_or("Field 'remark' is missing or not a string")?
            .to_string();
        let id = 0;
        let time = "".to_string();
        let result = "".to_string();
        // Parse optional bool fields
        let mut board = BoardInfo::new(
            id,
            mac,
            board_id,
            board_name,
            tag,
            remark,
            parse_opt_bool("blecard_init_check"),
            parse_opt_bool("blecard_recv_check"),
            parse_opt_bool("nvs_check"),
            parse_opt_bool("onboard_sensor_check"),
            parse_opt_bool("online_check"),
            parse_opt_bool("send_data_check"),
            parse_opt_bool("ble_mode_check"),
            parse_opt_bool("config_to_gateway"),
            parse_opt_bool("recv_sensor_check"),
            time,
            result,
        );
        board.result = calculate_result(&board); // 计算结果字段
        Ok(board)
    }
}

pub fn create_db() -> Result<String, String> {
    let db_path = crate::get_board_path().ok_or("全局变量丢失".to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS board (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mac TEXT NOT NULL,
            board_id INTEGER NOT NULL,
            board_name TEXT NOT NULL,
            tag TEXT,
            remark TEXT,
            blecard_init_check INTEGER,
            blecard_recv_check INTEGER,
            nvs_check INTEGER,
            onboard_sensor_check INTEGER,
            online_check INTEGER,
            send_data_check INTEGER,
            ble_mode_check INTEGER,
            config_to_gateway INTEGER,
            recv_sensor_check INTEGER,
            time TEXT,
            result TEXT
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    println!("Table 'board' is ready!");
    Ok("200".to_string())
}

pub fn insert_db(s: &str) -> Result<String, String> {
    let board = BoardInfo::from_json(s)?;
    let db_path = crate::get_board_path().ok_or("全局变量丢失".to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    println!("Inserting board: {:?}", board);
    conn.execute(
        "INSERT INTO board (
            mac, board_id, board_name, tag, remark,
            blecard_init_check, blecard_recv_check, nvs_check,
            onboard_sensor_check, online_check, send_data_check,
            ble_mode_check, config_to_gateway,recv_sensor_check, time,result
        ) VALUES (
            :mac, :board_id, :board_name, :tag, :remark,
            :blecard_init_check, :blecard_recv_check, :nvs_check,
            :onboard_sensor_check, :online_check, :send_data_check,
            :ble_mode_check, :config_to_gateway, :recv_sensor_check, datetime('now','localtime'),:result
        )",
        named_params! {
            ":mac": board.mac,
            ":board_id": board.board_id,
            ":board_name": board.board_name,
            ":tag": board.tag,
            ":remark": board.remark,
            ":blecard_init_check": board.blecard_init_check.map(|b| b as i32),
            ":blecard_recv_check": board.blecard_recv_check.map(|b| b as i32),
            ":nvs_check": board.nvs_check.map(|b| b as i32),
            ":onboard_sensor_check": board.onboard_sensor_check.map(|b| b as i32),
            ":online_check": board.online_check.map(|b| b as i32),
            ":send_data_check": board.send_data_check.map(|b| b as i32),
            ":ble_mode_check": board.ble_mode_check.map(|b| b as i32),
            ":config_to_gateway": board.config_to_gateway.map(|b| b as i32),
            ":recv_sensor_check": board.recv_sensor_check.map(|b| b as i32),
            ":result":board.result,
        },
    )
    .map_err(|e| e.to_string())?;

    Ok("200".to_string())
}

fn calculate_result(board: &BoardInfo) -> String {
    let checks = vec![
        (board.blecard_init_check, "蓝牙卡片检验"),
        (board.blecard_recv_check, "蓝牙卡片接收检验"),
        (board.nvs_check, "内存检验"),
        (board.onboard_sensor_check, "板载传感器检验"),
        (board.online_check, "入网检验"),
        (board.send_data_check, "发送消息检验"),
        (board.ble_mode_check, "蓝牙模式检验"),
        (board.recv_sensor_check, "主机配对检验"),
        (board.config_to_gateway, "接收传感器消息"),
    ];

    let mut failed_checks = Vec::new();

    for (check, name) in &checks {
        if let Some(d) = check {
            if !d {
                failed_checks.push(name.to_string());
            }
        }
    }

    if failed_checks.is_empty() {
        "检验通过".to_string()
    } else {
        let mut failed_messages = "检验不通过(".to_string();
        for item in failed_checks {
            failed_messages.push_str(&format!(" {} ", item));
        }
        failed_messages.push(')');
        failed_messages
    }
}

fn map_row_board_record(row: &rusqlite::Row) -> Result<BoardRecord, rusqlite::Error> {
    Ok(BoardRecord {
        id: row.get(0)?,
        mac: row.get(1)?,
        board_name: row.get(2)?,
        tag: row.get(3)?,
        remark: row.get(4)?,
        time: row.get(5)?,
        result: row.get(6)?,
    })
}

fn map_row_inspector_sheet(row: &rusqlite::Row) -> Result<InspectorSheet, rusqlite::Error> {
    Ok(InspectorSheet::new(
        row.get(0)?, // id
        row.get(1)?, // mac
        row.get(2)?, // board_name
        row.get(3)?, // tag
        row.get(4)?, // remark
        row.get(5)?, // time
        row.get(6)?, // result
    ))
}

pub fn query_db(s: &str) -> Result<String, String> {
    // 解析 JSON 字符串
    let json: serde_json::Value =
        serde_json::from_str(s).map_err(|e| format!("JSON 解析错误: {}", e))?;

    // 提取 mac 和 tag 字段
    let mac = json["mac"]
        .as_str()
        .ok_or("Field 'mac' is missing or not a string")?
        .to_string();
    let tag = json["tag"]
        .as_str()
        .ok_or("Field 'tag' is missing or not a string")?
        .to_string();

    // 初始化 WHERE 子句和参数
    let mut where_clause = String::new();
    let mut params = Vec::new();

    if mac != "NULL" {
        where_clause.push_str("mac = ?");
        params.push(mac);
    }

    if tag != "NULL" {
        if !where_clause.is_empty() {
            where_clause.push_str(" AND ");
        }
        where_clause.push_str("tag = ?");
        params.push(tag);
    }

    // 如果没有提供任何条件，返回错误
    if where_clause.is_empty() {
        return Err("At least one of mac or tag must be provided and not 'NULL'".to_string());
    }

    // 构建完整的 SQL 查询
    let query = format!(
        r#"
        SELECT DISTINCT
            id, mac, board_name, tag, remark, time, result
        FROM board
        WHERE {}
       ORDER BY id ASC;
        "#,
        where_clause
    );
    let db_path = crate::get_board_path().ok_or("全局变量丢失".to_string())?;
    // 打开数据库连接
    let conn = Connection::open(db_path).map_err(|e| format!("数据库连接错误: {}", e))?;

    // 准备 SQL 查询语句
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| format!("准备查询失败: {}", e))?;
    // 将参数转换为 Vec<&dyn ToSql>
    let params: Vec<&dyn ToSql> = params.iter().map(|p| p as &dyn ToSql).collect();
    // 动态传递参数，确保参数数量与占位符数量一致
    let rows = match params.len() {
        1 => stmt
            .query_map(params![params[0]], map_row_board_record)
            .map_err(|e| e.to_string())?,
        2 => stmt
            .query_map(params![params[0], params[1]], map_row_board_record)
            .map_err(|e| e.to_string())?,
        _ => return Err("Unsupported number of parameters".to_string()),
    };

    let mut results = Vec::new();

    for row in rows {
        let board = row.map_err(|e| format!("行解析错误: {}", e))?;
        results.push(board);
    }

    // 将结果转换为 JSON
    let json_result =
        serde_json::to_string(&results).map_err(|e| format!("JSON 序列化错误: {}", e))?;

    Ok(json_result)
}

pub fn query_db_with_id(id: Vec<u32>) -> Result<Vec<InspectorSheet>, String> {
    let db_path = crate::get_board_path().ok_or("全局变量丢失".to_string())?;
    let conn = Connection::open(db_path).map_err(|e| format!("数据库连接错误: {}", e))?;
    let id_str = id
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let query = format!(
        r#"
        SELECT  DISTINCT
            id, mac, board_name, tag, remark, time, result
        FROM board
        WHERE  id IN ({})
        ORDER BY id ASC;
        "#,
        id_str
    );
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| format!("准备查询失败: {}", e))?;

    let rows = stmt
        .query_map(params![], map_row_inspector_sheet)
        .map_err(|e| e.to_string())?;

    let mut vec_inspectors = Vec::new();
    for row in rows {
        let inspector = row.map_err(|e| format!("行解析错误: {}", e))?;
        vec_inspectors.push(inspector);
    }
    Ok(vec_inspectors)
}
