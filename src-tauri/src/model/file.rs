use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn write_file(file_path: String, content: &str) -> std::io::Result<()> {
    if let Some(dir_path) = crate::get_config_dir() {
        let dir_path = PathBuf::from(dir_path);
        let file_path = dir_path.join(file_path);
        match fs::create_dir_all(dir_path) {
            Ok(_) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&file_path)?;
                writeln!(file, "{}", content)?;
            }
            Err(_) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(false)
                    .open(&file_path)?;
                writeln!(file, "{}", content)?;
            }
        }

        // 写入数据

        Ok(())
    } else {
        Ok(())
    }
}
