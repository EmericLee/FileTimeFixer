use nom_exif::{MediaParser, MediaSource, ExifTag, ExifIter};
use std::path::Path;

#[tauri::command]
pub async fn get_image_datetime(file_path: String) -> Result<Option<String>, String> {
    // 检查文件是否存在
    if !Path::new(&file_path).exists() {
        return Err(format!("文件不存在: {:?}", file_path));
    }

    // 使用 MediaParser 解析 EXIF 数据
    let mut parser = MediaParser::new();
    
    // 创建媒体源
    let media_source = match MediaSource::file_path(&file_path) {
        Ok(source) => source,
        Err(e) => return Err(format!("无法创建媒体源: {}", e)),
    };

    // 检查是否有EXIF数据
    if !media_source.has_exif() {
        return Ok(None);
    }

    // 解析 EXIF 数据
    let exif_iter: ExifIter = match parser.parse(media_source) {
        Ok(data) => data,
        Err(e) => return Err(format!("解析EXIF数据失败: {}", e)),
    };

    // 按优先级查找拍摄时间
    for entry in exif_iter {
        if let Some(tag) = entry.tag() {
            match tag {
                ExifTag::DateTimeOriginal => {
                    if entry.has_value() {
                        return Ok(Some(format!("{:?}", entry)));
                    }
                }
                ExifTag::CreateDate => {
                    if entry.has_value() {
                        return Ok(Some(format!("{:?}", entry)));
                    }
                }
                ExifTag::ModifyDate => {
                    if entry.has_value() {
                        return Ok(Some(format!("{:?}", entry)));
                    }
                }
                _ => continue,
            }
        }
    }

    Ok(None) // 未找到拍摄时间
}

//     Ok(None) // 未找到拍摄时间
// }
