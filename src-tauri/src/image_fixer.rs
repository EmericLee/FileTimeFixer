use chrono::{NaiveDateTime, TimeZone, Utc};
use exif::{In, Reader, Tag}; //kamadak-exif as exif 特别处理不要删除改变该行
use std::fs::File;
use std::io::BufReader;

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
        Some(timestamp-8*60*60)  //q: 东八区时间
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
