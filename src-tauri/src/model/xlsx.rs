use crate::model::config::Settings;
use chrono::{Datelike, NaiveDateTime};
use rust_xlsxwriter::{Format, Workbook, Worksheet, XlsxError};
use serde::Deserialize;
use std::error::Error;

// 通用列宽自适应函数

/// 改进的自适应列宽函数（同时考虑标题和数据）
fn auto_adjust_column_width<T>(
    worksheet: &mut Worksheet,
    columns: &[&str],
    headers: &[&str], // 新增标题参数
    data: &[T],
    get_length: impl Fn(&T, &str) -> usize,
) -> Result<(), XlsxError> {
    for (col_index, col_name) in columns.iter().enumerate() {
        // 计算标题长度
        let header_len = headers.get(col_index).map(|s| s.len()).unwrap_or(0);

        // 计算数据最大长度
        let data_max_len = data
            .iter()
            .map(|item| get_length(item, col_name))
            .max()
            .unwrap_or(0);

        // 取标题和数据中的最大值
        let max_len = std::cmp::max(header_len, data_max_len);

        // 经验公式调整（中文按1.5倍计算）
        let adjusted_width = (max_len as f64 * 1.5) + 3.0;
        worksheet.set_column_width(col_index as u16, adjusted_width)?;
    }
    Ok(())
}
//检验信息表
pub struct InspectorSheet {
    pub id: i32,
    pub mac: String,
    pub board_name: String,
    pub tag: String,
    pub remark: String,
    pub time: String,
    pub result: String,
}
//设备表
#[allow(unused)]
pub struct DeviceSheet {
    pub mac: String,
    pub typ: String,
    pub board_name: String,
    pub number: String,
    pub protocol: String,
}

//设备标识表
pub struct DeviceIdenticationSheet {
    pub board_name: String,
    pub typ: String,
    pub mac: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct QRCodeSheet {
    pub id: i32,                     // ID
    pub device_name: String,         // 设备名称
    pub identifier: String,          // 识别标志
    pub column1: String,             // 列1
    pub production_year: String,     // 生产年份
    pub production_week: String,     // 生产星期
    pub production_category: String, // 生产类别
    pub appearance: String,          // 外观
    pub production_type: String,     // 生产类型
    pub device_code: String,         // 设备识别码
    pub column2: String,             // 列2
    pub qr_code_content: String,     // 二维码内容
    pub product_model: String,       // 产品型号
    pub company_logo: String,        // 公司logo
    pub manufacturer: String,        // 生产产商
}
pub struct CreateSheet {
    pub path: String,
    pub inspector_sheet: Vec<InspectorSheet>,
    pub device_sheet: Vec<DeviceSheet>,
    pub identication_sheet: Vec<DeviceIdenticationSheet>,
    pub qr_sheet: Vec<QRCodeSheet>,
}

impl InspectorSheet {
    pub fn new(
        id: i32,
        mac: String,
        board_name: String,
        tag: String,
        remark: String,
        time: String,
        result: String,
    ) -> Self {
        InspectorSheet {
            id,
            mac,
            board_name,
            tag,
            remark,
            time,
            result,
        }
    }
}

impl DeviceSheet {
    pub fn new(
        mac: String,
        typ: String,
        board_name: String,
        number: String,
        protocol: String,
    ) -> Self {
        DeviceSheet {
            mac,
            typ,
            board_name,
            number,
            protocol,
        }
    }

    pub fn sheet_new(inspector: &Vec<InspectorSheet>) -> Result<Vec<DeviceSheet>, String> {
        let mut res = Vec::new(); // 使用 mut 关键字声明可变的 Vec
        for item in inspector {
            let tmp = DeviceSheet::new(
                item.mac.clone(),                                  // mac
                convert_name_to_typ(&item.board_name).to_string(), // typ
                item.board_name.clone(),                           // name
                item.mac.clone(),                                  // number
                "MQTT".to_string(),                                // protocol
            );
            res.push(tmp);
        }
        Ok(res)
    }
}

impl DeviceIdenticationSheet {
    pub fn new(board_name: String, typ: String, mac: String) -> Self {
        DeviceIdenticationSheet {
            board_name,
            typ,
            mac,
        }
    }

