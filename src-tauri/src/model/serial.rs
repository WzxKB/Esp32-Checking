use crate::model::file;
use serde_json::json;
use serialport5::SerialPort;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn get_serial_port() -> String {
    // 获取当前所有串口
    let esp32_ports: Vec<String> = serialport5::available_ports()
        .unwrap_or_default()
        .iter()
        .filter_map(|p| {
            if let serialport5::SerialPortType::UsbPort(info) = &p.port_type {
                // 检查是否为 ESP32 设备（根据 VID），见ESP32的USB VID/PID资料
                if info.vid == 0x303A {
                    println!("Found ESP32 device at {}", p.port_name);
                    Some(p.port_name.clone()) // 返回 ESP32 设备的串口名称
                } else {
                    None // 不是 ESP32 设备，返回 None
                }
            } else {
                None // 不是 USB 设备，返回 None
            }
        })
        .collect();

    // 将 ESP32 设备的串口名称列表转换为 JSON 字符串
    match serde_json::to_string(&esp32_ports) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to serialize ports to JSON: {}", err);
            "[]".to_string() // 如果序列化失败，返回一个空数组的 JSON 字符串
        }
    }
}

pub fn start_inspect(
    port: &str,
    baud: u32,
    tag: &str,
    app: tauri::AppHandle,
) -> Result<String, String> {
    println!("start_inspect");
    println!("port:{},baud:{},tag:{}", port, baud, tag);
    //打开串口，只在串口读取线程使用
    let mut port = match connect_serial_port(port, baud) {
        Ok(port) => {
            println!("connect_serial_port success {:?}", port);
            port
        }
        Err(e) => match e.kind() {
            serialport5::ErrorKind::NoDevice => Err("Serial port not found".to_string())?,
            serialport5::ErrorKind::InvalidInput => Err("Permission denied".to_string())?,
            serialport5::ErrorKind::Io(io_error) => match io_error {
                std::io::ErrorKind::NotFound => Err("Serial port not found".to_string())?,
                std::io::ErrorKind::PermissionDenied => Err("Permission denied".to_string())?,
                std::io::ErrorKind::TimedOut => Err("Timeout".to_string())?,
                _ => Err(format!("I/O error: {:?}", io_error))?,
            },
            _ => Err("Unknown error".to_string())?,
        },
    };
    //发送重启指令
    let output = ">>>Reboot<<<".as_bytes();
    if let Err(error) = port.write(output) {
        Err(format!("Write Failed: {}", error))?
    }
    println!("Write: Reboot");
    let (tx, rx) = mpsc::channel();
    let port = Arc::new(Mutex::new(port)); // 使用 Arc 和 Mutex 共享 port
    start_port_recv_parse(rx, tag, app);
    println!("Write: Reboot2");
    let result = crate::model::thread::start_thread(
        "serial_read",
        move || {
            let mut serial_buf: Vec<u8> = vec![0; 2048];
            let mut port = port.lock().unwrap();
            match port.read(serial_buf.as_mut_slice()) {
                Ok(n) => {
                    serial_buf.truncate(n);
                    let result = String::from_utf8(serial_buf);
                    match result {
                        Ok(s) => {
                            println!("Read (UTF-8): {}", s);
                            tx.send(s).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Not UTF-8,err:{}", e);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Read Failed: {:?}", err);
                    match err.kind() {
                        std::io::ErrorKind::TimedOut => {}
                        _ => {
                            eprintln!("Read Failed: {:?}", err)
                        }
                    }
                }
            }
        },
        100,
    );
    match result {
        Ok(_) => Ok("200".to_string()),
        Err(err) => Err(err.to_string())?,
    }
}

fn start_port_recv_parse<T>(rx: Receiver<T>, tag: &str, app: tauri::AppHandle)
where
    T: Send + 'static + Debug,
{
    println!("start_port_recv_parse");
    let tag = tag.to_string();
    let app_clone = app.clone(); // 克隆 AppHandle
    let result = crate::model::thread::start_thread(
        "serial_parse",
        move || loop {
            match rx.recv() {
                Ok(data) => {
                    println!("recv data:{:?}", data);
                    let result =
                        crate::model::parse::parse(format!("{:?}", data), app_clone.clone());
                    match result {
                        Ok(_) => {
                            println!("成功解析");
                        }
                        Err(err) => {
                            eprintln!("parse error:{}", err);
                            break;
                        }
                    }

                    if let Err(err) = file::write_file(tag.to_string(), &format!("{:?}", data)) {
                        eprintln!("write_file error:{}", err);
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("recv error:{}", err);
                    break;
                }
            }
        },
        100,
    );
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("start_port_recv_parse error:{}", err);
        }
    }
}

pub fn stop_inspect(app: tauri::AppHandle) -> String {
    print!("stop_inspect");

    let mut result = crate::model::thread::stop_thread("serial_read");
    match result {
        Ok(_) => {
            result = crate::model::thread::stop_thread("serial_parse");
            match result {
                Ok(_) => {
                    //为了清楚界面信息发送的，收到这条信息，界面清零
                    let john: serde_json::Value = json!({
                                       "type": "board_info",
                                       "board_id": "0",
                                       "board_name": "",
                                       "mac":"",

                    });
                    if crate::model::parse::send_vue(john.to_string(), app, "board_info_event")
                        .is_ok()
                    {
                        "200".to_string()
                    } else {
                        "500".to_string()
                    }
                }
                Err(err) => err,
            }
        }

        Err(err) => err,
    }
}

fn connect_serial_port(port: &str, baud: u32) -> Result<SerialPort, serialport5::Error> {
    let port = SerialPort::builder()
        .baud_rate(baud)
        .read_timeout(Some(Duration::from_millis(10)))
        .open(port)?;

    Ok(port)
}
