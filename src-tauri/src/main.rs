#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::path::Path;
use tauri::api::dialog::blocking::FileDialogBuilder;

mod image_fixer;
use image_fixer::get_image_timestamp;

#[derive(serde::Serialize)]
struct FileInfo {
    name: String,
    path: String,
    size: u64,
    modified: u64,
    is_directory: bool,
}

#[tauri::command]
fn list_files(directory: String) -> Result<Vec<FileInfo>, String> {
    let path = Path::new(&directory);

    if !path.exists() {
        return Err("目录不存在".to_string());
    }

    if !path.is_dir() {
        return Err("路径不是目录".to_string());
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("无法读取目录: {}", e)),
    };

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path().display().to_string();

        let metadata = entry
            .metadata()
            .map_err(|e| format!("获取文件元数据失败: {}", e))?;
        

        let datetime = match get_image_timestamp(&path) {
            Ok(ts) => {
                eprintln!("+ : 路径: {} 时间戳: {:?}", path, ts);
                // datetime.unwrap_or(0)
                Ok(ts)
            }
            Err(e) => {
                eprintln!("x: 路径: {} 错误: {}", path, e);
                Err(e)
            }
        };


        let file_info = FileInfo {
            name: entry.file_name().display().to_string(),
            path: path,
            size: if metadata.is_dir() { 0 } else { metadata.len() },
            modified: datetime.unwrap_or(0),
            is_directory: metadata.is_dir(),
        };

        files.push(file_info);
    }

    // 按文件名排序
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(files)
}

#[tauri::command]
fn select_directory() -> Result<String, String> {
    let dialog = FileDialogBuilder::new();

    match dialog.pick_folder() {
        Some(path) => Ok(path.to_string_lossy().into_owned()),
        None => Err("用户取消选择".to_string()),
    }
}

fn main() {
    // 重定向标准错误到控制台
    #[cfg(debug_assertions)]
    {
        use std::io::Write;

        // 在调试模式下，确保标准错误输出到控制台
        let stderr = std::io::stderr();
        let mut handle = stderr.lock();

        // 写入一条测试消息来验证错误输出
        let _ = writeln!(handle, "File Time Fixer 应用程序启动...");
        let _ = writeln!(handle, "Rust 标准错误已重定向到控制台");
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_files, select_directory])
        .run(tauri::generate_context!())
        .expect("运行Tauri应用程序时出错");
}
