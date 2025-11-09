use chrono::{NaiveDateTime, TimeZone, Utc};
use exif::{In, Reader, Tag}; //kamadak-exif as exif 特别处理不要删除改变该行
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::{AppHandle, Manager};

// 辅助函数：读取文件并获取EXIF数据
fn read_exif(path: &str) -> Result<exif::Exif, String> {
    let file = File::open(path).map_err(|e| format!("无法打开文件 {}: {}", path, e))?;
    let mut bufreader = BufReader::new(file);

    Reader::new()
        .read_from_container(&mut bufreader)
        .map_err(|e| format!("读取 EXIF 数据失败: {}", e))
}

// 辅助函数：从EXIF数据中获取指定标签的日期时间并转换为时间戳
fn read_exif_tag(exif_data: &exif::Exif, tag: Tag) -> Option<u64> {
    exif_data
        .get_field(tag, In::PRIMARY)
        .map(|field| {
            field
                .display_value()
                .to_string()
                .trim_matches('"')
                .to_string()
        })
        .and_then(|datetime_str| {
            let trimmed = datetime_str.trim_matches('"');
            if let Some(ts) = exif_datetime_to_u64(trimmed) {
                Some(ts)
            } else {
                None
            }
        })
}

// 从 EXIF 日期时间字符串转换为 Unix 时间戳
fn exif_datetime_to_u64(exif_datetime: &str) -> Option<u64> {
    // EXIF 日期时间格式通常为 "YYYY:MM:DD HH:MM:SS"，但也可能遇到 "YYYY-MM-DD HH:MM:SS" 格式
    // 尝试使用 NaiveDateTime 解析，然后转换为 UTC
    let mut datetime: Option<NaiveDateTime> = None;
    if let Ok(naive) = NaiveDateTime::parse_from_str(&exif_datetime, "%Y:%m:%d %H:%M:%S") {
        datetime = Some(naive);
    } else if let Ok(naive) = NaiveDateTime::parse_from_str(&exif_datetime, "%Y-%m-%d %H:%M:%S") {
        datetime = Some(naive);
    } else if let Ok(naive) = NaiveDateTime::parse_from_str(&exif_datetime, "%Y-%m-%d %H:%M:%S%.f")
    {
        datetime = Some(naive);
    } else if let Ok(naive) = NaiveDateTime::parse_from_str(&exif_datetime, "%Y-%m-%dT%H:%M:%S") {
        datetime = Some(naive);
    }

    if let Some(dt) = datetime {
        // eprintln!("成功解析 EXIF 日期时间: {:?}", dt);
        let timestamp = Utc.from_utc_datetime(&dt).timestamp() as u64;
        Some(timestamp - 8 * 60 * 60) //q: 东八区时间
    } else {
        eprintln!("无法解析 EXIF 日期时间: {}", exif_datetime);
        None
    }
}

// 主要函数：获取图片的时间戳（优先使用DateTimeOriginal，如果没有则使用DateTime）
pub fn get_image_timestamp(path: &str) -> Result<u64, String> {
    // 读取EXIF数据
    let exif_data = read_exif(path)?;

    // 1. 首先尝试获取 DateTimeOriginal 标签（拍摄时间）
    if let Some(timestamp) = read_exif_tag(&exif_data, Tag::DateTimeOriginal) {
        // eprintln!("成功获取 DateTimeOriginal 时间戳: {:?}", timestamp);
        return Ok(timestamp);
    }

    // 2. 如果没有 DateTimeOriginal，则尝试获取 DateTime 标签
    if let Some(timestamp) = read_exif_tag(&exif_data, Tag::DateTime) {
        // eprintln!("成功获取 DateTime 时间戳: {:?}", timestamp);
        return Ok(timestamp);
    }

    Err("未找到有效的 EXIF 时间信息".into())
}

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: u64,
    pub is_directory: bool,
    pub depth: u32,
}