    pub fn sheet_new(
        inspector: &Vec<InspectorSheet>,
    ) -> Result<Vec<DeviceIdenticationSheet>, String> {
        let mut res = Vec::new(); // 使用 mut 关键字声明可变的 Vec
        for item in inspector {
            let tmp = DeviceIdenticationSheet::new(
                item.board_name.clone(),                           // name
                convert_name_to_typ(&item.board_name).to_string(), // typ
                item.mac.clone(),                                  // mac
            );
            res.push(tmp);
        }
        Ok(res)
    }
}
#[allow(unused, clippy::too_many_arguments)]
impl QRCodeSheet {
    pub fn new(
        id: i32,
        device_name: String,
        identifier: String,
        column1: String,
        production_year: String,
        production_week: String,
        production_category: String,
        appearance: String,
        production_type: String,
        device_code: String,
        column2: String,
        qr_code_content: String,
        product_model: String,
        company_logo: String,
        manufacturer: String,
    ) -> Self {
        QRCodeSheet {
            id,
            device_name,
            identifier,
            column1,
            production_year,
            production_week,
            production_category,
            appearance,
            production_type,
            device_code,
            column2,
            qr_code_content,
            product_model,
            company_logo,
            manufacturer,
        }
    }
    pub fn sheet_new(inspector: &Vec<InspectorSheet>) -> Result<Vec<QRCodeSheet>, String> {
        let setting = Settings::new().map_err(|e| format!("配置读取错误：{}", e))?;
        let mut res = Vec::new(); // 使用 mut 关键字声明可变的 Vec
        for item in inspector {
            let mut qr_device = setting.read_device_template(&item.board_name.clone())?;
            let (year, week) =
                calculate_week_year(&item.time.clone()).map_err(|e| "时间转换错误".to_string())?;
            qr_device.production_year = year;
            qr_device.production_week = week;
            qr_device.device_code = item.mac.clone();
            calculate_qr_content(&mut qr_device); // mac
            res.push(qr_device);
        }
        Ok(res)
    }
}

impl CreateSheet {
    pub fn new(
        path: String,
        inspector_sheet: Vec<InspectorSheet>,
        device_sheet: Vec<DeviceSheet>,
        identication_sheet: Vec<DeviceIdenticationSheet>,
        qr_sheet: Vec<QRCodeSheet>,
    ) -> Self {
        CreateSheet {
            path,
            inspector_sheet,
            device_sheet,
            identication_sheet,
            qr_sheet,
        }
    }

    pub fn create_inspector_sheet(work: &CreateSheet) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // 修正6：移除多余的引用
        let headers = [
            "id",
            "序列号",
            "设备名称",
            "标签",
            "检验结果",
            "检验时间",
            "备注",
        ];
        worksheet.write_row(0, 0, headers)?;

        for (row, item) in work.inspector_sheet.iter().enumerate() {
            let row = row as u32 + 1;
            worksheet.write(row, 0, item.id)?;
            worksheet.write(row, 1, &item.mac)?;
            worksheet.write(row, 2, &item.board_name)?;
            worksheet.write(row, 3, &item.tag)?;
            worksheet.write(row, 4, &item.result)?;
            worksheet.write(row, 5, &item.time)?;
            worksheet.write(row, 6, &item.remark)?;
        }

        workbook.save(format!("{}/InspectorSheet.xlsx", work.path))?;
        Ok(())
    }

    pub fn create_device_sheet(work: &CreateSheet) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        let headers = ["设备序列号", "设备类型", "设备名称", "设备编号", "协议类型"];
        worksheet.write_row(0, 0, headers)?;

        for (row, item) in work.device_sheet.iter().enumerate() {
            worksheet.write(row as u32 + 1, 0, &item.mac)?;
            worksheet.write(row as u32 + 1, 1, &item.typ)?;
            worksheet.write(row as u32 + 1, 2, &item.board_name)?;
            worksheet.write(row as u32 + 1, 3, &item.number)?;
            worksheet.write(row as u32 + 1, 4, &item.protocol)?;
        }

        // 设置自适应列宽
        let columns = ["A", "B", "C", "D", "E"];
        auto_adjust_column_width(
            worksheet,
            &columns,
            &headers,
            &work.device_sheet,
            |item, col| match col {
                "A" => item.mac.len(),
                "B" => item.typ.len(),
                "C" => item.board_name.len(),
                "D" => item.number.len(),
                "E" => item.protocol.len(),
                _ => 0,
            },
        )?;

