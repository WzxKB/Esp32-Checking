use serde_json::json;
use std::collections::HashMap;
use tauri::Emitter;

// 定义一个 trait，用于统一闭包函数的接口
trait Operation {
    fn execute(&self, a: &str, app: tauri::AppHandle) -> Result<(), String>;
}

// 实现闭包函数的 trait
impl<F> Operation for F
where
    F: Fn(&str, tauri::AppHandle) -> Result<(), String>,
{
    fn execute(&self, a: &str, app: tauri::AppHandle) -> Result<(), String> {
        self(a, app)
    }
}

// 改进后的 extract_content 函数，可以处理多个匹配项
fn extract_contents<'a>(s: &'a str, prefix: &'a str, suffix: &'a str) -> Vec<&'a str> {
    let mut contents = Vec::new();
    let mut start = 0;

    while let Some(prefix_start) = s[start..].find(prefix) {
        let prefix_start = start + prefix_start;
        let content_start = prefix_start + prefix.len();

        if let Some(suffix_start) = s[content_start..].find(suffix) {
            let suffix_start = content_start + suffix_start;
            contents.push(&s[content_start..suffix_start]);
            start = suffix_start + suffix.len();
        } else {
            break;
        }
    }

    contents
}

pub fn parse(data: String, app: tauri::AppHandle) -> Result<(), String> {
    println!("parse data:{}", data);
    let contents = extract_contents(&data, "<<<", ">>>");
    if contents.is_empty() {
        return Err("No content found".to_string());
    }

    for content in contents {
        parse_content(content, app.clone())?;
    }

    Ok(())
}

pub fn send_vue(message: String, app: tauri::AppHandle, event: &str) -> Result<(), String> {
    println!("send_vue:{}", message);
    app.emit(event, message).unwrap();

    Ok(())
}

// 解析设备信息
// 形如  {"type": "board_info","board_id": "1","board_name":"智慧养老主机V40","mac":"A001EC0824879889"}
fn gateway_info_parse_closer(s: Option<&str>, app: tauri::AppHandle) -> Result<(), String> {
    match s {
        Some(s) => {
            println!("gateway_info_parse_closer:{}", s);
            let cleaned_json_str = String::from(s).replace(r#"\""#, r#"""#);

            let json: serde_json::Value =
                serde_json::from_str(&cleaned_json_str).map_err(|e| e.to_string())?;
            let john: serde_json::Value = json!({
                "type": "board_info",
                "board_id": json["board_id"],
                "board_name": json["board_name"],
                "mac":json["mac"],

            });
            send_vue(john.to_string(), app, "board_info_event")?;
            Ok(())
        }
        None => Err("No content found".to_string()),
    }
}

// 解析设备检验信息 board_id 1代表主机 2代表组网式传感器 3代表独立式传感器
/// JOSN数据形如 {"board_id":1,"message":"blecard_init_check","content":true}
fn inspect_parse_closer(s: Option<&str>, app: tauri::AppHandle) -> Result<(), String> {
    match s {
        Some(s) => {
            println!("inspector_parse_closer:{}", s);
            let cleaned_json_str = String::from(s).replace(r#"\""#, r#"""#);

            let json: serde_json::Value =
                serde_json::from_str(&cleaned_json_str).map_err(|e| e.to_string())?;
            let john: serde_json::Value = json!({
                "type": "inspect",
                "board_id": json["board_id"],
                "message":json["message"],
                "content":json["content"]
            });
            send_vue(john.to_string(), app, "inspect_tips_event")?;
            Ok(())
        }
        None => Err("No content found".to_string()),
    }
}

fn parse_content(s: &str, app: tauri::AppHandle) -> Result<(), String> {
    // 创建 HashMap 映射闭包
    let mut operations: HashMap<&str, Box<dyn Operation>> = HashMap::new();
    operations.insert(
        "board_info",
        Box::new(|s: &str, app: tauri::AppHandle| -> Result<(), String> {
            gateway_info_parse_closer(Some(s), app)
        }),
    );
    operations.insert(
        "inspect",
        Box::new(|s: &str, app: tauri::AppHandle| -> Result<(), String> {
            inspect_parse_closer(Some(s), app)
        }),
    );

    // 提取所有 &1 和 *1 之间的内容
    let contents = extract_contents(s, "&1", "*1");
    if contents.is_empty() {
        return Err("No content found".to_string());
    }

    // 遍历每个 &1 和 *1 之间的内容
    for content in contents {
        // 提取操作类型
        let operation_type = content.trim(); // 去除可能的空白字符
        match operations.get(operation_type) {
            Some(op) => {
                // 提取 &2 和 *2 之间的内容
                let content2 = extract_contents(s, "&2", "*2");
                if content2.is_empty() {
                    return Err("No content2 found".to_string());
                }

                // 遍历每个 &2 和 *2 之间的内容
                for data in content2 {
                    op.execute(data, app.clone())?;
                }
            }
            None => {
                return Err(format!("Operation type '{}' not found", operation_type));
            }
        }
    }

    Ok(())
}