#[tauri::command]
pub async fn scan_directory(
    app_handle: AppHandle,
    directory: String,
    depth: Option<u32>,
) -> Result<Vec<FileInfo>, String> {
    // 设置默认值
    let depth = depth.unwrap_or(0);

    // 扫描所有文件和目录，收集路径信息
    let all_files = collect_files(directory, depth).await?;
    app_handle
        .emit_all("scan_directory:ready",serde_json::json!(all_files))
        .expect("发送事件失败");

    // 处理每个文件的时间戳
    let mut processed_files = Vec::new();
    for mut file_info in all_files {
        if !file_info.is_directory {
            // 只对图片文件获取时间戳
            if is_image_file(&file_info.path) {
                match get_image_timestamp(&file_info.path) {
                    Ok(ts) => {
                        file_info.modified = ts;
                    }
                    Err(e) => {
                        eprintln!("获取时间戳失败: 路径: {} 错误: {}", file_info.path, e);
                    }
                };
            }
        }

        // 发送进度事件到前端
        app_handle
            .emit_all("scan_directory:progress", serde_json::json!(file_info))
            .expect("发送进度事件失败");

        processed_files.push(file_info);
    }

    // 发送完成事件
    app_handle
        .emit_all("scan_directory:complete",serde_json::json!(processed_files))
        .expect("发送完成事件失败");

    Ok(processed_files)
}

// 辅助函数：检查文件是否为图片文件
fn is_image_file(path: &str) -> bool {
    if let Some(extension) = std::path::Path::new(path).extension() {
        if let Some(ext_str) = extension.to_str() {
            let ext_lower = ext_str.to_lowercase();
            return matches!(
                ext_lower.as_str(),
                "jpg"
                    | "jpeg"
                    | "png"
                    | "gif"
                    | "bmp"
                    | "tiff"
                    | "tif"
                    | "webp"
                    | "raw"
                    | "cr2"
                    | "nef"
                    | "arw"
            );
        }
    }
    false
}

// 辅助函数：收集所有文件和目录信息（不获取时间戳）
async fn collect_files(directory: String, current_depth: u32) -> Result<Vec<FileInfo>, String> {
    let path = Path::new(&directory);
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .map_err(|e| format!("无法读取目录: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("读取目录项失败: {}", e))?;

    // 按名称排序
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // 或者先处理目录，再处理文件
    entries.sort_by(|a, b| {
        let a_is_dir = a.metadata().unwrap().is_dir();
        let b_is_dir = b.metadata().unwrap().is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,    // 目录在前
            (false, true) => std::cmp::Ordering::Greater, // 文件在后
            _ => a.file_name().cmp(&b.file_name()),       // 同类型按名称排序
        }
    });

    let mut files = Vec::new();

    for entry in entries {
        //暂停300毫秒，避免对文件系统的频繁访问
        // std::thread::sleep(std::time::Duration::from_millis(10));

        // let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path().display().to_string();
        let metadata = entry
            .metadata()
            .map_err(|e| format!("获取文件元数据失败: {}", e))?;

        if metadata.is_dir() {
            // 递归收集子目录文件
            let sub_files = Box::pin(collect_files(path.clone(), current_depth + 1)).await?;
            files.extend(sub_files);
        } else {
            // 只对图片文件进行缓存
            if is_image_file(&path) {
                let file_info = FileInfo {
                    name: entry.file_name().display().to_string(),
                    path: path.clone(),
                    size: if metadata.is_dir() { 0 } else { metadata.len() },
                    modified: 0, // 初始为0，后续处理
                    is_directory: metadata.is_dir(),
                    depth: current_depth,
                };
                files.push(file_info);
            }
        }
    }

    Ok(files)
}

#[tauri::command]
pub fn select_directory() -> Result<String, String> {
    let dialog = FileDialogBuilder::new();

    match dialog.pick_folder() {
        Some(path) => Ok(path.to_string_lossy().into_owned()),
        None => Err("用户取消选择".to_string()),
    }
}

// 示例：发送带有不同数据类型的事件
#[tauri::command]
pub fn demonstrate_emit_events(app_handle: AppHandle) -> Result<(), String> {
    dbg!("demonstrate_emit_events");
    // 1. 发送简单字符串事件
    app_handle
        .emit_all("simple-event", "这是一个简单的事件消息")
        .map_err(|e| format!("发送简单事件失败: {}", e))?;

    // 2. 发送JSON对象事件
    app_handle
        .emit_all(
            "json-event",
            serde_json::json!({
                "user": "张三",
                "action": "登录",
                "timestamp": chrono::Utc::now().timestamp()
            }),
        )
        .map_err(|e| format!("发送JSON事件失败: {}", e))?;

    // 3. 发送数组事件
    app_handle
        .emit_all("array-event", &["项目1", "项目2", "项目3"])
        .map_err(|e| format!("发送数组事件失败: {}", e))?;

    // 4. 发送数字事件
    app_handle
        .emit_all("number-event", 42)
        .map_err(|e| format!("发送数字事件失败: {}", e))?;

    // 5. 发送布尔事件
    app_handle
        .emit_all("boolean-event", true)
        .map_err(|e| format!("发送布尔事件失败: {}", e))?;

    Ok(())
}