        workbook.save(format!("{}/DeviceSheet.xlsx", work.path))?;
        Ok(())
    }

    pub fn create_device_identication_sheet(work: &CreateSheet) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        let headers = ["设备名称", "设备类型", "设备序列号"];
        worksheet.write_row(0, 0, headers)?;

        for (row, item) in work.identication_sheet.iter().enumerate() {
            worksheet.write(row as u32 + 1, 0, &item.board_name)?;
            worksheet.write(row as u32 + 1, 1, &item.typ)?;
            worksheet.write(row as u32 + 1, 2, &item.mac)?;
        }

        // 自适应列宽
        let columns = ["A", "B", "C"];
        auto_adjust_column_width(
            worksheet,
            &columns,
            &headers,
            &work.identication_sheet,
            |item, col| match col {
                "A" => item.board_name.len(),
                "B" => item.typ.len(),
                "C" => item.mac.len(),
                _ => 0,
            },
        )?;

        workbook.save(format!("{}/IdenticationDeviceSheet.xlsx", work.path))?;
        Ok(())
    }

    pub fn create_qr_device_sheet(work: &CreateSheet) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // 设置中文格式
        let chinese_format = Format::new().set_font_name("Microsoft YaHei");

        let headers = [
            "ID",
            "设备名称",
            "识别标志",
            "列1",
            "生产年份",
            "生产星期",
            "生产类别",
            "外观",
            "生产类型",
            "设备识别码",
            "列2",
            "二维码内容",
            "产品型号",
            "公司logo",
            "生产厂商",
        ];

        // 写入中文标题
        worksheet.write_row_with_format(0, 0, headers, &chinese_format)?;

        // 写入数据
        for (row, item) in work.qr_sheet.iter().enumerate() {
            let row = row as u32 + 1;
            worksheet.write(row, 0, item.id)?;
            worksheet.write(row, 1, &item.device_name)?;
            worksheet.write(row, 2, &item.identifier)?;
            worksheet.write(row, 3, &item.column1)?;
            worksheet.write(row, 4, &item.production_year)?;
            worksheet.write(row, 5, &item.production_week)?;
            worksheet.write(row, 6, &item.production_category)?;
            worksheet.write(row, 7, &item.appearance)?;
            worksheet.write(row, 8, &item.production_type)?;
            worksheet.write(row, 9, &item.device_code)?;
            worksheet.write(row, 10, &item.column2)?;
            worksheet.write(row, 11, &item.qr_code_content)?;
            worksheet.write(row, 12, &item.product_model)?;
            worksheet.write(row, 13, &item.company_logo)?;
            worksheet.write(row, 14, &item.manufacturer)?;
        }

        // 自适应列宽
        let columns = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        auto_adjust_column_width(
            worksheet,
            &columns,
            &headers,
            &work.qr_sheet,
            |item, col| match col {
                "A" => item.id.to_string().len(),
                "B" => item.device_name.len(),
                "C" => item.identifier.len(),
                "D" => item.column1.len(),
                "E" => item.production_year.len(),
                "F" => item.production_week.len(),
                "G" => item.production_category.len(),
                "H" => item.appearance.len(),
                "I" => item.production_type.len(),
                "J" => item.device_code.len(),
                "K" => item.column2.len(),
                "L" => item.qr_code_content.len(),
                "M" => item.product_model.len(),
                "N" => item.company_logo.len(),
                "O" => item.manufacturer.len(),
                _ => 0,
            },
        )?;

        workbook.save(format!("{}/QRCode.xlsx", work.path))?;
        Ok(())
    }
}

fn convert_name_to_typ(name: &str) -> &str {
    // 检查 name 的前两个字是否是 "独立"
    if name.starts_with("独立") {
        return "安全监护";
    }

    // 检查 name 是否是 "智慧养老智能主机V40"
    if name == "智慧养老智能主机V40" {
        return "智慧网关";
    }

    // 其他情况返回空字符串
    ""
}

fn calculate_week_year(time: &str) -> Result<(String, String), Box<dyn Error>> {
    let dt = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")?;

    let year_short = (dt.year() % 100) as u8;
    let year_str = format!("{:02X}", year_short);

    let week = dt.date().iso_week().week() as u8;
    let week_str = format!("{:02X}", week);

    Ok((year_str, week_str))
}

// 参数改为可变引用 &mut QRCodeSheet
fn calculate_qr_content(qr_sheet: &mut QRCodeSheet) {
    let mut content = String::new();
    content.push_str(&qr_sheet.identifier);
    content.push_str(&qr_sheet.column1);
    content.push_str(&qr_sheet.production_year);
    content.push_str(&qr_sheet.production_week);
    content.push_str(&qr_sheet.production_category);
    content.push_str(&qr_sheet.appearance);
    content.push_str(&qr_sheet.device_code);

    // 所有权转移：将 content 克隆后赋值给结构体字段
    qr_sheet.qr_code_content = content.clone();
}
